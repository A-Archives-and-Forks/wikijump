/*
 * services/domain/service.rs
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

//! Service for managing domains as used by Wikijump sites.
//!
//! This service has two components, management of canonical domains (e.g. `scp-wiki.wikijump.com`)
//! and custom domains (e.g. `scpwiki.com`).

// TODO disallow custom domains that are subdomains of the main domain or files domain
// TODO disallow preferred domains for default site (www)
// TODO expire redis cache on change to domains

use super::prelude::*;
use crate::models::site::{self, Entity as Site, Model as SiteModel};
use crate::models::site_domain::{self, Entity as SiteDomain, Model as SiteDomainModel};
use crate::services::SiteService;
use std::borrow::Cow;

pub const DEFAULT_SITE_SLUG: &str = "www";

#[derive(Debug)]
pub struct DomainService;

impl DomainService {
    /// Creates a custom domain for a site.
    pub async fn create_custom(
        ctx: &ServiceContext<'_>,
        CreateCustomDomain { domain, site_id }: CreateCustomDomain,
    ) -> Result<()> {
        info!("Creating custom domain '{domain}' (site ID {site_id})");

        let txn = ctx.transaction();
        if Self::custom_domain_exists(ctx, &domain).await? {
            error!("Custom domain already exists, cannot create");
            return Err(Error::CustomDomainExists);
        }

        let model = site_domain::ActiveModel {
            domain: Set(domain),
            site_id: Set(site_id),
            created_at: Set(now()),
        };
        model.insert(txn).await?;
        Ok(())
    }

    /// Delete the given custom domain.
    ///
    /// Yields `Error::CustomDomainNotFound` if it's missing.
    pub async fn remove_custom(ctx: &ServiceContext<'_>, domain: String) -> Result<()> {
        info!("Deleting custom domain '{domain}'");

        let txn = ctx.transaction();
        let DeleteResult { rows_affected, .. } =
            SiteDomain::delete_by_id(domain).exec(txn).await?;

        if rows_affected == 1 {
            Ok(())
        } else {
            Err(Error::CustomDomainNotFound)
        }
    }

    pub async fn site_from_custom_domain_optional(
        ctx: &ServiceContext<'_>,
        domain: &str,
    ) -> Result<Option<SiteModel>> {
        info!("Getting site for custom domain {domain:?}");

        // Join with the site table so we can get that data, rather than just the ID.
        let txn = ctx.transaction();
        let model = Site::find()
            .join(JoinType::Join, site::Relation::SiteDomain.def())
            .filter(site_domain::Column::Domain.eq(domain))
            .one(txn)
            .await?;

        Ok(model)
    }

    /// Determines if the given custom domain is registered.
    #[inline]
    pub async fn custom_domain_exists(
        ctx: &ServiceContext<'_>,
        domain: &str,
    ) -> Result<bool> {
        Self::site_from_custom_domain_optional(ctx, domain)
            .await
            .map(|site| site.is_some())
    }

    #[inline]
    pub fn get_canonical(config: &Config, site_slug: &str) -> String {
        // 'main_domain' is already prefixed with .
        format!("{}{}", site_slug, config.main_domain)
    }

    /// Gets the preferred domain for the given site.
    pub fn preferred_domain<'a>(config: &'a Config, site: &'a SiteModel) -> Cow<'a, str> {
        debug!(
            "Getting preferred domain for site '{}' (ID {})",
            site.slug, site.site_id,
        );

        match &site.preferred_domain {
            Some(domain) => cow!(domain),
            None if site.slug == DEFAULT_SITE_SLUG => Self::www_domain(config),
            None => Cow::Owned(Self::get_canonical(config, &site.slug)),
        }
    }

    /// Return the preferred domain for the `www` site.
    ///
    /// This site is a special exception, instead of visiting `www.wikijump.com`
    /// it should instead redirect to just `wikijump.com`. The use of the `www`
    /// slug is an internal detail.
    #[inline]
    fn www_domain(config: &Config) -> Cow<str> {
        Cow::Borrowed(&config.main_domain_no_dot)
    }

    /// Gets all custom domains for a site.
    pub async fn list_custom(
        ctx: &ServiceContext<'_>,
        site_id: i64,
    ) -> Result<Vec<SiteDomainModel>> {
        info!("Getting domains for site ID {site_id}");

        let txn = ctx.transaction();
        let models = SiteDomain::find()
            .filter(site_domain::Column::SiteId.eq(site_id))
            .all(txn)
            .await?;

        Ok(models)
    }
}
