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

use super::HEADER_SITE_SLUG;
use crate::path::get_path;
use crate::state::ServerState;
use axum::{
    extract::State,
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

    let site_slug = get_site_slug(&headers);
    let domain = &state.domains.files_domain;
    let path = get_path(&uri);
    let destination = format!("https://{site_slug}{domain}{path}");
    Redirect::permanent(&destination)
}

pub async fn redirect_to_main(
    State(state): State<ServerState>,
    headers: HeaderMap,
    uri: Uri,
) -> Redirect {
    let site_slug = get_site_slug(&headers);
    let domain = &state.domains.main_domain;
    let path = get_path(&uri);
    let destination = format!("https://{site_slug}{domain}{path}");
    Redirect::permanent(&destination)
}

fn get_site_slug(headers: &HeaderMap) -> &str {
    headers
        .get(HEADER_SITE_SLUG)
        .expect("Site slug header not set by parent rounter")
        .to_str()
        .expect("Unable to convert site slug header to string")
}
