/*
 * handler/mod.rs
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

mod code;
mod error;
mod file;
mod html;
mod misc;
mod redirect;
mod robots;
mod well_known;

pub use self::code::*;
pub use self::error::FallbackError;
pub use self::file::*;
pub use self::html::*;
pub use self::misc::*;
pub use self::redirect::*;
pub use self::robots::*;
pub use self::well_known::*;

use axum::http::header::{HeaderMap, HeaderName};

pub const HEADER_IS_WIKIJUMP: HeaderName = HeaderName::from_static("x-wikijump");
pub const HEADER_SITE_ID: HeaderName = HeaderName::from_static("x-wikijump-site-id");
pub const HEADER_SITE_SLUG: HeaderName = HeaderName::from_static("x-wikijump-site-slug");
pub const HEADER_TARGET_SERVER: HeaderName = HeaderName::from_static("x-wikijump-target-server");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TargetServer {
    Main,
    Files,
}

/// Helper function to get the site ID and slug from headers.
fn get_site_info(headers: &HeaderMap) -> (i64, &str) {
    let site_id = headers
        .get(HEADER_SITE_ID)
        .expect("No site ID header in request")
        .to_str()
        .expect("Site ID header is not UTF-8")
        .parse()
        .expect("Site ID is not a valid integer");

    let site_slug = headers
        .get(HEADER_SITE_SLUG)
        .expect("No site slug header in request")
        .to_str()
        .expect("Site slug header is not UTF-8");

    (site_id, site_slug)
}

/// Helper function to get which target server Caddy has told us we are.
///
/// This is either `main` or `files`, and refers to whether routes like
/// `robots.txt` are `foo.wikijump.com` or `foo.wjfiles.com`.
fn get_target_server(headers: &HeaderMap) -> TargetServer {
    let value = headers
        .get(HEADER_TARGET_SERVER)
        .expect("No target server header in request")
        .as_bytes();

    match value {
        b"main" => TargetServer::Main,
        b"files" => TargetServer::Files,
        _ => panic!("Invalid header value: {value:?}"),
    }
}
