/*
 * error/mod.rs
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

pub mod prelude {
    pub use super::{
        NewError as Error, NewErrorType as ErrorType, NewResult as Result, StdError,
    };
    pub use exn::ResultExt;
}

mod new;
mod old;

pub use self::new::{
    Error as NewError, ErrorType as NewErrorType, exn_error_to_rpc_error,
};
pub use self::old::OldError as Error;
pub use exn::Result as ExnResult;
pub use std::error::Error as StdError;

pub type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = StdResult<T, Error>;
pub type NewResult<T> = ExnResult<T, NewError>;
