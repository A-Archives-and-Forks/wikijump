/*
 * services/domain/structs.rs
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

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case", tag = "result", content = "data")]
pub enum SiteAndHost {
    MainSite { site_id: i64, site_slug: String },
    MainSiteRedirect { domain: String },
    MissingSiteSlug { slug: String },
    MissingCustomDomain { domain: String },
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCustomDomain {
    pub domain: String,
    pub site_id: i64,
}
