/*
 * handler/mod.rs
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

mod code;
mod file;
mod framerail;
mod html;
mod misc;
mod redirect;

pub use self::code::*;
pub use self::file::*;
pub use self::framerail::*;
pub use self::html::*;
pub use self::misc::*;
pub use self::redirect::*;

use crate::{
    host::{lookup_host, SiteAndHost},
    path::get_path,
    state::ServerState,
};
use axum::{
    body::Body,
    extract::Request,
    response::{Html, IntoResponse, Redirect, Response},
    Router,
};
use tower::util::ServiceExt;

#[deprecated]
pub async fn handle_hello_world() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

/// Entry route handler to first process host information.
///
/// Before we can give this request to the right place,
/// we first must determine if it's a main or files request,
/// and then what site it corresponds to. Then we can pass
/// it to the appropriate location.
pub async fn handle_host_delegation(
    state: ServerState,
    hostname: String,
    mut request: Request<Body>,
    main_router: Router,
    files_router: Router,
) -> Response {
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
            let destination = format!("https://{}", state.domains.main_domain_no_dot);
            Redirect::temporary(&destination).into_response()
        }
    }
}
