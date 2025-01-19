/*
 * cache.rs
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

//! Manages cached data in Redis.
//!
//! Whenever you make changes to this module, make sure that the code is
//! compatible with DEEPWELL's Redis code.

use crate::{deepwell::FileData, error::Result};
use redis::AsyncCommands;

macro_rules! get_connection {
    ($client:expr) => {
        $client.get_multiplexed_async_connection().await?
    };
}

macro_rules! hset {
    ($conn:expr, $key:expr, $field:expr, $value:expr $(,)?) => {
        $conn.hset::<_, _, _, ()>(&$key, $field, $value).await?
    };
}

#[derive(Debug)]
pub struct Cache {
    client: redis::Client,
}

impl Cache {
    /// Connect to the Redis cluster.
    pub fn connect(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        Ok(Cache { client })
    }

    pub async fn get_site_from_slug(&self, site_slug: &str) -> Result<Option<i64>> {
        let mut conn = get_connection!(self.client);
        let key = format!("site_slug:{site_slug}");
        let value = conn.hget(key, "id").await?;
        Ok(value)
    }

    pub async fn set_site_from_slug(&self, site_slug: &str, site_id: i64) -> Result<()> {
        let mut conn = get_connection!(self.client);
        let key = format!("site_slug:{site_slug}");
        hset!(conn, key, "id", site_id);
        Ok(())
    }

    pub async fn get_site_from_domain(&self, domain: &str) -> Result<Option<(i64, String)>> {
        let mut conn = get_connection!(self.client);
        let key = format!("site_domain:{domain}");
        let value = conn.hget(key, &["id", "slug"]).await?;
        Ok(value)
    }

    pub async fn set_site_from_domain(
        &self,
        domain: &str,
        site_id: i64,
        site_slug: &str,
    ) -> Result<()> {
        let mut conn = get_connection!(self.client);
        let key = format!("site_domain:{domain}");
        hset!(conn, key, "id", site_id);
        hset!(conn, key, "slug", site_slug);
        Ok(())
    }

    pub async fn get_page(&self, site_id: i64, page_slug: &str) -> Result<Option<i64>> {
        let mut conn = get_connection!(self.client);
        let key = format!("page_slug:{site_id}:{page_slug}");
        let value = conn.hget(key, "id").await?;
        Ok(value)
    }

    pub async fn set_page(&self, site_id: i64, page_slug: &str, page_id: i64) -> Result<()> {
        let mut conn = get_connection!(self.client);
        let key = format!("page_slug:{site_id}:{page_slug}");
        hset!(conn, key, "id", page_id);
        Ok(())
    }

    pub async fn get_file(
        &self,
        site_id: i64,
        page_id: i64,
        filename: &str,
    ) -> Result<Option<FileData>> {
        let mut conn = get_connection!(self.client);
        let key = format!("file_name:{site_id}:{page_id}:{filename}");
        let (file_id, s3_hash) = conn.hget(key, &["id", "s3_hash"]).await?;
        Ok(Some(FileData { file_id, s3_hash }))
    }

    pub async fn set_file(
        &self,
        site_id: i64,
        page_id: i64,
        filename: &str,
        data: &FileData,
    ) -> Result<()> {
        let mut conn = get_connection!(self.client);
        let key = format!("file_name:{site_id}:{page_id}:{filename}");
        hset!(conn, key, "id", data.file_id);
        hset!(conn, key, "s3_hash", &data.s3_hash);
        Ok(())
    }
}
