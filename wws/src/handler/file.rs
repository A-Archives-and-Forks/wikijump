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

use super::get_site_info;
use crate::{error::ServerErrorCode, state::ServerState};
use axum::{
    body::Body,
    extract::{Path, State},
    http::header::{self, HeaderMap},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::response::Attachment;
use s3::request::request_trait::ResponseDataStream;
use wikidot_normalize::normalize;

macro_rules! fetch_file {
    ($state:expr, $headers:expr, $page_slug:expr, $filename:expr $(,)?) => {{
        normalize(&mut $page_slug);

        let (site_id, _) = get_site_info(&$headers);

        let state = &$state;
        let page_slug = &$page_slug;
        let filename = &$filename;

        let page_id = match state.get_page(site_id, page_slug).await {
            Ok(Some(page_id)) => page_id,
            Ok(None) => {
                error!(
                    site_id = site_id,
                    page_slug = page_slug,
                    "Cannot get file, no such page",
                );
                return ServerErrorCode::PageNotFound { site_id, page_slug }.into_response();
            }
            Err(error) => {
                error!(
                    site_id = site_id,
                    page_slug = page_slug,
                    "Cannot get page info: {error}",
                );
                return ServerErrorCode::PageFetch { site_id, page_slug }.into_response();
            }
        };

        let file_info = match state.get_file(site_id, page_id, filename).await {
            Ok(Some(file_info)) => file_info,
            Ok(None) => {
                error!(
                    site_id = site_id,
                    page_id = page_id,
                    filename = filename,
                    "Cannot get file, none with filename",
                );
                return ServerErrorCode::FileNotFound {
                    site_id,
                    page_id,
                    filename,
                }
                .into_response();
            }
            Err(error) => {
                error!(
                    site_id = site_id,
                    page_id = page_id,
                    filename = filename,
                    "Cannot get file info: {error}",
                );
                return ServerErrorCode::FileFetch {
                    site_id,
                    page_id,
                    filename,
                }
                .into_response();
            }
        };

        let body = match state.s3_bucket.get_object_stream(&file_info.s3_hash).await {
            Ok(ResponseDataStream { bytes, status_code }) => {
                assert_eq!(
                    status_code,
                    StatusCode::OK,
                    "get_object_stream() succeeded but did not reply 200",
                );
                Body::from_stream(bytes)
            }
            Err(error) => {
                // NOTE: If the error here is 404 we still return 500.
                //
                //       If we have a file record for a file, then the
                //       corresponding blob *should* exist.
                //
                //       If it doesn't, the data invariant is not being met,
                //       which is an unexpected error.
                error!(
                    site_id = site_id,
                    page_slug = page_slug,
                    filename = filename,
                    s3_hash = &file_info.s3_hash,
                    "Cannot get blob data: {error}",
                );
                return ServerErrorCode::BlobFetch {
                    site_id,
                    page_slug,
                    filename,
                }
                .into_response();
            }
        };

        (file_info, body)
    }};
}

pub async fn handle_file_redirect(Path((page_slug, filename)): Path<(String, String)>) -> Redirect {
    let destination = format!("/-/file/{page_slug}/{filename}");
    Redirect::permanent(&destination)
}

pub async fn handle_file_fetch(
    State(state): State<ServerState>,
    Path((mut page_slug, filename)): Path<(String, String)>,
    headers: HeaderMap,
) -> Response {
    info!(
        page_slug = page_slug,
        filename = filename,
        "Returning file data",
    );

    let (file_info, body) = fetch_file!(state, headers, page_slug, filename);

    let result = Response::builder()
        .header(header::CONTENT_TYPE, &file_info.mime)
        .body(body);

    match result {
        Ok(response) => response,
        Err(error) => {
            error!("Unable to convert response: {error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn handle_file_download(
    State(state): State<ServerState>,
    Path((mut page_slug, filename)): Path<(String, String)>,
    headers: HeaderMap,
) -> Response {
    info!(
        page_slug = page_slug,
        filename = filename,
        "Returning file download",
    );

    let (file_info, body) = fetch_file!(state, headers, page_slug, filename);

    Attachment::new(body)
        .filename(&filename)
        .content_type(&file_info.mime)
        .into_response()
}
