/*
 * framerail.rs
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

use axum::http::Uri;

#[derive(Debug)]
pub struct Framerail {
    host: String,
}

impl Framerail {
    #[inline]
    pub fn new(host: String) -> Self {
        Framerail { host }
    }

    pub fn proxy_uri(&self, path: &str) -> Uri {
        let uri = format!("http://{}{}", self.host, path);
        Uri::try_from(uri).expect("Internal framerail URI is invalid")
    }
}
