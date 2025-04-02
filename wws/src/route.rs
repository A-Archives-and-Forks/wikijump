/*
 * route.rs
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

use crate::{handler::*, state::ServerState};
use axum::{
    http::header::HeaderValue,
    routing::{any, get},
    Router,
};
use tower_http::{
    compression::CompressionLayer, normalize_path::NormalizePathLayer,
    set_header::SetResponseHeaderLayer, trace::TraceLayer,
};

pub fn build_router(state: ServerState) -> Router {
    // NOTE: For all GET routes, axum automatically handles HEAD requests.
    //       The same logic is run, but the body is removed, which is very
    //       convenient for us.
    //
    //       If we can avoid an expensive operation in a HEAD, then add
    //       a "method: http::Method" parameter in the request then check
    //       that before doing the relevant operation.

    Router::new()
        // Wikidot redirects
        .route(
            "/local--files/{page_slug}/{filename}",
            any(handle_file_redirect),
        )
        .route(
            "/local--code/{page_slug}/{index}",
            any(handle_code_redirect),
        )
        .route("/local--html/{page_slug}/{id}", any(handle_html_redirect))
        // Wikijump redirects
        .route("/-/files/{page_slug}/{filename}", any(handle_file_redirect))
        // Files
        .route("/-/file/{page_slug}/{filename}", get(handle_file_fetch))
        .route("/-/file/{page_slug}/{filename}", any(handle_invalid_method))
        .route(
            "/-/download/{page_slug}/{filename}",
            get(handle_file_download),
        )
        .route(
            "/-/download/{page_slug}/{filename}",
            any(handle_invalid_method),
        )
        // Code and HTML
        .route("/-/code/{page_slug}/{index}", get(handle_code_block))
        .route("/-/code/{page_slug}/{index}", any(handle_invalid_method))
        .route("/-/html/{page_slug}/{id}", get(handle_html_block))
        .route("/-/html/{page_slug}/{id}", any(handle_invalid_method))
        // General routes
        .route("/robots.txt", get(handle_robots_txt)) // TODO
        .route("/.well-known", any(handle_well_known)) // TODO
        .route("/-/health-check", any(handle_health_check))
        .fallback(redirect_to_main)
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(
            CompressionLayer::new()
                .gzip(true)
                .deflate(true)
                .br(true)
                .zstd(true),
        )
        .layer(SetResponseHeaderLayer::overriding(
            HEADER_IS_WIKIJUMP,
            Some(HeaderValue::from_static("1")),
        ))
        .with_state(state)
}
