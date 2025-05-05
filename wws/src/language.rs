/*
 * language.rs
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

use axum::http::header::HeaderMap;

/// Parse the `Accept-Language` header.
/// If there are no languages, or there is no header, then use English.
pub fn parse_accept_language(headers: &HeaderMap) -> Vec<String> {
    const FALLBACK_LANGUAGE: &str = "en";

    fn get_header_value(headers: &HeaderMap) -> Option<&str> {
        match headers.get("accept-language") {
            Some(value) => value.to_str().ok(),
            None => None,
        }
    }

    let header_value = match get_header_value(headers) {
        Some(value) => value,
        None => return vec![str!(FALLBACK_LANGUAGE)],
    };

    let mut languages = accept_language::parse(header_value);
    if languages.is_empty() {
        languages.push(str!(FALLBACK_LANGUAGE));
    }

    languages
}
