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

use crate::deepwell::Domains;
use crate::handler::*;
use crate::info;
use crate::state::ServerState;
use axum::{
    body::Body,
    extract::Request,
    http::header::{HeaderName, HeaderValue},
    response::Redirect,
    routing::{any, get},
    Router,
};
use axum_extra::extract::Host;
use std::sync::Arc;
use tower::util::ServiceExt;
use tower_http::{
    compression::CompressionLayer, normalize_path::NormalizePathLayer,
    set_header::SetResponseHeaderLayer, trace::TraceLayer,
};

pub fn build_router(state: ServerState) -> Router {
    let main_state = Arc::clone(&state);
    let file_state = Arc::clone(&state);
    let host_state = Arc::clone(&state);
    let header_state = Arc::clone(&state);

    macro_rules! header_value {
        ($value:expr) => {
            HeaderValue::from_str($value).expect("Version is not a valid header value")
        };
    }

    // Router that serves framerail
    // TODO
    let main_router = Router::new()
        .route("/local--files/{*rest}", any(redirect_to_files))
        .route("/local--code/{*rest}", any(redirect_to_files))
        .route("/local--html/{*rest}", any(redirect_to_files))
        .route("/-/file/{*rest}", any(redirect_to_files))
        .route("/-/download/{*rest}", any(redirect_to_files))
        .route("/-/code/{*rest}", any(redirect_to_files))
        .route("/-/html/{*rest}", any(redirect_to_files))
        .route("/", any(proxy_framerail))
        .with_state(main_state);

    // Router that serves wjfiles
    // TODO
    let file_router = Router::new()
        .route(
            "/local--files/{page_slug}/{filename}",
            get(handle_hello_world),
        )
        .route("/local--code/{page_slug}/{index}", get(handle_hello_world))
        .route("/local--html/{page_slug}/{id}", get(handle_hello_world))
        .route("/-/file/{page_slug}/{filename}", get(handle_hello_world))
        .route(
            "/-/download/{page_slug}/{filename}",
            get(handle_hello_world),
        )
        .route("/-/code/{page_slug}/{index}", get(handle_hello_world))
        .route("/-/html/{page_slug}/{hash}", get(handle_hello_world))
        .route("/", get(handle_hello_world))
        .with_state(file_state);

    Router::new()
        // Domain delegation logic
        .route(
            "/",
            any(|Host(ref hostname): Host, request: Request<Body>| async move {
                let Domains {
                    ref main_domain,
                    ref main_domain_no_dot,
                    ref files_domain,
                    ref files_domain_no_dot,
                    ..
                } = host_state.domains;

                // First, check if it's the main domain by itself.
                if hostname = main_domain_no_dot {
                    // TODO
                    println!("DEBUG main default");
                    return main_router.oneshot(request).await;
                }

                // Determine if it's the main domain.
                if let Some(site_slug) = hostname.strip_suffix(main_domain) {
                    // TODO
                    println!("DEBUG main ({site_slug})");
                    return main_router.oneshot(request).await;
                }

                // Determine if it's a files domain.
                if let Some(site_slug) = hostname.strip_suffix(files_domain) {
                    // TODO
                    println!("DEBUG files (site {site_slug})");
                    return file_router.oneshot(request).await;
                }

                // Finally, check if it's the files domain by itself.
                //
                // This is weird, wjfiles should always a site slug subdomain,
                // so in this case we just temporary redirect to the main domain,
                // stripping the path.
                //
                // Since this is expected to be uncommon, we're putting it after
                // the site files check.
                if hostname = files_domain_no_dot {
                    // TODO
                    println!("DEBUG files default");
                    return file_router.oneshot(request).await;
                }

                // If it's anything else, it must be a custom domain.
                // Do a lookup, then set the site data as appropriate.
                println!("DEBUG main {hostname}");
                main_router.oneshot(request).await
            }),
        )
        // Easter egg
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
            HeaderName::from_static("x-wikijump"),
            Some(HeaderValue::from_static("1")),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-wikijump-wws-ver"),
            Some(header_value!(&*info::VERSION_INFO)),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-wikijump-deepwell-ver"),
            Some(header_value!(&header_state.domains.deepwell_version)),
        ))
}
