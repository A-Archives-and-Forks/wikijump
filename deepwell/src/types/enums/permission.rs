/*
 * types/enums/permission.rs
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

use sea_orm::DeriveValueType;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
    DeriveValueType,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    Display,
    Serialize,
)]
#[sea_orm(value_type = "String")]
#[strum(serialize_all = "kebab_case", ascii_case_insensitive)]
#[serde(rename_all = "kebab-case")]
pub enum Resource {
    Page,
    Role,
    Site,
}

#[derive(
    DeriveValueType,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    Display,
    Serialize,
)]
#[sea_orm(value_type = "String")]
#[strum(serialize_all = "kebab_case", ascii_case_insensitive)]
#[serde(rename_all = "kebab-case")]
pub enum Action {
    View,
    Edit,
    Create,
    Delete,
    Rename,
    Assign,
}

/// A valid (Resource, Action) pair representing a type of permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PermissionType {
    pub resource: Resource,
    pub action: Action,
}

/// Macro for generating a list of all valid Permission types, for iteration/validation purposes.
macro_rules! define_permission_types {
    ( $( $resource:ident => [ $($action:ident),+ $(,)? ] ),+ $(,)? ) => {
        impl PermissionType {
            pub const ALL: &[PermissionType] = &[
                $($(
                    PermissionType { resource: Resource::$resource, action: Action::$action },
                )+)+
            ];

            pub const fn new(resource: Resource, action: Action) -> Option<PermissionType> {
                match (resource, action) {
                    $($(
                        (Resource::$resource, Action::$action) => {
                            Some(PermissionType { resource, action })
                        }
                    )+)+
                    _ => None,
                }
            }
        }
    };
}

// Define all valid permission types.
define_permission_types! {
    Page => [View, Edit, Create, Delete, Rename],
    Role => [View, Edit, Assign],
    Site => [View, Edit],
}
