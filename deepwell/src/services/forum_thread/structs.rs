/*
 * services/forum_thread/structs.rs
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

#![allow(dead_code)] // TEMP

use crate::types::Maybe;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateForumThread {
    pub forum_category_id: i64,
    pub user_id: i64,
    pub associated_page_id: Option<i64>,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub sticky: bool,
    #[serde(default)]
    pub from_wikidot: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateForumThread {
    pub forum_thread_id: i64,
    pub user_id: i64,

    #[serde(flatten)]
    pub body: UpdateForumThreadBody,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct UpdateForumThreadBody {
    pub forum_category_id: Maybe<i64>,
    pub title: Maybe<String>,
    pub description: Maybe<String>,
    pub sticky: Maybe<bool>,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct GetForumThread {
    pub forum_thread_id: i64,
    #[serde(default)]
    pub include_deleted: bool,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ForumThreadListOrder {
    #[default]
    Activity,
    Created,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct GetForumThreads {
    pub forum_category_id: i64,
    #[serde(default)]
    pub include_deleted: bool,
    pub start_thread_id: Option<i64>,
    #[serde(default = "default_limit")]
    pub limit: u64,
    #[serde(default)]
    pub order: ForumThreadListOrder,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct DeleteForumThread {
    pub forum_thread_id: i64,
    pub user_id: i64,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct TouchForumThread {
    pub forum_thread_id: i64,
    pub user_id: Option<i64>,
}

#[inline]
fn default_limit() -> u64 {
    20
}
