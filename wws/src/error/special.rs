/*
 * error/special.rs
 *
 * Wilson's Web Server - Serves a zoo of user-generated content
 * Copyright (C) 2019-2025 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::{
    deepwell::TextBlockType, error::FallbackError, language::parse_accept_language,
    state::ServerStateInner,
};
use axum::{
    body::Body,
    http::{
        header::{self, HeaderMap},
        status::StatusCode,
    },
    response::{IntoResponse, Response},
};
use paste::paste;

pub use crate::deepwell::SpecialErrorHtml;

#[derive(Debug, Copy, Clone)]
pub enum SpecialError<'a> {
    SiteSlug {
        site_slug: &'a str,
    },
    SiteCustom {
        host: &'a str,
    },
    PageSlug {
        site_id: i64,
        page_slug: &'a str,
    },
    PageFetch {
        site_id: i64,
        page_slug: &'a str,
    },
    FileName {
        site_id: i64,
        page_slug: &'a str,
        filename: &'a str,
    },
    FileFetch {
        site_id: i64,
        page_slug: &'a str,
        filename: &'a str,
    },
    TextBlock {
        site_id: i64,
        index: &'a str,
        block_type: TextBlockType,
        reason: TextBlockErrorReason,
    },
    FileRoot,
}

#[derive(Debug)]
pub struct SpecialErrorOutput {
    pub title: String,
    pub body: String,
    pub status: StatusCode,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextBlockErrorReason {
    /// This hosted text block does not exist.
    Missing,

    /// The URL to this hosted text block is invalid.
    Invalid,

    /// The server was unable to retrieve this hosted text block.
    Fetch,
}

impl TextBlockErrorReason {
    #[inline]
    pub fn value(self) -> &'static str {
        // These must match the values in the Fluent files.
        match self {
            TextBlockErrorReason::Missing => "missing",
            TextBlockErrorReason::Invalid => "invalid",
            TextBlockErrorReason::Fetch => "fetch",
        }
    }
}

pub async fn build_special_error_response(
    // NOTE: We need to accept the inner struct specifically here, since there are
    //       some places in state.rs itself where we need to call this function.
    state: &ServerStateInner,
    headers: &HeaderMap,
    special_error: SpecialError<'_>,
) -> Response {
    // Get a list of preferred locales from the Accept-Language header.
    let locales = parse_accept_language(headers);

    // Build the appropriate error case

    macro_rules! deepwell_fetch {
        ($method:ident => $status_code:ident $(,)?) => {
            deepwell_fetch!($method, => $status_code)
        };
        ($method:ident, $($arg:expr),* => $status_code:ident $(,)?) => {
            deepwell_fetch!($method, $($arg),* ; StatusCode::$status_code)
        };
        ($method:ident, $($arg:expr),* ; $status_code:expr $(,)?) => {{
            paste! {
                let result = state.deepwell.[<special_error_ $method>](&locales, $($arg),*).await;
            }

            match result {
                Ok(SpecialErrorHtml { title, body }) => {
                    SpecialErrorOutput { title, body, status: $status_code }
                }
                Err(error) => {
                    // XF-1001
                    error!(
                        "Unable to get special error for {}: {}",
                        stringify!($method),
                        error,
                    );
                    return FallbackError::SpecialErrorFetch.into_response();
                }
            }
        }};
    }

    let SpecialErrorOutput {
        title,
        body,
        status,
    } = match special_error {
        SpecialError::SiteSlug { site_slug } => {
            deepwell_fetch!(missing_site_slug, site_slug => NOT_FOUND)
        }
        SpecialError::SiteCustom { host } => {
            deepwell_fetch!(missing_custom_domain, host => NOT_FOUND)
        }
        SpecialError::PageSlug { site_id, page_slug } => {
            deepwell_fetch!(missing_page_slug, site_id, page_slug => NOT_FOUND)
        }
        SpecialError::PageFetch { site_id, page_slug } => {
            deepwell_fetch!(page_fetch, site_id, page_slug => INTERNAL_SERVER_ERROR)
        }
        SpecialError::FileName {
            site_id,
            page_slug,
            filename,
        } => {
            deepwell_fetch!(missing_file_name, site_id, page_slug, filename => NOT_FOUND)
        }
        SpecialError::FileFetch {
            site_id,
            page_slug,
            filename,
        } => {
            deepwell_fetch!(file_fetch, site_id, page_slug, filename => INTERNAL_SERVER_ERROR)
        }
        SpecialError::TextBlock {
            site_id,
            index,
            block_type,
            reason,
        } => {
            let status_code = match reason {
                TextBlockErrorReason::Missing => StatusCode::NOT_FOUND,
                TextBlockErrorReason::Invalid => StatusCode::BAD_REQUEST,
                TextBlockErrorReason::Fetch => StatusCode::INTERNAL_SERVER_ERROR,
            };

            deepwell_fetch!(text_block, site_id, index, block_type, reason; status_code)
        }
        SpecialError::FileRoot => {
            deepwell_fetch!(file_root => BAD_REQUEST)
        }
    };

    // SAFETY: Both string fields here come from DEEPWELL,
    //         which in turn come from Fluent translation lines.
    //         As such, they can be trusted to not contain malicious HTML.

    const HTML_START: &str = r#"<html><head><meta name="viewport" content="width=device-width, initial-scale=1.0"/><title>"#;
    const HTML_MIDDLE: &str = "</title></head><body><article>";
    const HTML_END: &str = "</article></body></html>\n";

    let html = format!("{HTML_START}{title}{HTML_MIDDLE}{body}{HTML_END}");
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(html))
        .expect("Unable to convert response data")
}
