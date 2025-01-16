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

/// The slug for the default site.
///
/// This refers to the site displayed when you visit `wikijump.com`
/// with no subdomain component.
const DEFAULT_SITE_SLUG: &str = "www";

#[derive(Debug)]
pub enum SiteAndHost<'a> {
    Main { site_id: i64, site_slug: &'a str },
    MainMissing { site_slug: &'a str },
    MainCustom { site_id: i64, site_slug: String },
    MainCustomMissing,
    DefaultRedirect,
    File { site_id: i64, site_slug: &'a str },
    FileMissing { site_slug: &'a str },
    FileRoot,
}

pub async fn lookup_host<'a>(state: &ServerState, hostname: &'a str) -> Result<SiteAndHost<'a>> {
    let Domains {
        ref main_domain,
        ref main_domain_no_dot,
        ref files_domain,
        ref files_domain_no_dot,
        ..
    } = state.domains;

    if &hostname == main_domain_no_dot {
        // First, check if it's the default domain by itself.
        main_site_slug(state, hostname, None).await
    } else if let Some(site_slug) = hostname.strip_suffix(main_domain) {
        // Determine if it's the main domain.

        let site_id = state.get_site_slug(site_slug).await?;

        if site_slug == DEFAULT_SITE_SLUG {
            // We should be redirecting to the non-www version of the link
            return Ok(SiteAndHost::DefaultRedirect);
        }

        main_site_slug(state, hostname, Some(site_slug)).await
    } else if let Some(site_slug) = hostname.strip_suffix(files_domain) {
        // Determine if it's a files domain.
        let site_id = state.get_site_slug(site_slug).await?;
        match site_id {
            Some(site_id) => {
                // Site exists
                info!(
                    domain = hostname,
                    site_slug = site_slug,
                    site_id = site_id,
                    "Routing files site request",
                );

                Ok(SiteAndHost::File { site_id, site_slug })
            }
            None => {
                // No such site
                warn!(
                    domain = hostname,
                    site_slug = site_slug,
                    "No such site with slug (files)",
                );

                Ok(SiteAndHost::FileMissing { site_slug })
            }
        }
    } else if &hostname == files_domain_no_dot {
        // Finally, check if it's the files domain by itself.
        //
        // This is weird, wjfiles should always a site slug subdomain,
        // so in this case we just temporary redirect to the main domain,
        // stripping the path.
        //
        // Since this is expected to be uncommon, we're putting it after
        // the site files check.
        info!(domain = hostname, "Handling lone files site request");
        Ok(SiteAndHost::FileRoot)
    } else {
        // If it's anything else, it must be a custom domain.
        // Do a lookup, then set the site data as appropriate.

        match state.get_site_domain(&hostname).await? {
            Some((site_id, site_slug)) => {
                // Site exists
                info!(
                    domain = hostname,
                    site_id = site_id,
                    "Routing main site request (custom)",
                );

                Ok(SiteAndHost::MainCustom { site_id, site_slug })
            }
            None => {
                // No such site
                warn!(domain = hostname, "No such site with slug (custom)");
                Ok(SiteAndHost::MainCustomMissing)
            }
        }
    }
}

/// Process a request from `[site-slug].wikijump.com`.
///
/// Because `wikijump.com` (default) and specifying a slug
/// have essentially the same code paths, we avoid code
/// duplication by using this helper function.
async fn main_site_slug<'a>(
    state: &ServerState,
    hostname: &str,
    site_slug: Option<&'a str>,
) -> Result<SiteAndHost<'a>> {
    // This is our way of passing in "is default site" or not.
    // If it's None, it's 'wikijump.com', if it's Some(_), it's 'xxx.wikijump.com'.
    let (site_slug, is_default) = match site_slug {
        Some(site_slug) => (site_slug, false),
        None => (DEFAULT_SITE_SLUG, true),
    };

    // Return site present or missing response based on site ID.
    let site_id = state.get_site_slug(site_slug).await?;
    match site_id {
        Some(site_id) => {
            // Site exists
            info!(
                domain = hostname,
                site_id = site_id,
                "Routing main site request ({})",
                if is_default { "default" } else { "slug" },
            );

            Ok(SiteAndHost::Main { site_id, site_slug })
        }
        None => {
            // No such site
            warn!(
                domain = hostname,
                site_slug = site_slug,
                "No such site with slug (main)",
            );

            Ok(SiteAndHost::MainMissing { site_slug })
        }
    }
}
