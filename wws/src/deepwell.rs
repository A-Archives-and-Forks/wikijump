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
use jsonrpsee::{core::client::ClientT, http_client::HttpClient, rpc_params};
use serde::Deserialize;
use std::time::Duration;

const JSONRPC_MAX_REQUEST: u32 = 16 * 1024;
const JSONRPC_TIMEOUT: Duration = Duration::from_millis(200);

#[derive(Debug)]
pub struct Deepwell {
    client: HttpClient,
}

impl Deepwell {
    pub fn connect(deepwell_url: &str) -> Result<Self> {
        let client = HttpClient::builder()
            .max_request_size(JSONRPC_MAX_REQUEST)
            .request_timeout(JSONRPC_TIMEOUT)
            .build(deepwell_url)?;

        Ok(Deepwell { client })
    }

    /// Attempt to ping DEEPWELL, panicking if connecting failed.
    pub async fn check(&self) {
        self.ping().await.expect("Unable to connect to DEEPWELL");
    }

    pub async fn ping(&self) -> Result<()> {
        let response: String = self.client.request("ping", rpc_params![]).await?;
        assert!(!response.is_empty());
        Ok(())
    }

    pub async fn domains(&self) -> Result<Domains> {
        #[derive(Deserialize, Debug)]
        struct Response {
            main_domain_no_dot: String,
            file_domain_no_dot: String,
            deepwell_version: String,
        }

        let Response {
            main_domain_no_dot,
            file_domain_no_dot,
            deepwell_version,
        } = self.client.request("domains", rpc_params![]).await?;

        assert!(
            !main_domain_no_dot.starts_with('.'),
            "Main domain returned from DEEPWELL starts with '.': {main_domain_no_dot:?}",
        );
        let main_domain = format!(".{main_domain_no_dot}");

        assert!(
            !file_domain_no_dot.starts_with('.'),
            "File domain returned from DEEPWELL starts with '.': {file_domain_no_dot:?}",
        );
        let file_domain = format!(".{file_domain_no_dot}");

        Ok(Domains {
            main_domain,
            main_domain_no_dot,
            file_domain,
            file_domain_no_dot,
            deepwell_version,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Domains {
    pub main_domain: String,
    pub main_domain_no_dot: String,
    pub file_domain: String,
    pub file_domain_no_dot: String,
    pub deepwell_version: String,
}
