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
use crate::{path::get_path, state::ServerState};
use axum::{
    extract::State,
    http::{header::HeaderMap, Uri},
    response::Redirect,
};

pub async fn redirect_to_main(
    State(state): State<ServerState>,
    headers: HeaderMap,
    uri: Uri,
) -> Redirect {
    let (_, site_slug) = get_site_info(&headers);
    let path = get_path(&uri);

    // Only remove www for the main site.
    // The files site should always have an explicit site slug.

    let destination = if site_slug == "www" {
        let domain = &state.domains.main_domain_no_dot;
        format!("https://{domain}{path}")
    } else {
        let domain = &state.domains.main_domain;
        format!("https://{site_slug}{domain}{path}")
    };

    Redirect::permanent(&destination)
}
