/*
 * deepwell.rs
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

use crate::error::Result;
use jsonrpsee::{core::client::ClientT, http_client::HttpClient, rpc_params};
use serde::Deserialize;
use std::{num::NonZeroU16, time::Duration};

pub const BLOCK_TYPE_HTML: &str = "html";
pub const BLOCK_TYPE_CODE: &str = "code";

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
                panic!("Parameter `{}` cannot be serialized: {:?}", stringify!($key), error);
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

    // Getters

    pub async fn get_site_domain(&self, site_id: i64) -> Result<String> {
        let params = rpc_params![site_id];
        let domain: String = self.client.request("site_domain", params).await?;
        Ok(domain)
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

    pub async fn get_text_block_index(
        &self,
        page_id: i64,
        block_type: &'static str,
        name: &str,
    ) -> Result<Option<TextBlockIndex>> {
        debug_assert!(
            block_type == BLOCK_TYPE_HTML || block_type == BLOCK_TYPE_CODE,
            "Passed block type does not match an enum value: {block_type}",
        );

        let params = rpc_object! {
            "page_id" => page_id,
            "block_type" => block_type,
            "name" => name,
        };

        let block_info: Option<TextBlockIndex> =
            self.client.request("text_block_get_index", params).await?;

        Ok(block_info)
    }

    // Special errors

    pub async fn special_error_missing_site_slug(
        &self,
        locales: &[String],
        site_slug: &str,
    ) -> Result<SpecialError> {
        let params = rpc_object! {
            "locales" => locales,
            "site_slug" => site_slug,
        };

        let html: SpecialError = self
            .client
            .request("special_error_missing_site_slug", params)
            .await?;

        Ok(html)
    }

    pub async fn special_error_missing_custom_domain(
        &self,
        locales: &[String],
        domain: &str,
    ) -> Result<SpecialError> {
        let params = rpc_object! {
            "locales" => locales,
            "domain" => domain,
        };

        let html: SpecialError = self
            .client
            .request("special_error_missing_custom_domain", params)
            .await?;

        Ok(html)
    }

    pub async fn special_error_file_root(&self, locales: &[String]) -> Result<SpecialError> {
        let params = rpc_object! {
            "locales" => locales,
        };

        let html: SpecialError = self
            .client
            .request("special_error_file_root", params)
            .await?;

        Ok(html)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PageData {
    pub page_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileData {
    pub file_id: i64,
    pub mime: String,
    pub size: i64,
    pub s3_hash: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextBlockIndex {
    pub index: NonZeroU16,
    pub s3_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpecialError {
    pub title: String,
    pub body: String,
}
