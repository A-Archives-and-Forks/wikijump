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

use super::{get_site_slug, HEADER_SPECIAL_ERROR};
use crate::{
    error::{build_special_error_response, FallbackError, SpecialError, SpecialErrorHtml},
    state::ServerState,
};
use axum::{
    extract::{Path, State},
    http::{header::HeaderMap, status::StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::extract::Host;

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

    // Build the appropriate SpecialError enum case
    let input = match error_code.as_str() {
        // Required headers:
        // - x-wikijump-site-slug
        "site-slug" => {
            let site_slug = get_site_slug(&headers);
            SpecialError::SiteSlug { site_slug }
        }
        // No required headers
        "site-custom" => SpecialError::SiteCustom { host: &host },
        // No required headers
        "file-root" => SpecialError::FileRoot,
        // Invalid
        _ => {
            error!("Invalid special error code: {error_code}");
            return FallbackError::SpecialErrorCode.into_response();
        }
    };

    build_special_error_response(&state, &headers, input).await
}
