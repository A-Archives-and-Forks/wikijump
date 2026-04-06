/*
 * tests/common/assert.rs
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

//! Functions and macros for performing assertions in tests.

use deepwell::error::prelude::*;
use exn::{Exn, Frame};

/// Extract the deepwell error from the `Exn<Error>` which matches a condition.
///
/// This walks the error tree until it finds the first `Error` type which
/// matches the `ErrorType`-based condition passed in.
pub fn extract_error<'e, F>(exn_error: &'e Exn<Error>, condition: F) -> Option<&'e Error>
where
    F: Fn(&ErrorType) -> bool + Copy,
{
    fn walk<'e, F>(frame: &'e Frame, condition: F) -> Option<&'e Error>
    where
        F: Fn(&ErrorType) -> bool + Copy,
    {
        match frame.error().downcast_ref::<Error>() {
            Some(found) if condition(&found.error_type) => Some(&found),
            _ => frame
                .children()
                .iter()
                .find_map(|frame| walk(frame, condition)),
        }
    }

    walk(exn_error.frame(), condition)
}

/// Extract the deepwell error from the `Exn<Error>` which matches an error type pattern.
///
/// This is a wrapper macro for `extract_error()` which allows you to pass in a pattern
/// to be evaluated in `matches!`.
macro_rules! extract_error {
    ($exn_error:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        crate::common::extract_error(
            &$exn_error,
            |etype| matches!(etype, $pattern $(if $guard)?),
        )
    };
}

/// Asserts that there exists an error type matching the given pattern within the `Exn<Error>`.
macro_rules! assert_contains_error {
    ($exn_error:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {{
        let exn_error = &$exn_error;
        let extracted = extract_error!(exn_error, $pattern $(if $guard)?);
        assert!(
            extracted.is_some(),
            "Cannot find error within trace matching '{}':\n{:?}",
            stringify!($pattern),
            exn_error,
        );
    }};
}

/// Asserts that there are no erroras within the `Exn<Error>` matching the given pattern.
macro_rules! assert_no_error {
    ($exn_error:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {{
        let exn_error = &$exn_error;
        let extracted = extract_error!(exn_error, $pattern $(if $guard)?);
        assert!(
            extracted.is_none(),
            "Found error within trace matching '{}': {:?}\n{:?}",
            stringify!($pattern),
            extracted.unwrap(), // known to exist at this point
            exn_error,
        );
    }};
}

/// Allows for equality assertions on `Option<String>` without boilerplate.
///
/// This avoids the type annoyance that comes with these two types,
/// for improved readability within test code.
///
/// This code is simple, readable, and doesn't work:
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

        let right: Option<&str> = $right;

        assert_eq!(left, right);
    }};
}
