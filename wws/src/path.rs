/*
 * path.rs
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

use axum::http::Uri;

/// Extracts the path and query from a URI.
///
/// Since `Uri::path_and_query()` returns an `Option`,
/// we need a match statement to get the path if there
/// is no query string portion.
pub fn get_path(uri: &Uri) -> &str {
    match uri.path_and_query() {
        Some(path_and_query) => path_and_query.as_str(),
        None => uri.path(),
    }
}
