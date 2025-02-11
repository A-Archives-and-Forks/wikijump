/*
 * host.rs
 *
 * Wilson's Web Server - Serves a zoo of content (framerail, user files, code, etc)
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

use crate::{deepwell::Domains, error::Result, state::ServerState};
use serde::Deserialize;

/// The slug for the default site.
///
/// This refers to the site displayed when you visit `wikijump.com`
/// with no subdomain component.
pub const DEFAULT_SITE_SLUG: &str = "www";

/// Describes which Wikijump site and router this request is pointed towards.
/// Gets the data from DEEPWELL, but adds fields for the files server routing.
///
/// * "Main" refers to the framerail handler, i.e. `[site-slug].wikijump.com`.
/// * "Files" refers to the wjfiles handlers, i.e. `[site-slug].wjfiles.com`.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case", tag = "result", content = "data")]
pub enum SiteAndHost {
    /// Main router existent site, ready to process request.
    MainSite { site_id: i64, site_slug: String },

    /// Main router existent site, request to preferred domain.
    MainSiteRedirect { domain: String },

    /// Files router, existent site.
    FileSite { site_id: i64, site_slug: String },

    /// Request is the root domain on the files router, which has no meaning.
    /// Special case.
    FileRoot,

    /// Any router, non-existent site, canonical domain.
    MissingSiteSlug { site_slug: String },

    /// Any router, non-existent site, custom domain.
    MissingCustomDomain { domain: String },
}

pub async fn lookup_host(state: &ServerState, hostname: &str) -> Result<SiteAndHost> {
    let Domains {
        ref files_domain,
        ref files_domain_no_dot,
        ..
    } = state.domains;

    if let Some(site_slug) = hostname.strip_suffix(files_domain) {
        // Determine if it's a files domain.
        let site_id = state.get_site_from_slug(site_slug).await?;
        let site_slug = site_slug.to_owned(); // We cannot use the borrowed version because
                                              // the struct is Deserialize.
        match site_id {
            // Site exists
            Some(site_id) => Ok(SiteAndHost::FileSite { site_id, site_slug }),
            // Site missing
            None => Ok(SiteAndHost::MissingSiteSlug { site_slug }),
        }
    } else if hostname == files_domain_no_dot {
        // Check if it's the files domain by itself.
        //
        // This is weird, wjfiles should always a site slug subdomain,
        // so in this case we just temporary redirect to the main domain,
        // stripping the path.
        Ok(SiteAndHost::FileRoot)
    } else {
        // If it's anything else, it must be a canonical or custom domain.
        // That means it's the main site. Let's do a lookup and let
        // DomainService handle it for us.
        //
        // This also caches the lookup, to avoid us having to talk to
        // DEEPWELL more than necessary.
        state.get_host_from_domain(hostname).await
    }
}
