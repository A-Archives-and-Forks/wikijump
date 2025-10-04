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
    get_header, get_site_id, get_site_slug, HEADER_FILENAME, HEADER_PAGE_SLUG,
    HEADER_SPECIAL_ERROR,
};
use crate::{
    error::{build_special_error_response, FallbackError, SpecialError},
    state::ServerState,
};
use axum::{
    extract::{Path, State},
    http::header::HeaderMap,
    response::{IntoResponse, Response},
};
use axum_extra::extract::Host;

fn get_page_slug(headers: &HeaderMap) -> &str {
    get_header(
        headers,
        HEADER_PAGE_SLUG,
        "No page slug header in request",
        "Page slug header is not UTF-8",
    )
}

fn get_filename(headers: &HeaderMap) -> &str {
    get_header(
        headers,
        HEADER_FILENAME,
        "No filename header in request",
        "Filename header is not UTF-8",
    )
}

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
        // XF-1002
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
        // Required headers:
        // - x-wikijump-page-slug
        "page-slug" => {
            let site_id = get_site_id(&headers);
            let page_slug = get_page_slug(&headers);
            SpecialError::PageSlug { site_id, page_slug }
        }
        // Required headers:
        // - x-wikijump-page-slug
        "page-fetch" => {
            let site_id = get_site_id(&headers);
            let page_slug = get_page_slug(&headers);
            SpecialError::PageFetch { site_id, page_slug }
        }
        // Required headers:
        // - x-wikijump-page-slug
        // - x-wikijump-filename
        "file-name" => {
            let site_id = get_site_id(&headers);
            let page_slug = get_page_slug(&headers);
            let filename = get_filename(&headers);
            SpecialError::FileName {
                site_id,
                page_slug,
                filename,
            }
        }
        // Required headers:
        // - x-wikijump-page-slug
        // - x-wikijump-filename
        "file-fetch" => {
            let site_id = get_site_id(&headers);
            let page_slug = get_page_slug(&headers);
            let filename = get_filename(&headers);
            SpecialError::FileFetch {
                site_id,
                page_slug,
                filename,
            }
        }
        // No required headers
        "file-root" => SpecialError::FileRoot,
        // Invalid
        _ => {
            // XF-1000
            error!("Invalid special error code: {error_code}");
            return FallbackError::SpecialErrorCode.into_response();
        }
    };

    build_special_error_response(&state, &headers, input).await
}
