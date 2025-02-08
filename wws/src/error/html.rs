/*
 * error/html.rs
 *
 * Wilson's Web Server - Serves a zoo of content (framerail, user files, code, etc)
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

//! Helpers for converting error states into axum responses.
//!
//! This is for cases where getting a full or proper error message
//! (complete with localization) is not feasible due to how high
//! up this error is, and so we return an error message annotated
//! with an error code instead.
//!
//! This is very basic HTML generation. If we need to do anything
//! more fancy in the future, then feel free to replace this with
//! something better.

use axum::{
    body::Body,
    http::{
        header::{self, HeaderValue},
        StatusCode,
    },
    response::Response,
};
use v_htmlescape::escape as html_escape;

const HTML_BEGIN: &str = r"<html><head><title>";
const HTML_MIDDLE: &str = "</title><body>";
const HTML_END: &str = "</body></html>";

/// Error codes represented in wws.
/// These must match the corresponding errors in deepwell (`src/service/error.rs`)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ServerErrorCode<'a> {
    SiteNotFound {
        site_slug: &'a str,
    },
    CustomDomainNotFound {
        domain: &'a str,
    },
    PageNotFound {
        site_id: i64,
        page_slug: &'a str,
    },
    FileNotFound {
        site_id: i64,
        page_id: i64,
        filename: &'a str,
    },
    SiteFetch {
        domain: &'a str,
    },
    PageFetch {
        site_id: i64,
        page_slug: &'a str,
    },
    FileFetch {
        site_id: i64,
        page_id: i64,
        filename: &'a str,
    },
    BlobFetch {
        site_id: i64,
        page_slug: &'a str,
        filename: &'a str,
    },
}

impl ServerErrorCode<'_> {
    /// Returns the error code corresponding to this error.
    ///
    /// See `src/service/error.rs` for a listing.
    ///
    /// Note that, despite the acceptable error range only being positive,
    /// the same type (`i32`) is used here as in DEEPWELL.
    pub fn error_code(self) -> i32 {
        match self {
            ServerErrorCode::SiteNotFound { .. } => 2004,
            ServerErrorCode::CustomDomainNotFound { .. } => 2013,
            ServerErrorCode::PageNotFound { .. } => 2005,
            ServerErrorCode::FileNotFound { .. } => 2009,
            ServerErrorCode::SiteFetch { .. } => 6001,
            ServerErrorCode::PageFetch { .. } => 6002,
            ServerErrorCode::FileFetch { .. } => 6003,
            ServerErrorCode::BlobFetch { .. } => 6004,
        }
    }

    /// Returns the HTTP status code for this error.
    pub fn status_code(self) -> StatusCode {
        match self {
            ServerErrorCode::SiteNotFound { .. }
            | ServerErrorCode::CustomDomainNotFound { .. }
            | ServerErrorCode::PageNotFound { .. }
            | ServerErrorCode::FileNotFound { .. } => StatusCode::NOT_FOUND,
            ServerErrorCode::SiteFetch { .. }
            | ServerErrorCode::PageFetch { .. }
            | ServerErrorCode::FileFetch { .. }
            | ServerErrorCode::BlobFetch { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Returns the HTML title for this error.
    fn title(self) -> &'static str {
        match self {
            ServerErrorCode::SiteNotFound { .. } | ServerErrorCode::CustomDomainNotFound { .. } => {
                "Site not found"
            }
            ServerErrorCode::PageNotFound { .. } => "Page not found",
            ServerErrorCode::FileNotFound { .. } => "File not found",
            ServerErrorCode::SiteFetch { .. } => "Cannot load site information",
            ServerErrorCode::PageFetch { .. } => "Cannot load page",
            ServerErrorCode::FileFetch { .. } => "Cannot load file",
            ServerErrorCode::BlobFetch { .. } => "Cannot load file data",
        }
    }

    pub fn into_response(self) -> Response {
        // Build error HTML
        let mut body = String::with_capacity(HTML_BEGIN.len() + HTML_END.len() + 70);
        body.push_str(HTML_BEGIN);
        body.push_str(self.title());
        body.push_str(HTML_MIDDLE);

        let error_code = self.error_code();
        str_write!(&mut body, "<strong>[Error #{error_code}]</strong> ");

        // Write error body
        match self {
            ServerErrorCode::SiteNotFound { site_slug } => {
                str_write!(
                    body,
                    "No site exists at \"<code>{}</code>\".",
                    html_escape(site_slug),
                )
            }
            ServerErrorCode::CustomDomainNotFound { domain } => {
                str_write!(
                    body,
                    "No site exists with the custom domain \"<code>{}</code>\".",
                    html_escape(domain),
                )
            }
            ServerErrorCode::PageNotFound { site_id, page_slug } => {
                str_write!(
                    body,
                    "Cannot find page \"<code>{}</code>\" in site ID <code>{}</code>.",
                    html_escape(page_slug),
                    site_id,
                );
            }
            ServerErrorCode::FileNotFound {
                site_id,
                page_id,
                filename,
            } => {
                str_write!(
                    body,
                    "Cannot find file \"<code>{}</code>\" in page ID <code>{}</code> in site ID <code>{}</code>",
                    html_escape(filename),
                    page_id,
                    site_id,
                );
            }
            ServerErrorCode::SiteFetch { domain } => {
                str_write!(
                    body,
                    "Cannot load site information for domain \"<code>{}</code>\".",
                    html_escape(domain),
                );
            }
            ServerErrorCode::PageFetch { site_id, page_slug } => {
                str_write!(
                    body,
                    "Cannot load page \"<code>{}</code>\" in site ID <code>{}</code>.",
                    html_escape(page_slug),
                    site_id,
                );
            }
            ServerErrorCode::FileFetch {
                site_id,
                page_id,
                filename,
            } => {
                str_write!(
                    body,
                    "Cannot load file \"<code>{}</code>\", in page ID <code>{}</code> in site ID <code>{}</code>.",
                    html_escape(filename),
                    page_id,
                    site_id,
                );
            }
            ServerErrorCode::BlobFetch {
                site_id,
                page_slug,
                filename,
            } => {
                str_write!(
                    body,
                    "Cannot load file data for \"<code>{}</code>\", in page \"<code>{}</code>\" in site ID <code>{}</code>.",
                    html_escape(filename),
                    html_escape(page_slug),
                    site_id,
                );
            }
        };

        body.push_str(HTML_END);

        // Build and return response
        Response::builder()
            .status(self.status_code())
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            )
            .body(Body::from(body))
            .expect("Unable to build response")
    }
}
