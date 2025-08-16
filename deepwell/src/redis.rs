/*
 * redis.rs
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

use crate::services::job::{
    JOB_QUEUE_DELAY, JOB_QUEUE_MAXIMUM_SIZE, JOB_QUEUE_NAME, JOB_QUEUE_PROCESS_TIME,
};
use anyhow::Result;
use redis::{ConnectionAddr, ConnectionInfo, IntoConnectionInfo, RedisConnectionInfo};
use rsmq_async::{Rsmq, RsmqConnection, RsmqOptions};

const RSMQ_NAMESPACE: &str = "rsmq";

pub async fn connect(redis_uri: &str) -> Result<(redis::Client, Rsmq)> {
    // Parse redis connection URI
    let redis = redis::Client::open(redis_uri)?;
    let rsmq_options = get_rsmq_options(redis_uri)?;
    let mut rsmq = Rsmq::new(rsmq_options).await?;

    // Set up queue if it doesn't already exist
    if !job_queue_exists(&mut rsmq).await? {
        info!("Creating Redis job queue '{JOB_QUEUE_NAME}'");
        info!("* Process time: {JOB_QUEUE_PROCESS_TIME:?}");
        info!("* Delay time:   {JOB_QUEUE_DELAY:?}");
        info!("* Maximum body: {JOB_QUEUE_MAXIMUM_SIZE:?} bytes");

        rsmq.create_queue(
            JOB_QUEUE_NAME,
            JOB_QUEUE_PROCESS_TIME,
            JOB_QUEUE_DELAY,
            JOB_QUEUE_MAXIMUM_SIZE,
        )
        .await?;
    }

    Ok((redis, rsmq))
}

fn get_rsmq_options(redis_uri: &str) -> Result<RsmqOptions> {
    let ConnectionInfo {
        addr,
        redis:
            RedisConnectionInfo {
                db,
                username,
                password,
                protocol,
            },
    } = redis_uri.into_connection_info()?;

    let (host, port) = match addr {
        ConnectionAddr::Tcp(host, port) => (host, port),
        ConnectionAddr::TcpTls { .. } => {
            panic!("Custom TLS connection not supported for Redis: {addr:#?}")
        }
        ConnectionAddr::Unix(path) => {
            panic!("Unix socket not supported for Redis: {}", path.display())
        }
    };

    Ok(RsmqOptions {
        host,
        port,
        db: db.try_into().expect("DB int value too large for u8"),
        username,
        password,
        protocol,
        ns: str!(RSMQ_NAMESPACE),
        realtime: false,
    })
}

async fn job_queue_exists(rsmq: &mut Rsmq) -> Result<bool> {
    // NOTE: Effectively the same as rsmq.list_queues().await?.contains(JOB_QUEUE_NAME),
    //       except we don't have to deal with the "&String" type issue.
    let queues = rsmq.list_queues().await?;
    let exists = queues.iter().any(|name| JOB_QUEUE_NAME == name);
    Ok(exists)
}
