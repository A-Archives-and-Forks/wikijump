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

use crate::Secrets;
use anyhow::Result;
use redis::{Client as RedisClient, IntoConnectionInfo, RedisError};
use s3::bucket::Bucket;
use std::time::Duration;

const BUCKET_REQUEST_TIMEOUT: Duration = Duration::from_millis(200);

#[derive(Debug)]
pub struct ServerState {
    redis: redis::Client,
    s3_bucket: Box<Bucket>,
}

impl ServerState {
    pub fn build(secrets: Secrets) -> Result<Self> {
        let redis = redis::Client::open(secrets.redis_url)?;
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

        Ok(ServerState { redis, s3_bucket })
    }
}
