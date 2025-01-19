/*
 * handler/file.rs
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

use crate::{error::Result, state::ServerState};
use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
};
use axum_extra::response::Attachment;
use wikidot_normalize::normalize;

pub async fn handle_file_redirect(Path((page_slug, filename)): Path<(String, String)>) -> Redirect {
    let destination = format!("/-/file/{page_slug}/{filename}");
    Redirect::permanent(&destination)
}

pub async fn handle_file_fetch(
    State(state): State<ServerState>,
    Path((mut page_slug, filename)): Path<(String, String)>,
) -> Html<&'static str> {
    info!(
        page_slug = page_slug,
        filename = filename,
        "Returning file data",
    );

    // TODO
    todo!()
}

pub async fn handle_file_download(
    State(state): State<ServerState>,
    Path((mut page_slug, filename)): Path<(String, String)>,
) -> Html<&'static str> {
    info!(
        page_slug = page_slug,
        filename = filename,
        "Returning file download",
    );

    // TODO Attachment
    todo!()
}

async fn get_file(state: &ServerState, site_id: i64, page_slug: &mut String, filename: &str) -> Result<i8> {
    normalize(page_slug);

    let page_id = state.get_page_slug(site_id, &page_slug).await?;
    todo!()
}
