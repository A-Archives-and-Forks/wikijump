/*
 * services/text_block/mime.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
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

pub const MIME_HTML: &str = "text/html; charset=utf-8";
pub const MIME_TEXT: &str = "text/plain; charset=utf-8";

/// Gets the MIME type for a given `[[code]]` language specification.
pub fn mime_for_language<S: AsRef<str>>(language: &Option<S>) -> &'static str {
    let language = match language {
        Some(ref language) => language.as_ref(),
        None => return MIME_TEXT,
    };

    // TODO
    todo!()
}
