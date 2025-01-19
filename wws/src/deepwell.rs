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

use crate::error::{Error, Result};
use jsonrpsee::{core::client::ClientT, http_client::HttpClient, rpc_params};
use serde::Deserialize;
use std::time::Duration;

const JSONRPC_MAX_REQUEST: u32 = 16 * 1024;
const JSONRPC_TIMEOUT: Duration = Duration::from_millis(200);

/// Macro to create `ObjectParams` instances.
/// This is the object equivalent to `rpc_params!`, which creates `ArrayParams` instances.
macro_rules! rpc_object {
    ($($key:expr => $value:expr,)+) => { rpc_object!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {{
        use jsonrpsee::core::params::ObjectParams;

        let mut params = ObjectParams::new();
        $(
            if let Err(error) = params.insert($key, $value) {
                panic!("Parameter `{}` cannot be serialized: {:?}", stringify!($), error);
            }
        )*
        params
    }};
}

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
        debug!("Successfully pinged DEEPWELL");
        Ok(())
    }

    pub async fn domains(&self) -> Result<Domains> {
        #[derive(Deserialize, Debug)]
        struct Response {
            main_domain_no_dot: String,
            files_domain_no_dot: String,
            deepwell_version: String,
        }

        let Response {
            main_domain_no_dot,
            files_domain_no_dot,
            deepwell_version,
        } = self.client.request("domains", rpc_params![]).await?;

        assert!(
            !main_domain_no_dot.starts_with('.'),
            "Main domain returned from DEEPWELL starts with '.': {main_domain_no_dot:?}",
        );
        let main_domain = format!(".{main_domain_no_dot}");

        assert!(
            !files_domain_no_dot.starts_with('.'),
            "Files domain returned from DEEPWELL starts with '.': {files_domain_no_dot:?}",
        );
        let files_domain = format!(".{files_domain_no_dot}");

        assert_ne!(
            main_domain, files_domain,
            "Cannot set domain for main and files service!",
        );

        info!(
            main_domain = main_domain_no_dot,
            files_domain = files_domain_no_dot,
            "Got domain information from DEEPWELL {deepwell_version}",
        );

        Ok(Domains {
            main_domain,
            main_domain_no_dot,
            files_domain,
            files_domain_no_dot,
            deepwell_version,
        })
    }

    pub async fn get_site_from_slug(&self, slug: &str) -> Result<Option<SiteData>> {
        let site_data: Option<SiteData> = self
            .client
            .request("site_get", rpc_object! { "site" => slug })
            .await?;

        Ok(site_data)
    }

    pub async fn get_site_from_domain(&self, domain: &str) -> Result<Option<SiteData>> {
        let site_data: Option<SiteData> = self
            .client
            .request("site_from_domain", rpc_params![domain])
            .await?;

        Ok(site_data)
    }

    pub async fn get_page(&self, site_id: i64, page_slug: &str) -> Result<Option<PageData>> {
        let params = rpc_object! {
            "site_id" => site_id,
            "page" => page_slug,
            "wikitext" => false,
            "compiled" => false,
        };

        let page_data: Option<PageData> = self.client.request("page_get", params).await?;

        Ok(page_data)
    }

    pub async fn get_file(
        &self,
        site_id: i64,
        page_id: i64,
        filename: &str,
    ) -> Result<Option<FileData>> {
        let params = rpc_object! {
            "site_id" => site_id,
            "page_id" => page_id,
            "file" => filename,
            "data" => false,
        };

        let file_data: Option<FileData> = self.client.request("file_get", params).await?;
        Ok(file_data)
    }
}

#[derive(Debug, Clone)]
pub struct Domains {
    pub main_domain: String,
    pub main_domain_no_dot: String,
    pub files_domain: String,
    pub files_domain_no_dot: String,
    pub deepwell_version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SiteData {
    pub site_id: i64,
    pub slug: String,
    pub name: String,
    pub custom_domain: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PageData {
    pub page_id: i64,
    pub title: String,
    pub alt_title: Option<String>,
    pub hidden_fields: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileData {
    pub file_id: i64,
    pub s3_hash: Vec<u8>,
}
