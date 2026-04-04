/*
 * types/connection_type.rs
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

use crate::error::prelude::*;
use sea_orm::{TryFromU64, entity::prelude::*};
use std::str::FromStr;
use strum_macros::EnumIter;
use strum_macros::{Display, EnumString};

#[derive(
    EnumIter,
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    DeriveValueType,
    EnumString,
    Display,
)]
#[sea_orm(value_type = "String")]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab_case", ascii_case_insensitive)]
pub enum ConnectionType {
    IncludeMessy,
    IncludeElements,
    Component,
    Link,
    Redirect,
}

impl TryFromU64 for ConnectionType {
    fn try_from_u64(_: u64) -> std::result::Result<Self, DbErr> {
        Err(DbErr::ConvertFromU64(
            "cannot construct ConnectionType from u64; auto_increment must be false for this primary key",
        ))
    }
}

/// Ensure `ConnectionType::name()` produces the same output as serde.
#[test]
fn name_serde() {
    use strum::IntoEnumIterator;

    for variant in ConnectionType::iter() {
        let output = serde_json::to_string(&variant).expect("Unable to serialize JSON");
        let serde_name: String =
            serde_json::from_str(&output).expect("Unable to deserialize JSON");

        assert_eq!(
            serde_name,
            variant.to_string(),
            "Serde name does not match variant name",
        );

        let converted: ConnectionType =
            serde_name.as_str().parse().expect("Could not convert item");

        assert_eq!(converted, variant, "Converted item does not match variant");
    }
}
