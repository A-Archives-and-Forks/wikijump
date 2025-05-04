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
    error::{
        build_special_error_response, FallbackError, SpecialError, TextBlockErrorReason,
        TextBlockType,
    },
    state::ServerState,
};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{
        header::{self, HeaderMap},
        status::StatusCode,
    },
    response::{IntoResponse, Response},
};
use std::{collections::HashMap, num::NonZeroU16};

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
) -> Response {
    let site_id = get_site_id(&headers);
    let page_id = try_response!(state.get_page_or_response(&headers, site_id, &page_slug));

    let index: NonZeroU16 = match index.parse() {
        Ok(index) => index,
        Err(_) => {
            error!(index = index, "Invalid text block index");

            return build_special_error_response(
                &state,
                &headers,
                SpecialError::TextBlock {
                    site_id,
                    index: &index,
                    block_type: TextBlockType::Html,
                    reason: TextBlockErrorReason::Invalid,
                },
            )
            .await;
        }
    };

    let s3_filename = format_filename(BLOCK_TYPE_HTML, page_id, index);
    info!("Fetching HTML text block from S3 object '{s3_filename}'");

    // Since text blocks are much smaller than files (necessarily being
    // at most as big as the biggest page's sources) then it's fine for
    // us to download the whole thing to memory instead of streaming it.
    let s3_response = match state.s3_tblocks_bucket.get_object(&s3_filename).await {
        Ok(response) => {
            assert_eq!(
                response.status_code(),
                StatusCode::OK,
                "get_object() succeeded but did not reply 200",
            );

            response
        }
        Err(error) => {
            // NOTE: If the error here is 404 we still return 500.
            //
            //       If we have a file record for a file, then the
            //       corresponding blob *should* exist.
            //
            //       If it doesn't, the data invariant is not being met,
            //       which is an unexpected error.
            //
            //       Fallback error code: XF-1004
            error!(
                page_id = page_id,
                block_type = "html",
                s3_filename = s3_filename,
                "Cannot get text block data: {error}",
            );
            return FallbackError::TextBlockS3Fetch.into_response();
        }
    };

    let mime = get_content_type(s3_response.headers());
    let body = Body::from(s3_response.to_vec());
    let result = Response::builder()
        .header(header::CONTENT_TYPE, &mime)
        .body(body);

    match result {
        Ok(response) => response,
        Err(error) => {
            error!("Unable to convert response: {error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn handle_code_block(
    State(state): State<ServerState>,
    Path((page_slug, index)): Path<(String, String)>,
) -> Response {
    info!(
        page_slug = page_slug,
        index = index,
        "Returning code block data",
    );

    // TODO
    let _ = state;
    todo!()
}

// Since this thing isn't returning a case-insensitive map...
fn get_content_type(headers: HashMap<String, String>) -> String {
    for (key, value) in headers.into_iter() {
        if key.eq_ignore_ascii_case("content-type") {
            return value;
        }
    }

    panic!("No Content-Type header returned in S3 response");
}
