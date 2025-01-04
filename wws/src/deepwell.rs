/*
 * deepwell.rs
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

use anyhow::Result;
use jsonrpsee::{http_client::HttpClient, rpc_params};
use std::time::Duration;

const JSONRPC_MAX_REQUEST: u32 = 16 * 1024;
const JSONRPC_TIMEOUT: Duration = Duration::from_millis(200);

#[derive(Debug)]
pub struct Deepwell {
    client: HttpClient,
}

impl Deepwell {
    pub fn new(deepwell_url: &str) -> Result<Self> {
        let client = HttpClient::builder()
            .max_request_size(JSONRPC_MAX_REQUEST)
            .request_timeout(JSONRPC_TIMEOUT)
            .build(deepwell_url)?;

        Ok(Deepwell { client })
    }
}
