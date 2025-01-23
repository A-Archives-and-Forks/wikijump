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

pub async fn redirect_to_file_route(
    State(state): State<ServerState>,
    Path((page_slug, filename)): Path<(String, String)>,
    headers: HeaderMap,
) -> Redirect {
    let (_, site_slug) = get_site_info(&headers);
    let domain = &state.domains.files_domain;
    let destination = format!("https://{site_slug}{domain}/-/file/{page_slug}/{filename}");
    Redirect::permanent(&destination)
}

pub async fn redirect_to_code_route(
    State(state): State<ServerState>,
    Path((page_slug, index)): Path<(String, String)>,
    headers: HeaderMap,
) -> Redirect {
    let (_, site_slug) = get_site_info(&headers);
    let domain = &state.domains.files_domain;
    let destination = format!("https://{site_slug}{domain}/-/code/{page_slug}/{index}");
    Redirect::permanent(&destination)
}

pub async fn redirect_to_html_route(
    State(state): State<ServerState>,
    Path((page_slug, id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Redirect {
    let (_, site_slug) = get_site_info(&headers);
    let domain = &state.domains.files_domain;
    let destination = format!("https://{site_slug}{domain}/-/html/{page_slug}/{id}");
    Redirect::permanent(&destination)
}
