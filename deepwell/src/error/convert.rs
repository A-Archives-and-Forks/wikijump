/*
 * error/convert.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2026 Wikijump Team
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

//! This module concerns conversion methods for constrainted error types.
//!
//! JSONRPC expects a very particular kind of error structure, which our
//! error type does account for, but it needs some conversion to take
//! the stack of `exn::Frame`s and convert it.

use super::{Error, ErrorType};
use exn::{Error as ExnErrorTrait, Exn, Frame};
use jsonrpsee::types::error::ErrorObjectOwned;
use sea_orm::TransactionError;
use serde_json::json;

/// Unwraps Sea-ORM transaction error into a standard crate error.
///
/// Sea-ORM wraps all results from transactions in this enum.
/// This function either passes through the real error or makes
/// a new layer if it's a database connectivity issue.
pub fn unwrap_transaction_error(txn_error: TransactionError<Exn<Error>>) -> Exn<Error> {
    match txn_error {
        TransactionError::Transaction(error) => error,
        TransactionError::Connection(error) => error.raise().raise(Error::new(
            "database transaction failed",
            ErrorType::DatabaseQuery,
        )),
    }
}

/// Converts an `Exn<deepwell::error::Error>` to a JSONRPC object type.
///
/// This is not a `From` implementation since, technically, `Exn<T>` is a
/// foreign type. 🙁
pub fn exn_error_to_rpc_error(exn_error: Exn<Error>) -> ErrorObjectOwned {
    // Traverse the tree until we hit the highest-level Error
    // that is not a high-level error type, as it's not going
    // to be the most useful Error to emit as the description.
    fn walk(frame: &Frame) -> Option<&Error> {
        match frame.as_any().downcast_ref::<Error>() {
            Some(err) if !err.error_type.is_high_level() => Some(err),
            _ => frame.children().iter().find_map(walk),
        }
    }

    match walk(exn_error.as_frame()) {
        // Get the top non-request Error
        Some(error) => {
            let error_code = error.code();
            let message = error.summary();
            let data = match error.error_type {
                // Special case, if authentication then don't include call trace
                // See comment in auth_login in endpoints/auth.rs
                ErrorType::InvalidAuthentication => None,

                // Normal case, provide error context
                _ => Some(json!({
                    "call_trace": format!("{exn_error:?}"),
                    "extra": error.data(),
                })),
            };
            ErrorObjectOwned::owned(error_code, message, data)
        }

        // No crate Error exists in chain,
        // just return as string.
        None => {
            let message = str!(exn_error);
            let data = json!({
                "call_trace": format!("{exn_error:?}"),
            });
            ErrorObjectOwned::owned(0, message, Some(data))
        }
    }
}
