/*
 * main.rs
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

//! A server to handle incoming web requests.
//!
//! Depending on the hostname, requests are routed to either framerail
//! or given to logic to serve wjfiles data.

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Router that serves framerail
    let main_router = Router::new().route("/_TODO", get(handler)); // handle wjfiles routes

    // Router that serves wjfiles
    let file_router = Router::new()
        .route("/local--files/{page_slug}/{filename}", get(handler))
        .route("/local--code/{page_slug}/{index}", get(handler))
        .route("/local--html/{page_slug}/{id}", get(handler))
        .route("/-/file/{page_slug}/{filename}", get(handler))
        .route("/-/download/{page_slug}/{filename}", get(handler))
        .route("/-/code/{page_slug}/{index}", get(handler))
        .route("/-/html/{page_slug}/{hash}", get(handler))
        .route("/{*path}", get(handler));

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

    // run it
    let listener = tokio::net::TcpListener::bind("[::]:8080").await?;

    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
