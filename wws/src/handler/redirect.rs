/*
 * handler/redirect.rs
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

use super::{get_site_id, FallbackError};
use crate::{path::get_path, state::ServerState};
use axum::{
    extract::State,
    http::{header::HeaderMap, Uri},
    response::{IntoResponse, Redirect, Response},
};

pub async fn redirect_to_main(
    State(state): State<ServerState>,
    headers: HeaderMap,
    uri: Uri,
) -> Response {
    let site_id = get_site_id(&headers);
    let path = get_path(&uri);
    redirect_to_main_inner(&state, site_id, path).await
}

pub async fn redirect_to_main_inner(
    state: &ServerState,
    site_id: i64,
    path: &str,
) -> Response {
    match state.get_site_domain(site_id).await {
        Ok(domain) => {
            let destination = format!("https://{domain}{path}");
            Redirect::permanent(&destination).into_response()
        }
        Err(error) => {
            error!("Could not fetch preferred site domain for site ID {site_id}: {error}");
            FallbackError::RedirectMain.into_response()
        }
    }
}
