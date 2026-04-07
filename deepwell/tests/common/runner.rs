/*
 * tests/common/runner.rs
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

//! Helper functions and macros for running individual test cases.

use deepwell::api::{ServerState, build_server_state};
use deepwell::config::{Config, Secrets};
use deepwell::services::ServiceContext;
use jsonrpsee::types::Params;
use sea_orm::{DatabaseTransaction, TransactionTrait};
use self_cell::self_cell;
use serde_json::Value as JsonValue;
use tokio::task;

#[inline]
pub fn empty_params() -> Params<'static> {
    Params::new(None)
}

#[inline]
pub fn make_params(value: JsonValue) -> Params<'static> {
    // This is kind of inconvenient, converting back and forth
    // and making multiple owned buffers, but it's okay because
    // this is just for tests, and it's convenient that it enables
    // use of json! in request inputs.

    let json = serde_json::to_string(&value).expect("Unable to emit JSON");
    let params = Params::new(Some(&json));
    params.into_owned()
}

#[derive(Debug)]
pub struct TestRunnerRequestContext {
    state: ServerState,
    transaction: Option<DatabaseTransaction>,
}

impl TestRunnerRequestContext {
    pub async fn new() -> Self {
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

        TestRunnerRequestContext {
            state,
            transaction: Some(txn),
        }
    }

    fn transaction(&self) -> &DatabaseTransaction {
        // Only should be unset in Drop
        self.transaction.as_ref().expect("Should never be None")
    }

    #[inline]
    fn build_service_context<'txn>(&'txn self) -> ServiceContext<'txn> {
        ServiceContext::new(&self.state, self.transaction())
    }
}

impl Drop for TestRunnerRequestContext {
    fn drop(&mut self) {
        let txn = self
            .transaction
            .take()
            .expect("Transaction was None at time of drop");

        task::spawn(async move {
            txn.rollback()
                .await
                .expect("Unable to roll back transaction")
        });
    }
}

self_cell!(
    pub struct TestRunner {
        owner: TestRunnerRequestContext,

        #[covariant]
        dependent: ServiceContext,
    }

    impl {Debug}
);

impl TestRunner {
    pub async fn setup() -> Self {
        let request_ctx = TestRunnerRequestContext::new().await;
        Self::new(request_ctx, TestRunnerRequestContext::build_service_context)
    }
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

#[allow(unused_macros)]
macro_rules! run_endpoint {
    ($endpoint:expr, $ctx:expr, $params_value:expr $(,)?) => {
        run_endpoint!($endpoint => $ctx, common::make_params($params_value))
    };

    ($endpoint:expr, $ctx:expr $(,)?) => {
        run_endpoint!($endpoint => $ctx, common::empty_params())
    };

    ($endpoint:expr => $ctx:expr, $params:expr) => {
        // Not using .expect() because we want a custom panic message
        match $endpoint(&$ctx, $params).await {
            Ok(result) => result,
            Err(error) => {
                panic!("Call to method '{}' failed!\n{:?}", stringify!($endpoint), error);
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! run_endpoint_err {
    ($endpoint:expr, $ctx:expr, $params_value:expr $(,)?) => {
        run_endpoint_err!($endpoint => $ctx, common::make_params($params_value))
    };

    ($endpoint:expr, $ctx:expr $(,)?) => {
        run_endpoint_err!($endpoint => $ctx, common::empty_params())
    };

    ($endpoint:expr => $ctx:expr, $params:expr) => {
        // Not using .expect_err() because we want a custom panic message
        match $endpoint(&$ctx, $params).await {
            Err(error) => error,
            Ok(result) => {
                panic!(
                    "Call to method '{}' succeeded when it should have failed\n{:?}",
                    stringify!($endpoint),
                    result,
                );
            }
        }
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
