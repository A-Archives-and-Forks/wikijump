/*
 * handler/framerail.rs
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

use crate::path::get_path;
use crate::state::ServerState;
use axum::{
    extract::{Request, State},
    http::{status::StatusCode, Uri},
    response::Html,
};

pub async fn proxy_framerail(
    State(state): State<ServerState>,
    mut req: Request,
) -> Html<&'static str> {
    // Get path and query
    let path = get_path(req.uri());

    // Create and set framerail URL
    let framerail_host = "framerail"; // TODO
    let framerail_port = 3000; // TODO
    let uri = format!("http://{framerail_host}:{framerail_port}{path}");
    *req.uri_mut() = Uri::try_from(uri).expect("Internal framerail URI is invalid");

    // TODO
    todo!()
}
