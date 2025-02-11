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

use crate::{
    deepwell::{Domains, SiteDomainInfo},
    error::Result,
    state::ServerState,
};

/// The slug for the default site.
///
/// This refers to the site displayed when you visit `wikijump.com`
/// with no subdomain component.
pub const DEFAULT_SITE_SLUG: &str = "www";

/// Describes which Wikijump site and router this request is pointed towards.
///
/// * "Main" refers to the framerail handler, i.e. `[site-slug].wikijump.com`.
/// * "Files" refers to the wjfiles handlers, i.e. `[site-slug].wjfiles.com`.
#[derive(Debug)]
pub enum SiteAndHost<'a> {
    /// Main router existent site, canonical domain.
    Main { site_id: i64, site_slug: String },

    /// Main router, non-existent site, canonical domain.
    MainSiteSlugMissing { site_slug: String },

    /// Main router, non-existent site, custom domain.
    MainCustomMissing { domain: String },

    /// Main router, request to preferred domain for the site.
    MainSiteRedirect { domain: String },

    /// Files router, existent site.
    File { site_id: i64, site_slug: &'a str },

    /// Files router, non-existent site.
    FileMissing { site_slug: &'a str },

    /// Request is the root domain on the files router, which has no meaning.
    /// Special case.
    FileRoot,
}

pub async fn lookup_host<'a>(state: &ServerState, hostname: &'a str) -> Result<SiteAndHost<'a>> {
    let Domains {
        ref files_domain,
        ref files_domain_no_dot,
        ..
    } = state.domains;

    if let Some(site_slug) = hostname.strip_suffix(files_domain) {
        // Determine if it's a files domain.
        let site_id = state.get_site_from_slug(site_slug).await?;
        match site_id {
            Some(site_id) => {
                // Site exists
                info!(
                    r#type = "files",
                    domain = hostname,
                    site_slug = site_slug,
                    site_id = site_id,
                    "Routing site request",
                );
                Ok(SiteAndHost::File { site_id, site_slug })
            }
            None => {
                // No such site
                warn!(
                    r#type = "files",
                    domain = hostname,
                    site_slug = site_slug,
                    "No such site with slug",
                );
                Ok(SiteAndHost::FileMissing { site_slug })
            }
        }
    } else if hostname == files_domain_no_dot {
        // Finally, check if it's the files domain by itself.
        //
        // This is weird, wjfiles should always a site slug subdomain,
        // so in this case we just temporary redirect to the main domain,
        // stripping the path.
        //
        // Since this is expected to be uncommon, we're putting it after
        // the site files check.
        info!(
            r#type = "files",
            domain = hostname,
            "Handling lone files site request",
        );
        Ok(SiteAndHost::FileRoot)
    } else {
        // If it's anything else, it must be a canonical or custom domain.
        // Let's do a lookup and let DomainService handle it for us.
        //
        // This also caches the lookup, to avoid us having to talk to
        // DEEPWELL more than necessary.
        //
        // Then we map it to the corresponding SiteAndHost variant.

        match state.get_site_from_domain(hostname).await? {
            SiteDomainInfo::SiteFound {
                site_id,
                slug: site_slug,
            } => {
                info!(
                    r#type = "main",
                    domain = hostname,
                    site_id = site_id,
                    site_slug = site_slug,
                    "Routing site request",
                );
                Ok(SiteAndHost::Main { site_id, site_slug })
            }
            SiteDomainInfo::SiteRedirect { domain } => {
                info!(
                    r#type = "main",
                    domain = domain,
                    "Found site, but needs redirect to preferred",
                );
                Ok(SiteAndHost::MainSiteRedirect { domain })
            }
            SiteDomainInfo::MissingSiteSlug { slug: site_slug } => {
                info!(
                    r#type = "main",
                    domain = hostname,
                    site_slug = site_slug,
                    "No such site with slug",
                );
                Ok(SiteAndHost::MainSiteSlugMissing { site_slug })
            }
            SiteDomainInfo::MissingCustomDomain { domain } => {
                info!(
                    r#type = "main",
                    domain = domain,
                    "No such site with custom domain",
                );
                Ok(SiteAndHost::MainCustomMissing { domain })
            }
        }
    }
}
