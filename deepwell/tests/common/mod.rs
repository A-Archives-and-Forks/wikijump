/*
 * tests/common/mod.rs
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

use deepwell::api::{ServerState, build_server_state};
use deepwell::config::SetupConfig;
use deepwell::services::ServiceContext;
use sea_orm::TransactionTrait;
use std::future::Future;
use std::num::NonZeroU16;

pub async fn setup() -> ServerState {
    let (config, secrets) = {
        let SetupConfig {
            mut config,
            secrets,
        } = SetupConfig::load();
        config.pid_file = None;
        config.watch_files = false;
        config.run_seeder = false;
        config.job_workers = NonZeroU16::new(2).unwrap();
        (config, secrets)
    };

    build_server_state(config, secrets)
        .await
        .expect("Unable to set up server state")
}

pub async fn test<F, Fut>(state: &ServerState, f: F)
where
    F: FnOnce(&ServiceContext) -> Fut,
    Fut: Future<Output = ()>,
{
    let txn = state
        .database
        .begin()
        .await
        .expect("Unable to start database transaction");

    let ctx = ServiceContext::new(state, &txn);

    f(&ctx).await;

    // NOTE: We always rollback since we want the database state to be the same for each test
    txn.rollback()
        .await
        .expect("Unable to roll back transaction");
}
