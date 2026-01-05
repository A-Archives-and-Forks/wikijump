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

//! Common utilities for running DEEPWELL integration tests.

use deepwell::api::{ServerState, build_server_state};
use deepwell::config::{Config, Secrets};
use sea_orm::{DatabaseTransaction, TransactionTrait};

// Endpoint invocation

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

// Test setup

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

#[allow(unused_macros)]
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

// Test macros

/// Allows for equality assertions on `Option<String>` without boilerplate.
///
/// This avoids the type annoyance that comes with these two types,
/// for improved readability within test code.
///
/// This code is simple, readable, and doens't work:
/// ```compile_fail
/// let left: Option<String> = Some(str!("foo"));
/// assert_eq!(left, Some("foo"));
/// ```
///
/// So instead, we have to unwrap it, which either means a bulky
/// `.expect()` message or a separate assertion in front of it to
/// catch cases of `None`. Both are inelegant:
///
/// ```
/// # let left: Option<String> = Some(str!("foo"));
/// assert_eq!(left.expect("Left is None"), "foo");
/// ```
///
/// ```
/// # let left: Option<String> = Some(str!("foo"));
/// assert!(left.is_some());
/// assert_eq!(left.unwrap(), "foo");
/// ```
///
/// Instead, we can use this macro to compare them, which coerces both
/// types to `Option<&str>` instead, permitting a more natural comparison:
/// ```
/// # let left: Option<String> = Some(str!("foo"));
/// assert_str_eq!(left, Some("foo"));
/// ```
#[allow(unused_macros)]
macro_rules! assert_str_eq {
    ($left:expr, $right:expr $(,)?) => {{
        let left: Option<&str> = match $left {
            Some(ref s) => Some(s.as_str()),
            None => None,
        };

        let right: Option<&str> = match $right {
            Some(ref s) => Some(s.as_str()),
            None => None,
        };

        assert_eq!(left, right);
    }};
}
