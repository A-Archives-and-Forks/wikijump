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
    response::Html,
};
use axum_extra::extract::Host;

pub async fn redirect_to_files(
    State(state): State<ServerState>,
    Host(hostname): Host,
    uri: Uri,
) -> Html<&'static str> {
    let path = get_path(&uri);

    // xyz.wikijump.com -> xyz.wjfiles.com
    // customdomain.com -> xyz.wjfiles.com

    let uri = format!("https://{hostname}{path}");
    todo!()
}

pub async fn redirect_to_main(
    State(state): State<ServerState>,
    headers: HeaderMap,
    uri: Uri,
) -> Html<&'static str> {
    let site_slug = headers
        .get(HEADER_SITE_SLUG)
        .expect("Site slug header not set by parent rounter")
        .to_str()
        .expect("Unable to convert site slug header to string");

    let path = get_path(&uri);
    let uri = format!("https://{}{}{}", site_slug, state.domains.main_domain, path,);

    todo!()
}
