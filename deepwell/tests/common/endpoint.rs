/*
 * tests/common/endpoint.rs
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

#![allow(unused_macros)]

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
