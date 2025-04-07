/*
 * handler/html.rs
 *
 * Wilson's Web Server - Serves a zoo of user-generated content
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

use super::{get_site_id, get_site_slug};
use crate::state::ServerState;
use axum::{
    extract::{Path, State},
    http::header::HeaderMap,
    response::Html,
};

pub async fn handle_html_block(
    State(state): State<ServerState>,
    Path((page_slug, html_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Html<&'static str> {
    // TODO
    let _ = state;
    let _ = page_slug;
    let _ = html_id;
    let _site_id = get_site_id(&headers);
    let _site_slug = get_site_slug(&headers);
    todo!()
}
