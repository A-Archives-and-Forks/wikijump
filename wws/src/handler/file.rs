/*
 * handler/file.rs
 *
 * Wilson's Web Server - Serves a zoo of user-generated content
 * Copyright (C) 2019-2026 Wikijump Team
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

use super::get_site_id;
use crate::{
    deepwell::FileData,
    error::{BasicError, ResponseResult, build_basic_error_response},
    state::ServerState,
};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{
        Method, StatusCode,
        header::{self, HeaderMap},
    },
    response::{IntoResponse, Response},
};
use s3::request::request_trait::ResponseDataStream;
use std::fmt::Write;
use wikidot_normalize::normalize;

const MULTIPART_BOUNDARY: &str = "wikijump_byteranges";

// Reject requests with more than this many ranges to limit DoS surface
const MAX_RANGES: usize = 10;

// HTTP Range support (see RFC 9110 §14)

#[derive(Debug, Clone, Copy)]
struct ByteRange {
    start: u64,
    end: u64,
}

impl ByteRange {
    fn len(self) -> u64 {
        self.end - self.start + 1
    }
}

#[derive(Debug)]
enum ParsedRange {
    // No range requested or header was malformed
    None,

    // One or more satisfiable byte ranges
    Satisfiable(Vec<ByteRange>),

    // Valid header but every range is unsatisfiable
    NotSatisfiable,
}

fn parse_range_header(value: &str, file_size: u64) -> ParsedRange {
    let spec = match value.trim().strip_prefix("bytes=") {
        Some(s) => s,
        None => return ParsedRange::None,
    };

    if file_size == 0 {
        return ParsedRange::NotSatisfiable;
    }

    let mut ranges = Vec::new();
    let mut any_part = false;

    for part in spec.split(',') {
        let part = part.trim();
        if part.is_empty() {
            return ParsedRange::None;
        }
        any_part = true;

        if let Some(suffix_str) = part.strip_prefix('-') {
            // "-n" -> last n bytes
            let n: u64 = match suffix_str.trim().parse() {
                Ok(n) if n > 0 => n,
                _ => return ParsedRange::None,
            };
            let start = file_size.saturating_sub(n);
            ranges.push(ByteRange {
                start,
                end: file_size - 1,
            });
        } else if let Some((start_str, end_str)) = part.split_once('-') {
            let start: u64 = match start_str.trim().parse() {
                Ok(n) => n,
                Err(_) => return ParsedRange::None,
            };
            let end_str = end_str.trim();

            if end_str.is_empty() {
                // "n-" -> byte n to EOF
                if start >= file_size {
                    continue; // unsatisfiable, skip
                }
                ranges.push(ByteRange {
                    start,
                    end: file_size - 1,
                });
            } else {
                let end: u64 = match end_str.parse() {
                    Ok(n) => n,
                    Err(_) => return ParsedRange::None,
                };
                if start > end {
                    return ParsedRange::None;
                }
                if start >= file_size {
                    continue; // unsatisfiable, skip
                }
                ranges.push(ByteRange {
                    start,
                    end: end.min(file_size - 1),
                });
            }
        } else {
            return ParsedRange::None;
        }

        if ranges.len() > MAX_RANGES {
            return ParsedRange::None;
        }
    }

    if !any_part {
        return ParsedRange::None;
    }

    if ranges.is_empty() {
        ParsedRange::NotSatisfiable
    } else {
        ParsedRange::Satisfiable(ranges)
    }
}

// File info lookup (no S3 body fetch)

async fn fetch_file_info(
    state: &ServerState,
    headers: &HeaderMap,
    page_slug: &mut String,
    filename: &str,
) -> ResponseResult<FileData> {
    normalize(page_slug);

    let site_id = get_site_id(headers);
    let page_id = state
        .get_page_or_response(headers, site_id, page_slug)
        .await?;

    state
        .get_file_or_response(headers, site_id, page_id, page_slug, filename)
        .await
}

async fn fetch_full_body(
    state: &ServerState,
    headers: &HeaderMap,
    file_info: &FileData,
    page_slug: &str,
    filename: &str,
) -> ResponseResult<Body> {
    match state
        .s3_files_bucket
        .get_object_stream(&file_info.s3_hash)
        .await
    {
        Ok(ResponseDataStream { bytes, status_code }) => {
            assert_eq!(
                status_code,
                StatusCode::OK,
                "get_object_stream() succeeded but did not reply 200",
            );
            Ok(Body::from_stream(bytes))
        }
        Err(error) => {
            // NOTE: If the error here is 404 we still return 500.
            //
            //       If we have a file record for a file, then the
            //       corresponding blob *should* exist.
            //
            //       If it doesn't, the data invariant is not being met,
            //       which is an unexpected error.
            let site_id = get_site_id(headers);
            error!(
                site_id = site_id,
                page_slug = page_slug,
                filename = filename,
                s3_hash = &file_info.s3_hash,
                "Cannot get blob data: {error}",
            );

            let response = build_basic_error_response(
                state,
                headers,
                BasicError::FileFetch {
                    site_id,
                    page_slug,
                    filename,
                },
            )
            .await;

            Err(response)
        }
    }
}

async fn fetch_range_bytes(
    state: &ServerState,
    file_info: &FileData,
    range: ByteRange,
) -> Result<Vec<u8>, s3::error::S3Error> {
    let resp = state
        .s3_files_bucket
        .get_object_range(&file_info.s3_hash, range.start, Some(range.end))
        .await?;
    Ok(resp.to_vec())
}

// ------------ Range evaluation helpers ------------

// If `If-Range` is present and its ETag doesn't match, `Range` must be
// ignored and the full representation returned (RFC 9110 §13.1.5)
fn should_evaluate_range(headers: &HeaderMap, etag: &str) -> bool {
    match headers.get(header::IF_RANGE) {
        Some(val) => val.to_str().is_ok_and(|v| v.trim() == etag),
        None => true,
    }
}

fn evaluate_range(headers: &HeaderMap, etag: &str, file_size: u64) -> ParsedRange {
    if !should_evaluate_range(headers, etag) {
        return ParsedRange::None;
    }

    match headers.get(header::RANGE) {
        Some(val) => match val.to_str() {
            Ok(v) => parse_range_header(v, file_size),
            Err(_) => ParsedRange::None,
        },
        None => ParsedRange::None,
    }
}

// ------------ Response builders ------------

fn base_headers(
    status: StatusCode,
    etag: &str,
    as_attachment: bool,
    filename: &str,
) -> axum::http::response::Builder {
    let mut builder = Response::builder()
        .status(status)
        .header(header::ETAG, etag)
        .header(header::ACCEPT_RANGES, "bytes");

    if as_attachment {
        let escaped = filename.replace('"', "\\\"");
        builder = builder.header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{escaped}\""),
        );
    }

    builder
}

fn build_or_500(result: Result<Response<Body>, axum::http::Error>) -> Response {
    match result {
        Ok(r) => r,
        Err(error) => {
            error!("Unable to build response: {error}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// Compute the `Content-Length` of a `multipart/byteranges` body (so HEAD can skip s3)
fn multipart_content_length(mime: &str, ranges: &[ByteRange], file_size: u64) -> usize {
    let mut len: usize = 0;
    for range in ranges {
        // --boundary\r\n
        len += 2 + MULTIPART_BOUNDARY.len() + 2;
        // Content-Type: {mime}\r\n
        len += "Content-Type: ".len() + mime.len() + 2;
        // Content-Range: bytes {start}-{end}/{file_size}\r\n
        let cr = format!(
            "Content-Range: bytes {}-{}/{file_size}\r\n",
            range.start, range.end
        );
        len += cr.len();
        // blank line
        len += 2;
        // data
        len += range.len() as usize;
        // trailing \r\n
        len += 2;
    }
    // --boundary--\r\n
    len += 2 + MULTIPART_BOUNDARY.len() + 2 + 2;
    len
}

// ------------ Core serve logic ------------

async fn serve_file(
    state: &ServerState,
    method: &Method,
    headers: &HeaderMap,
    file_info: &FileData,
    as_attachment: bool,
    page_slug: &str,
    filename: &str,
) -> Response {
    let file_size = file_info.size as u64;
    let etag = format!("\"{}\"", file_info.s3_hash);
    let is_head = *method == Method::HEAD;

    match evaluate_range(headers, &etag, file_size) {
        ParsedRange::None => {
            serve_full(
                state,
                headers,
                file_info,
                &etag,
                as_attachment,
                page_slug,
                filename,
                file_size,
                is_head,
            )
            .await
        }
        ParsedRange::NotSatisfiable => build_or_500(
            Response::builder()
                .status(StatusCode::RANGE_NOT_SATISFIABLE)
                .header(header::CONTENT_RANGE, format!("bytes */{file_size}"))
                .header(header::ACCEPT_RANGES, "bytes")
                .body(Body::empty()),
        ),
        ParsedRange::Satisfiable(ref ranges) if ranges.len() == 1 => {
            serve_single_range(
                state,
                file_info,
                &etag,
                as_attachment,
                filename,
                ranges[0],
                file_size,
                is_head,
            )
            .await
        }
        ParsedRange::Satisfiable(ranges) => {
            serve_multi_range(
                state,
                file_info,
                &etag,
                as_attachment,
                filename,
                &ranges,
                file_size,
                is_head,
            )
            .await
        }
    }
}

async fn serve_full(
    state: &ServerState,
    headers: &HeaderMap,
    file_info: &FileData,
    etag: &str,
    as_attachment: bool,
    page_slug: &str,
    filename: &str,
    file_size: u64,
    is_head: bool,
) -> Response {
    let body = if is_head {
        Body::empty()
    } else {
        match fetch_full_body(state, headers, file_info, page_slug, filename).await {
            Ok(b) => b,
            Err(resp) => return resp,
        }
    };

    build_or_500(
        base_headers(StatusCode::OK, etag, as_attachment, filename)
            .header(header::CONTENT_TYPE, &file_info.mime)
            .header(header::CONTENT_LENGTH, file_size)
            .body(body),
    )
}

async fn serve_single_range(
    state: &ServerState,
    file_info: &FileData,
    etag: &str,
    as_attachment: bool,
    filename: &str,
    range: ByteRange,
    file_size: u64,
    is_head: bool,
) -> Response {
    let body = if is_head {
        Body::empty()
    } else {
        match fetch_range_bytes(state, file_info, range).await {
            Ok(d) => Body::from(d),
            Err(error) => {
                error!(
                    s3_hash = &file_info.s3_hash,
                    start = range.start,
                    end = range.end,
                    "S3 range fetch failed: {error}",
                );
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    };

    let content_range = format!("bytes {}-{}/{file_size}", range.start, range.end);

    build_or_500(
        base_headers(StatusCode::PARTIAL_CONTENT, etag, as_attachment, filename)
            .header(header::CONTENT_TYPE, &file_info.mime)
            .header(header::CONTENT_RANGE, content_range)
            .header(header::CONTENT_LENGTH, range.len())
            .body(body),
    )
}

async fn serve_multi_range(
    state: &ServerState,
    file_info: &FileData,
    etag: &str,
    as_attachment: bool,
    filename: &str,
    ranges: &[ByteRange],
    file_size: u64,
    is_head: bool,
) -> Response {
    let content_type = format!("multipart/byteranges; boundary={MULTIPART_BOUNDARY}");

    if is_head {
        let len = multipart_content_length(&file_info.mime, ranges, file_size);
        return build_or_500(
            base_headers(StatusCode::PARTIAL_CONTENT, etag, as_attachment, filename)
                .header(header::CONTENT_TYPE, content_type)
                .header(header::CONTENT_LENGTH, len)
                .body(Body::empty()),
        );
    }

    let mut body = Vec::new();

    for range in ranges {
        let data = match fetch_range_bytes(state, file_info, *range).await {
            Ok(d) => d,
            Err(error) => {
                error!(
                    s3_hash = &file_info.s3_hash,
                    start = range.start,
                    end = range.end,
                    "S3 range fetch failed: {error}",
                );
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        let mut part_header = String::new();
        let _ = write!(
            part_header,
            "--{MULTIPART_BOUNDARY}\r\n\
             Content-Type: {}\r\n\
             Content-Range: bytes {}-{}/{file_size}\r\n\
             \r\n",
            file_info.mime, range.start, range.end,
        );
        body.extend_from_slice(part_header.as_bytes());
        body.extend_from_slice(&data);
        body.extend_from_slice(b"\r\n");
    }

    body.extend_from_slice(format!("--{MULTIPART_BOUNDARY}--\r\n").as_bytes());

    build_or_500(
        base_headers(StatusCode::PARTIAL_CONTENT, etag, as_attachment, filename)
            .header(header::CONTENT_TYPE, content_type)
            .header(header::CONTENT_LENGTH, body.len())
            .body(Body::from(body)),
    )
}

// ------------ Public handlers ------------

pub async fn handle_file_fetch(
    State(state): State<ServerState>,
    method: Method,
    Path((mut page_slug, filename)): Path<(String, String)>,
    headers: HeaderMap,
) -> Response {
    info!(
        page_slug = page_slug,
        filename = filename,
        "Returning file data",
    );

    let file_info =
        match fetch_file_info(&state, &headers, &mut page_slug, &filename).await {
            Ok(info) => info,
            Err(response) => return response,
        };

    serve_file(
        &state, &method, &headers, &file_info, false, &page_slug, &filename,
    )
    .await
}

pub async fn handle_file_download(
    State(state): State<ServerState>,
    method: Method,
    Path((mut page_slug, filename)): Path<(String, String)>,
    headers: HeaderMap,
) -> Response {
    info!(
        page_slug = page_slug,
        filename = filename,
        "Returning file download",
    );

    let file_info =
        match fetch_file_info(&state, &headers, &mut page_slug, &filename).await {
            Ok(info) => info,
            Err(response) => return response,
        };

    serve_file(
        &state, &method, &headers, &file_info, true, &page_slug, &filename,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ranges(value: &str, size: u64) -> Vec<(u64, u64)> {
        match parse_range_header(value, size) {
            ParsedRange::Satisfiable(rs) => rs.iter().map(|r| (r.start, r.end)).collect(),
            _ => panic!("expected Satisfiable"),
        }
    }

    fn is_none(value: &str, size: u64) -> bool {
        matches!(parse_range_header(value, size), ParsedRange::None)
    }

    fn is_not_satisfiable(value: &str, size: u64) -> bool {
        matches!(parse_range_header(value, size), ParsedRange::NotSatisfiable)
    }

    #[test]
    fn single_range() {
        assert_eq!(ranges("bytes=0-99", 12345), vec![(0, 99)]);
    }

    #[test]
    fn open_ended() {
        assert_eq!(ranges("bytes=500-", 12345), vec![(500, 12344)]);
    }

    #[test]
    fn suffix() {
        assert_eq!(ranges("bytes=-100", 12345), vec![(12245, 12344)]);
    }

    #[test]
    fn suffix_larger_than_file() {
        assert_eq!(ranges("bytes=-99999", 100), vec![(0, 99)]);
    }

    #[test]
    fn multiple() {
        assert_eq!(
            ranges("bytes=0-99, 200-299", 12345),
            vec![(0, 99), (200, 299)],
        );
    }

    #[test]
    fn clamp_end() {
        assert_eq!(ranges("bytes=0-99999", 100), vec![(0, 99)]);
    }

    #[test]
    fn not_satisfiable_past_eof() {
        assert!(is_not_satisfiable("bytes=12345-12400", 12345));
    }

    #[test]
    fn not_satisfiable_empty_file() {
        assert!(is_not_satisfiable("bytes=0-0", 0));
    }

    #[test]
    fn malformed_no_prefix() {
        assert!(is_none("blocks=0-99", 12345));
    }

    #[test]
    fn malformed_start_gt_end() {
        assert!(is_none("bytes=100-50", 12345));
    }

    #[test]
    fn malformed_empty_part() {
        assert!(is_none("bytes=0-99,,200-299", 12345));
    }

    #[test]
    fn skip_unsatisfiable_keep_good() {
        // First range is past EOF, second is valid.
        assert_eq!(ranges("bytes=99999-100000, 0-99", 12345), vec![(0, 99)]);
    }
}
