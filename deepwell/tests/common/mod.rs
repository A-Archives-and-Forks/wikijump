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
use deepwell::config::{Config, Secrets};
use sea_orm::{DatabaseTransaction, TransactionTrait};

macro_rules! params {
    () => {{
        use jsonrpsee::types::Params;
        Params::new(None)
    }};

    ($value:expr) => {{
        use jsonrpsee::types::Params;
        Params::new(Some($value))
    }};
}

pub async fn setup() -> (ServerState, DatabaseTransaction) {
    let secrets = Secrets::load();
    let config = Config::integration_testing();

    let state = build_server_state(config, secrets)
        .await
        .expect("Unable to set up server state");

    let txn = state
        .database
        .begin()
        .await
        .expect("Unable to start database transaction");

    (state, txn)
}

macro_rules! run_endpoint {
    ($endpoint:expr, $ctx:expr, $params_value:expr $(,)?) => {
        run_endpoint!($endpoint => $ctx, params!($params_value))
    };

    ($endpoint:expr, $ctx:expr $(,)?) => {
        run_endpoint!($endpoint => $ctx, params!())
    };

    ($endpoint:expr => $ctx:expr, $params:expr) => {
        $endpoint(&$ctx, $params).await.expect(
            concat!("Call to method '", stringify!($endpoint), "' failed")
        )
    };
}

macro_rules! run_endpoint_err {
    ($endpoint:expr, $ctx:expr, $params_value:expr $(,)?) => {
        run_endpoint_err!($endpoint => $ctx, params!($params_value))
    };

    ($endpoint:expr, $ctx:expr $(,)?) => {
        run_endpoint_err!($endpoint => $ctx, params!())
    };

    ($endpoint:expr => $ctx:expr, $params:expr) => {
        $endpoint(&$ctx, $params).await.expect_err(
            concat!("Call to method '", stringify!($endpoint), "' succeeded when it should have failed")
        )
    };
}

macro_rules! cleanup {
    ($state:expr, $txn:expr, $ctx:expr $(,)?) => {{
        use std::mem;

        // Explicitly drop all these bindings to prevent reuse later
        mem::drop($ctx);

        // We always rollback since we want the database state to be the same for each test
        $txn.rollback()
            .await
            .expect("Unable to roll back transaction");

        mem::drop($state);
    }};
}
