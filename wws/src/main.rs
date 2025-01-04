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
