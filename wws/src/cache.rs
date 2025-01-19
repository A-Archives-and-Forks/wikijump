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

use crate::error::Result;
use redis::AsyncCommands;

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

    /// Retrieve the site ID from the slug from the cache.
    pub async fn get_site_slug(&self, site_slug: &str) -> Result<Option<i64>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let key = format!("site_slug:{site_slug}");
        let value = conn.hget(key, "id").await?;
        Ok(value)
    }

    /// Set the site ID for a site slug.
    pub async fn set_site_slug(&self, site_slug: &str, site_id: i64) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let key = format!("site_slug:{site_slug}");
        conn.hset::<_, _, _, ()>(key, "id", site_id).await?;
        Ok(())
    }

    /// Retrieve the site slug and ID from a custom domain from the cache.
    pub async fn get_site_domain(&self, domain: &str) -> Result<Option<(i64, String)>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let key = format!("site_domain:{domain}");
        let value = conn.hget(key, &["id", "slug"]).await?;
        Ok(value)
    }

    /// Set the site slug and ID for a custom domain.
    pub async fn set_site_domain(&self, domain: &str, site_id: i64, site_slug: &str) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let key = format!("site_domain:{domain}");
        conn.hset::<_, _, _, ()>(&key, "id", site_id).await?;
        conn.hset::<_, _, _, ()>(&key, "slug", site_slug).await?;
        Ok(())
    }
}
