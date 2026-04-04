/*
 * types/enums.rs
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

//! Enums used across multiple services that correspond to database TEXT columns.

use sea_orm::DeriveValueType;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

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
pub enum UserType {
    Regular,
    System,
    Site,
    Bot,
}

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
pub enum AliasType {
    Site,
    User,
}

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
pub enum FileRevisionType {
    Regular,
    Rollback,
    Create,
    Delete,
    Undelete,
    Move,
}

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
pub enum PageRevisionType {
    Regular,
    Rollback,
    Undo,
    Create,
    Delete,
    Undelete,
    Move,
}

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
pub enum MessageRecipientType {
    Regular,
    Cc,
    Bcc,
}

impl sea_orm::TryFromU64 for MessageRecipientType {
    fn try_from_u64(_: u64) -> std::result::Result<Self, sea_orm::DbErr> {
        Err(sea_orm::DbErr::ConvertFromU64(
            "cannot construct MessageRecipientType from u64; auto_increment must be false for this primary key",
        ))
    }
}

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
pub enum TextBlockType {
    Code,
    Html,
}

impl sea_orm::TryFromU64 for TextBlockType {
    fn try_from_u64(_: u64) -> std::result::Result<Self, sea_orm::DbErr> {
        Err(sea_orm::DbErr::ConvertFromU64(
            "cannot construct TextBlockType from u64; auto_increment must be false for this primary key",
        ))
    }
}

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
#[strum(serialize_all = "kebab_case", ascii_case_insensitive)]
pub enum License {
    #[strum(serialize = "cc-by-sa-4.0")]
    #[serde(rename = "cc-by-sa-4.0")]
    CcBySa40,
    #[strum(serialize = "cc-by-4.0")]
    #[serde(rename = "cc-by-4.0")]
    CcBy40,
    #[strum(serialize = "cc-by-nd-4.0")]
    #[serde(rename = "cc-by-nd-4.0")]
    CcByNd40,
    #[strum(serialize = "cc-by-nc-4.0")]
    #[serde(rename = "cc-by-nc-4.0")]
    CcByNc40,
    #[strum(serialize = "cc-by-nc-sa-4.0")]
    #[serde(rename = "cc-by-nc-sa-4.0")]
    CcByNcSa40,
    #[strum(serialize = "cc-by-nc-nd-4.0")]
    #[serde(rename = "cc-by-nc-nd-4.0")]
    CcByNcNd40,
    #[strum(serialize = "cc-by-sa-3.0")]
    #[serde(rename = "cc-by-sa-3.0")]
    CcBySa30,
    #[strum(serialize = "cc-by-3.0")]
    #[serde(rename = "cc-by-3.0")]
    CcBy30,
    #[strum(serialize = "cc-by-nd-3.0")]
    #[serde(rename = "cc-by-nd-3.0")]
    CcByNd30,
    #[strum(serialize = "cc-by-nc-3.0")]
    #[serde(rename = "cc-by-nc-3.0")]
    CcByNc30,
    #[strum(serialize = "cc-by-nc-sa-3.0")]
    #[serde(rename = "cc-by-nc-sa-3.0")]
    CcByNcSa30,
    #[strum(serialize = "cc-by-nc-nd-3.0")]
    #[serde(rename = "cc-by-nc-nd-3.0")]
    CcByNcNd30,
    #[strum(serialize = "cc-by-sa-2.5")]
    #[serde(rename = "cc-by-sa-2.5")]
    CcBySa25,
    #[strum(serialize = "cc-by-2.5")]
    #[serde(rename = "cc-by-2.5")]
    CcBy25,
    #[strum(serialize = "cc-by-nd-2.5")]
    #[serde(rename = "cc-by-nd-2.5")]
    CcByNd25,
    #[strum(serialize = "cc-by-nc-2.5")]
    #[serde(rename = "cc-by-nc-2.5")]
    CcByNc25,
    #[strum(serialize = "cc-by-nc-sa-2.5")]
    #[serde(rename = "cc-by-nc-sa-2.5")]
    CcByNcSa25,
    #[strum(serialize = "cc-by-nc-nd-2.5")]
    #[serde(rename = "cc-by-nc-nd-2.5")]
    CcByNcNd25,
    #[strum(serialize = "gnu-fdl-1.3")]
    #[serde(rename = "gnu-fdl-1.3")]
    GnuFdl13,
    #[strum(serialize = "gnu-fdl-1.2")]
    #[serde(rename = "gnu-fdl-1.2")]
    GnuFdl12,
    #[strum(serialize = "gnu-fdl-1.1")]
    #[serde(rename = "gnu-fdl-1.1")]
    GnuFdl11,
    #[strum(serialize = "cc0")]
    #[serde(rename = "cc0")]
    Cc0,
}
