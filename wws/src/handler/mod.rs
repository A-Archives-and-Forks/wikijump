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
mod robots;
mod well_known;

pub use self::code::*;
pub use self::file::*;
pub use self::framerail::*;
pub use self::html::*;
pub use self::misc::*;
pub use self::redirect::*;
pub use self::robots::*;
pub use self::well_known::*;

use crate::{
    error::ServerErrorCode,
    host::{lookup_host, SiteAndHost},
    path::get_path,
    state::ServerState,
};
use axum::{
    body::Body,
    extract::Request,
    http::header::{HeaderMap, HeaderName},
    response::{IntoResponse, Redirect, Response},
    Router,
};
use tower::util::ServiceExt;

pub const HEADER_SITE_ID: HeaderName = HeaderName::from_static("x-wikijump-site-id");
pub const HEADER_SITE_SLUG: HeaderName = HeaderName::from_static("x-wikijump-site-slug");
pub const HEADER_DOMAIN: HeaderName = HeaderName::from_static("x-wikijump-domain");

pub const HEADER_IS_WIKIJUMP: HeaderName = HeaderName::from_static("x-wikijump");
pub const HEADER_WWS_VERSION: HeaderName = HeaderName::from_static("x-wikijump-wws-ver");
pub const HEADER_DEEPWELL_VERSION: HeaderName = HeaderName::from_static("x-wikijump-deepwell-ver");

/// Helper function to get the site ID and slug from headers.
fn get_site_info(headers: &HeaderMap) -> (i64, &str) {
    let site_id = headers
        .get(HEADER_SITE_ID)
        .expect("No site ID header in request")
        .to_str()
        .expect("Site ID header is not UTF-8")
        .parse()
        .expect("Site ID is not a valid integer");

    let site_slug = headers
        .get(HEADER_SITE_SLUG)
        .expect("No site slug header in request")
        .to_str()
        .expect("Site slug header is not UTF-8");

    (site_id, site_slug)
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
        headers.remove(HEADER_SITE_ID);
        headers.remove(HEADER_SITE_SLUG);
        headers.remove(HEADER_DOMAIN);

        // Also add the domain header since that is the same before lookup_host()
        headers.insert(HEADER_DOMAIN, header_value!(hostname));
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
            headers.insert(HEADER_SITE_ID, header_value!(str!($site_id)));
            headers.insert(HEADER_SITE_SLUG, header_value!($site_slug));
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
            ServerErrorCode::SiteNotFound { site_slug }.into_response()
        }
        SiteAndHost::MainCustomMissing => {
            ServerErrorCode::CustomDomainNotFound { domain: &hostname }.into_response()
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
            ServerErrorCode::SiteNotFound { site_slug }.into_response()
        }
        // Files site by itself
        // See the case in host.rs for an explanation
        SiteAndHost::FileRoot => {
            let destination = format!("https://{}", state.domains.main_domain_no_dot);
            Redirect::temporary(&destination).into_response()
        }
    }
}
