/*
 * utils/int.rs
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

//! Fallible conversion helpers between integer types.
//!
//! These functions exist because, with `.or_raise()`,
//! Rust cannot determine the type we want to convert
//! to unless we're explicit.

use crate::error::StdResult;
use std::num::TryFromIntError;

macro_rules! impl_convert_traits {
    ($int:ty => i64) => {
        impl ConvertToI64 for $int {
            fn try_into_i64(self) -> StdResult<i64, TryFromIntError> {
                self.try_into()
            }
        }
    };

    ($int:ty => u64) => {
        impl ConvertToU64 for $int {
            fn try_into_u64(self) -> StdResult<u64, TryFromIntError> {
                self.try_into()
            }
        }
    };

    ($int:ty => usize) => {
        impl ConvertToUsize for $int {
            fn try_into_usize(self) -> StdResult<usize, TryFromIntError> {
                self.try_into()
            }
        }
    };
}

pub trait ConvertToI64 {
    fn try_into_i64(self) -> StdResult<i64, TryFromIntError>;
}

pub trait ConvertToU64 {
    fn try_into_u64(self) -> StdResult<u64, TryFromIntError>;
}

pub trait ConvertToUsize {
    fn try_into_usize(self) -> StdResult<usize, TryFromIntError>;
}

impl_convert_traits!(i64 => u64);
impl_convert_traits!(i64 => usize);

impl_convert_traits!(u64 => i64);
impl_convert_traits!(u64 => usize);

impl_convert_traits!(usize => i64);
impl_convert_traits!(usize => u64);
