/*
 * handler/special_error.rs
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

use super::{
    fallback_error::FallbackError, get_site_slug, parse_accept_language,
    HEADER_SPECIAL_ERROR,
};
use crate::{deepwell::SpecialError, state::ServerState};
use axum::{
    extract::{Path, State},
    http::{header::HeaderMap, status::StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::extract::Host;
use paste::paste;

pub async fn handle_special_error(
    State(state): State<ServerState>,
    Host(host): Host,
    Path(error_code): Path<String>,
    headers: HeaderMap,
) -> Response {
    info!(error_code = error_code, "Returning special error response");

    // This header can only be set internally, so let's check it before
    // returning any error information.
    if headers.get(HEADER_SPECIAL_ERROR).is_none() {
        return FallbackError::SpecialErrorDirect.into_response();
    }

    // Get a list of preferred locales from the Accept-Language header.
    let locales = parse_accept_language(&headers);

    macro_rules! get_special_error {
        ($method:ident => $status_code:ident $(,)?) => {
            get_special_error!($method, => $status_code)
        };
        ($method:ident, $($arg:expr),* => $status_code:ident $(,)?) => {{
            paste! {
                let result = state.deepwell.[<special_error_ $method>](&locales, $($arg),*).await;
            }

            match result {
                Ok(output) => (output, StatusCode::$status_code),
                Err(error) => {
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

    // Fetch HTML from appropriate DEEPWELL special error endpoint
    let (SpecialError { title, body }, status_code) = match error_code.as_str() {
        // Required headers:
        // - x-wikijump-site-slug
        "site-slug" => {
            let site_slug = get_site_slug(&headers);
            get_special_error!(missing_site_slug, site_slug => NOT_FOUND)
        }
        // No required headers
        "site-custom" => {
            get_special_error!(missing_custom_domain, &host => NOT_FOUND)
        }
        // TODO where is this used?
        "site-fetch" => {
            get_special_error!(site_fetch, &host => INTERNAL_SERVER_ERROR)
        }
        // No required headers
        "file-root" => {
            get_special_error!(file_root => BAD_REQUEST)
        }
        // Invalid
        _ => {
            error!("Invalid special error code: {error_code}");
            return FallbackError::SpecialErrorCode.into_response();
        }
    };

    todo!()
}
