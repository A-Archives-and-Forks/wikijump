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

use crate::{
    handler::*,
    host::{lookup_host, SiteAndHost},
    info,
    path::get_path,
    state::ServerState,
};
use axum::{
    body::Body,
    extract::{Request, State},
    http::header::{HeaderName, HeaderValue},
    response::{IntoResponse, Redirect},
    routing::{any, get},
    Router,
};
use axum_extra::extract::Host;
use std::{convert::Infallible, sync::Arc};
use tower::util::ServiceExt;
use tower_http::{
    compression::CompressionLayer, normalize_path::NormalizePathLayer,
    set_header::SetResponseHeaderLayer, trace::TraceLayer,
};

pub fn build_router(state: ServerState) -> Router {
    let main_state = Arc::clone(&state);
    let file_state = Arc::clone(&state);
    let header_state = Arc::clone(&state);

    macro_rules! header_value {
        ($value:expr) => {
            HeaderValue::from_str(&$value).expect("Version is not a valid header value")
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
    let files_router = Router::new()
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
            any(
                |State(state): State<ServerState>,
                 Host(hostname): Host,
                 mut request: Request<Body>| async move {
                    {
                        let headers = request.headers_mut();

                        // Strip internal headers, just to be safe.
                        headers.remove("x-wikijump-site-id");
                        headers.remove("x-wikijump-site-slug");
                        headers.remove("x-wikijump-domain");

                        // Also add the domain header since that is the same before lookup_host()
                        headers.insert("x-wikijump-domain", header_value!(hostname));
                    }

                    macro_rules! forward_request {
                        ($router:expr) => {
                            match $router.oneshot(request).await {
                                Ok(response) => response,
                                Err(infallible) => match infallible {},
                            }
                        };
                    }

                    macro_rules! add_headers {
                        ($site_id:expr, $site_slug:expr) => {{
                            // Validate types
                            let _: i64 = $site_id;
                            let _: &str = &$site_slug;

                            // Add headers
                            let headers = request.headers_mut();
                            headers.insert("x-wikijump-site-id", header_value!(str!($site_id)));
                            headers.insert("x-wikijump-site-slug", header_value!($site_slug));
                        }};
                    }

                    // Determine what host and site (e.g. main vs files, what site slug and ID)
                    let host_data = match lookup_host(&state, &hostname).await {
                        Ok(host_data) => host_data,
                        Err(error) => {
                            // TODO error page response in case of an internal issue
                            todo!()
                        }
                    };

                    // Now that we have the general category of request type, we can
                    // give it to the right place to be processed.
                    match host_data {
                        // Main site route handling
                        SiteAndHost::Main { site_id, site_slug } => {
                            add_headers!(site_id, site_slug);
                            forward_request!(main_router)
                        }
                        SiteAndHost::MainCustom { site_id, site_slug } => {
                            // NOTE: The difference here is site_slug here is String not &str
                            add_headers!(site_id, site_slug);
                            forward_request!(main_router)
                        }
                        // Main site missing
                        SiteAndHost::MainMissing { site_slug } => {
                            // TODO
                            forward_request!(main_router)
                        }
                        SiteAndHost::MainCustomMissing => {
                            todo!()
                        }
                        // Default site redirect
                        // e.g. "www.wikijump.com/foo" -> "wikijump.com/foo"
                        SiteAndHost::DefaultRedirect => {
                            let destination = format!(
                                "https://{}{}",
                                state.domains.main_domain_no_dot,
                                get_path(request.uri()),
                            );
                            Redirect::permanent(&destination).into_response()
                        }
                        // Files site route handling
                        SiteAndHost::File { site_id, site_slug } => {
                            add_headers!(site_id, site_slug);
                            forward_request!(files_router)
                        }
                        SiteAndHost::FileMissing { site_slug } => {
                            // TODO
                            forward_request!(files_router)
                        }
                        // Files site by itself
                        // See the case in host.rs for an explanation
                        SiteAndHost::FileRoot => {
                            let destination =
                                format!("https://{}", state.domains.main_domain_no_dot);

                            Redirect::temporary(&destination).into_response()
                        }
                    }
                },
            ),
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
        .with_state(state)
}
