/*
 * services/caddy/structs.rs
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

use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct CaddyfileOptions<'a> {
    #[serde(default)]
    pub debug: bool,

    #[serde(default)]
    pub local: bool,

    #[serde(default)]
    pub http_port: Option<i64>,

    #[serde(default)]
    pub https_port: Option<i64>,

    // Infra information
    #[serde(default)]
    pub deploy_host: Option<Cow<'a, str>>,
    pub framerail_host: Cow<'a, str>,
    pub wws_host: Cow<'a, str>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SiteData {
    pub sites: Vec<(i64, String, Option<String>)>,
    pub domains: HashMap<i64, SiteDomainData>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SiteDomainData {
    pub aliases: Vec<String>,
    pub custom_domains: Vec<String>,
}
