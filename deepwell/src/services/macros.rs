/*
 * services/macros.rs
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

macro_rules! find_or_error {
    ($future:expr, $error:ident $(,)?) => {
        paste! {
            $future.await?.ok_or(OldError::[<$error NotFound>])
        }
    };
}

// temorary variant while migrating off of the old error type
macro_rules! find_or_error_tmp {
    ($future:expr, $noun:expr, $error:ident $(,)?) => {
        paste! {
            match $future.await {
                Ok(Some(result)) => Ok(result),
                Ok(None) => bail!(Error::new(
                    format!("{} does not exist", $noun),
                    ErrorType::[<$error NotFound>],
                )),
                Err(error) => bail!(error),
            }
        }
    };
}
