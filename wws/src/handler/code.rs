/*
 * handler/code.rs
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

use crate::state::ServerState;
use axum::{
    extract::{Path, State},
    response::Html,
};

pub async fn handle_code_block(
    State(state): State<ServerState>,
    Path((page_slug, index)): Path<(String, String)>,
) -> Html<&'static str> {
    info!(
        page_slug = page_slug,
        index = index,
        "Returning code block data",
    );

    // TODO
    let _ = state;
    todo!()
}
