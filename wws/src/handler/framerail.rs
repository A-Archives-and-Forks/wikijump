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
    http::status::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn proxy_framerail(State(state): State<ServerState>, mut req: Request) -> Response {
    let path = get_path(req.uri());
    info!(path = path, "Proxying request to framerail");

    // TODO add X-Real-IP and X-Forwarded-For
    *req.uri_mut() = state.framerail.proxy_uri(path);

    state
        .client
        .request(req)
        .await
        .map_err(|error| {
            error!("Reverse proxy to framerail failed: {error}");
            StatusCode::BAD_GATEWAY
        })
        .into_response()
}
