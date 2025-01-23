/*
 * handler/redirect.rs
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

use super::get_site_info;
use crate::{host::DEFAULT_SITE_SLUG, path::get_path, state::ServerState};
use axum::{
    extract::{Path, State},
    http::{header::HeaderMap, Uri},
    response::Redirect,
};
use paste::paste;

pub async fn redirect_to_files(
    State(state): State<ServerState>,
    headers: HeaderMap,
    uri: Uri,
) -> Redirect {
    // xyz.wikijump.com -> xyz.wjfiles.com
    // customdomain.com -> xyz.wjfiles.com

    let (_, site_slug) = get_site_info(&headers);
    let path = get_path(&uri);
    let domain = &state.domains.files_domain;
    let destination = format!("https://{site_slug}{domain}{path}");
    Redirect::permanent(&destination)
}

pub async fn redirect_to_main(
    State(state): State<ServerState>,
    headers: HeaderMap,
    uri: Uri,
) -> Redirect {
    let (_, site_slug) = get_site_info(&headers);
    let path = get_path(&uri);

    // Only remove www for the main site.
    // The files site should always have an explicit site slug.
    let destination = if site_slug == DEFAULT_SITE_SLUG {
        let domain = &state.domains.main_domain_no_dot;
        format!("https://{domain}{path}")
    } else {
        let domain = &state.domains.main_domain;
        format!("https://{site_slug}{domain}{path}")
    };

    Redirect::permanent(&destination)
}

/// Code generation macro to create the "page convenience redirect routes".
///
/// These are routes on the main server like `/my-page/code/1` which really
/// go to `/-/code/my-page/1` on the files server. Since they are identical
/// aside from what special route they go to, we can have a macro generate
/// them for us.
macro_rules! make_redirect_to_route {
    ($name:ident) => {
        paste! {
            pub async fn [<redirect_to_ $name _route>](
                State(state): State<ServerState>,
                Path((page_slug, extra)): Path<(String, String)>,
                headers: HeaderMap,
            ) -> Redirect {
                let (_, site_slug) = get_site_info(&headers);
                let domain = &state.domains.files_domain;
                let route = stringify!($name);
                let destination = format!("https://{site_slug}{domain}/-/{route}/{page_slug}/{extra}");
                Redirect::permanent(&destination)
            }
        }
    };
}

make_redirect_to_route!(code);
make_redirect_to_route!(html);
make_redirect_to_route!(file);
make_redirect_to_route!(download);
