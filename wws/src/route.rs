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

use crate::handler::handle_hello_world;
use axum::{
    body::Body,
    extract::{FromRequestParts, Path, Request},
    http::{request::Parts, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{any, get},
    RequestPartsExt, Router,
};
use axum_extra::extract::Host;
use tower::util::ServiceExt;

pub fn build_router() -> Router {
    // Router that serves framerail
    let main_router = Router::new().route("/_TODO", get(handle_hello_world)); // handle wjfiles routes

    // Router that serves wjfiles
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
        .route("/{*path}", get(handle_hello_world));

    let app = Router::new().route(
        "/{*path}",
        any(|Host(hostname): Host, request: Request<Body>| async move {
            match hostname.as_str() {
                "api.mydomain.com" => file_router.oneshot(request).await,
                _ => main_router.oneshot(request).await,
            }
        }),
    );
    // TODO .layer(Extension(state));

    app
}
