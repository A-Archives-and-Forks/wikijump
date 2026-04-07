/*
 * types/enums/macros.rs
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

/// Implement `TryFromU64` for enums that cannot be constructed from a u64 primary key.
///
/// SeaORM requires this trait when an enum is used as a primary key column type.
/// These enums are represented as strings in db, so u64 conversion should always error.
macro_rules! impl_try_from_u64 {
    ($($type:ty),+ $(,)?) => {
        $(
            impl sea_orm::TryFromU64 for $type {
                fn try_from_u64(_: u64) -> std::result::Result<Self, sea_orm::DbErr> {
                    Err(sea_orm::DbErr::ConvertFromU64(concat!(
                        "cannot construct ",
                        stringify!($type),
                        " from u64; auto_increment must be false for this primary key",
                    )))
                }
            }
        )+
    };
}
