/*
 * route.rs
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

use crate::{handler::*, info, state::ServerState};
use axum::{
    body::Body,
    extract::{Request, State},
    http::header::{HeaderName, HeaderValue},
    routing::{any, get},
    Router,
};
use axum_extra::extract::Host;
use std::sync::Arc;
use tower_http::{
    compression::CompressionLayer, normalize_path::NormalizePathLayer,
    set_header::SetResponseHeaderLayer, trace::TraceLayer,
};

pub fn build_router(state: ServerState) -> Router {
    let main_state = Arc::clone(&state);
    let file_state = Arc::clone(&state);
    let header_state = Arc::clone(&state);

    // Router that serves framerail
    let main_router = Router::new()
        .route("/local--files/{*rest}", any(redirect_to_files))
        .route("/local--code/{*rest}", any(redirect_to_files))
        .route("/local--html/{*rest}", any(redirect_to_files))
        .route("/-/files/{*rest}", any(redirect_to_files))
        .route("/-/file/{*rest}", any(redirect_to_files))
        .route("/-/download/{*rest}", any(redirect_to_files))
        .route("/-/code/{*rest}", any(redirect_to_files))
        .route("/-/html/{*rest}", any(redirect_to_files))
        .fallback(proxy_framerail)
        .with_state(main_state);

    // Router that serves wjfiles
    //
    // NOTE: For all GET routes, axum automatically handles HEAD requests.
    //       The same logic is run, but the body is removed, which is very
    //       convenient for us.
    //
    //       If we can avoid an expensive operation in a HEAD, then add
    //       a "method: http::Method" parameter in the request then check
    //       that before doing the relevant operation.
    let files_router = Router::new()
        // Wikidot routes
        .route(
            "/local--files/{page_slug}/{filename}",
            get(handle_file_redirect),
        )
        .route(
            "/local--code/{page_slug}/{index}",
            any(handle_code_redirect),
        )
        .route("/local--html/{page_slug}/{id}", any(handle_html_redirect))
        // Other redirects
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
        .fallback(redirect_to_main)
        .with_state(file_state);

    Router::new()
        // Forward requests to the appropriate sub-router depending on the hostname
        .fallback(
            |State(state): State<ServerState>,
            Host(hostname): Host,
            request: Request<Body>| async move {
                handle_host_delegation(state, hostname, request, main_router, files_router).await
            }
        )
        // General routes
        .route("/robots.txt", get(handle_robots_txt)) // TODO
        .route("/.well-known", any(handle_well_known)) // TODO
        .route("/-/teapot", any(handle_teapot))
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
        .layer(SetResponseHeaderLayer::overriding(
            HEADER_WWS_VERSION,
            Some(header_value!(&*info::VERSION_INFO)),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HEADER_DEEPWELL_VERSION,
            Some(header_value!(&header_state.domains.deepwell_version)),
        ))
        .with_state(state)
}
