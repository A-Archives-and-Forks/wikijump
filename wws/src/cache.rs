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

use crate::{
    deepwell::{FileData, SiteData, SiteDomainResult},
    error::Result,
};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use ref_map::*;

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

macro_rules! hdel {
    ($conn:expr, $key:expr, $field:expr $(,)?) => {
        $conn.hdel::<_, _, ()>(&$key, $field).await?
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

    pub async fn get_site_from_domain(&self, domain: &str) -> Result<Option<SiteDomainResult>> {
        type SiteDomainDataTuple = (Option<String>, Option<i64>, Option<String>, Option<String>);

        let mut conn = get_connection!(self.client);
        let key = format!("site_domain:{domain}");
        let fields = &["variant", "id", "slug", "domain"];
        let (variant, site_id, slug, domain) =
            conn.hget::<_, _, SiteDomainDataTuple>(&key, fields).await?;
        let variant = variant.ref_map(|s| s.as_str());
        match (variant, site_id, slug, domain) {
            // Each variant value has a set of fields that should be set for it
            // If a different group of fields are set, then it's invalid
            (Some("site_found"), Some(site_id), Some(slug), None) => {
                Ok(Some(SiteDomainResult::SiteFound { site_id, slug }))
            }
            (Some("site_redirect"), None, None, Some(domain)) => {
                Ok(Some(SiteDomainResult::SiteRedirect { domain }))
            }
            (Some("missing_site_slug"), None, Some(slug), None) => {
                Ok(Some(SiteDomainResult::MissingSiteSlug { slug }))
            }
            (Some("missing_custom_domain"), None, None, Some(domain)) => {
                Ok(Some(SiteDomainResult::MissingCustomDomain { domain }))
            }

            // Cache miss
            (None, None, None, None) => Ok(None),

            // Not a valid variant or set of fields
            _ => {
                clear_inconsistent_fields(&mut conn, &key, fields).await?;
                Ok(None)
            }
        }
    }

    pub async fn set_site_from_domain(
        &self,
        domain: &str,
        domain_data: &SiteDomainResult,
    ) -> Result<()> {
        let mut conn = get_connection!(self.client);
        let key = format!("site_domain:{domain}");

        let (variant, site_id, slug, domain): (
            &'static str,
            Option<i64>,
            Option<&str>,
            Option<&str>,
        ) = match domain_data {
            SiteDomainResult::SiteFound { site_id, slug } => {
                ("site_found", Some(*site_id), Some(slug), Some(domain))
            }
            SiteDomainResult::SiteRedirect { domain } => {
                ("site_redirect", None, None, Some(domain))
            }
            SiteDomainResult::MissingSiteSlug { slug } => {
                ("missing_site_slug", None, Some(slug), None)
            }
            SiteDomainResult::MissingCustomDomain { domain } => {
                ("missing_custom_domain", None, None, Some(domain))
            }
        };

        hset!(conn, key, "variant", variant);
        hset!(conn, key, "id", site_id);
        hset!(conn, key, "slug", slug);
        hset!(conn, key, "domain", domain);
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
        type FileDataTuple = (Option<i64>, Option<String>, Option<i64>, Option<String>);

        let mut conn = get_connection!(self.client);
        let key = format!("file_name:{site_id}:{page_id}:{filename}");
        let fields = &["id", "mime", "size", "s3_hash"];
        let values = conn.hget::<_, _, FileDataTuple>(&key, fields).await?;
        match values {
            // Ideally, all of these should be non-null, if it's a cache hit.
            (Some(file_id), Some(mime), Some(size), Some(s3_hash)) => Ok(Some(FileData {
                file_id,
                mime,
                size,
                s3_hash,
            })),

            // Cache miss
            (None, None, None, None) => Ok(None),

            // Some fields are set and others aren't. Let's clear all them out.
            _ => {
                clear_inconsistent_fields(&mut conn, &key, fields).await?;
                Ok(None)
            }
        }
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
        hset!(conn, key, "mime", &data.mime);
        hset!(conn, key, "size", data.size);
        hset!(conn, key, "s3_hash", &data.s3_hash);
        Ok(())
    }
}

async fn clear_inconsistent_fields(
    conn: &mut MultiplexedConnection,
    key: &str,
    fields: &[&str],
) -> Result<()> {
    warn!(key = key, "Inconsistent cache data, deleting");
    hdel!(conn, key, fields);
    Ok(())
}
