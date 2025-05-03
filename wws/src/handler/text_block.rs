/*
 * handler/text_block.rs
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
use crate::{
    deepwell::{BLOCK_TYPE_CODE, BLOCK_TYPE_HTML},
    state::ServerState,
};
use axum::{
    extract::{Path, State},
    http::header::HeaderMap,
    response::Html,
};
use std::num::NonZeroU16;

/// Formats the S3 filename for a hosted text block.
/// See `service/text_block/service.rs` for how this value is formatted.
#[inline]
fn format_filename(block_type: &'static str, page_id: i64, index: NonZeroU16) -> String {
    format!("{page_id}_{block_type}_{index}")
}

pub async fn handle_html_block(
    State(state): State<ServerState>,
    Path((page_slug, index)): Path<(String, String)>,
    headers: HeaderMap,
) -> Html<&'static str> {
    // TODO
    let _ = state;
    let _ = page_slug;
    let _ = index;
    let _site_id = get_site_id(&headers);
    let _site_slug = get_site_slug(&headers);
    todo!()
}

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
