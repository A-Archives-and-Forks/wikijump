/*
 * state.rs
 *
 * Wilson's Web Server - Serves a zoo of content (framerail, user files, code, etc)
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

use crate::{
    cache::Cache,
    config::Secrets,
    deepwell::{Deepwell, Domains, FileData, PageData, SiteData, SiteDomainInfo},
    error::Result,
    framerail::Framerail,
};
use axum::body::Body;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client as HyperClient},
    rt::TokioExecutor,
};
use s3::bucket::Bucket;
use std::sync::Arc;
use std::time::Duration;

const BUCKET_REQUEST_TIMEOUT: Duration = Duration::from_millis(200);

pub type ServerState = Arc<ServerStateInner>;
pub type Client = HyperClient<HttpConnector, Body>;

#[derive(Debug)]
pub struct ServerStateInner {
    pub domains: Domains,
    pub client: Client,
    pub deepwell: Deepwell,
    pub framerail: Framerail,
    pub cache: Cache,
    pub s3_bucket: Box<Bucket>,
}

pub async fn build_server_state(secrets: Secrets) -> Result<ServerState> {
    let framerail = Framerail::new(secrets.framerail_host);
    let deepwell = Deepwell::connect(&secrets.deepwell_url)?;
    deepwell.check().await;
    let domains = deepwell.domains().await?;
    let cache = Cache::connect(&secrets.redis_url)?;
    let client = HyperClient::builder(TokioExecutor::new()).build(HttpConnector::new());
    let s3_bucket = {
        let mut bucket = Bucket::new(
            &secrets.s3_bucket,
            secrets.s3_region.clone(),
            secrets.s3_credentials.clone(),
        )?;

        if secrets.s3_path_style {
            bucket = bucket.with_path_style();
        }

        bucket.request_timeout = Some(BUCKET_REQUEST_TIMEOUT);
        bucket
    };

    Ok(Arc::new(ServerStateInner {
        domains,
        client,
        deepwell,
        framerail,
        cache,
        s3_bucket,
    }))
}

impl ServerStateInner {
    // Contains implementations for the common pattern of "check the cache,
    // if not present, get it from DEEPWELL and populate it".

    pub async fn get_site_from_slug(&self, site_slug: &str) -> Result<Option<i64>> {
        match self.cache.get_site_from_slug(site_slug).await? {
            Some(site_id) => Ok(Some(site_id)),
            None => match self.deepwell.get_site_from_slug(site_slug).await? {
                None => Ok(None),
                Some(SiteData { site_id, .. }) => {
                    self.cache.set_site_from_slug(site_slug, site_id).await?;
                    Ok(Some(site_id))
                }
            },
        }
    }

    pub async fn get_site_from_domain(&self, site_domain: &str) -> Result<SiteDomainInfo> {
        match self.cache.get_site_from_domain(site_domain).await? {
            Some(domain_data) => Ok(domain_data),
            None => {
                let domain_data = self.deepwell.get_site_from_domain(site_domain).await?;
                self.cache
                    .set_site_from_domain(site_domain, &domain_data)
                    .await?;

                Ok(domain_data)
            }
        }
    }

    pub async fn get_page(&self, site_id: i64, page_slug: &str) -> Result<Option<i64>> {
        match self.cache.get_page(site_id, page_slug).await? {
            Some(page_id) => Ok(Some(page_id)),
            None => match self.deepwell.get_page(site_id, page_slug).await? {
                None => Ok(None),
                Some(PageData { page_id, .. }) => {
                    self.cache.set_page(site_id, page_slug, page_id).await?;
                    Ok(Some(page_id))
                }
            },
        }
    }

    pub async fn get_file(
        &self,
        site_id: i64,
        page_id: i64,
        filename: &str,
    ) -> Result<Option<FileData>> {
        match self.cache.get_file(site_id, page_id, filename).await? {
            Some(data) => Ok(Some(data)),
            None => match self.deepwell.get_file(site_id, page_id, filename).await? {
                None => Ok(None),
                Some(data) => {
                    self.cache
                        .set_file(site_id, page_id, filename, &data)
                        .await?;

                    Ok(Some(data))
                }
            },
        }
    }
}
