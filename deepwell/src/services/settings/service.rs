/*
 * services/settings/service.rs
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

use super::prelude::*;
use crate::services::{
    CategoryService, PageRevisionService, PageService, SiteService, TextService,
};
use ftml::layout::Layout;
use std::borrow::Cow;

#[derive(Debug)]
pub struct SettingsService;

impl SettingsService {
    /// Get the layout associated with this page.
    ///
    /// If this page has a specific layout override,
    /// then that is returned. Otherwise, the layout
    /// associated with the site is used.
    ///
    /// If no page ID is specified, then searching
    /// starts with site layout settings.
    pub async fn get_layout(
        ctx: &ServiceContext<'_>,
        site_id: i64,
        page_id: Option<i64>,
    ) -> Result<Layout> {
        fn parse_layout(value: &str) -> Result<Layout> {
            value.parse().map_err(|_| Error::InvalidEnumValue)
        }

        if let Some(page_id) = page_id {
            debug!("Getting layout for site ID {site_id} page ID {page_id}");
            let page = PageService::get_direct(ctx, page_id, true).await?;
            if let Some(layout) = page.layout {
                debug!("Found page-level layout override: {layout}");
                return parse_layout(&layout);
            }

            let category_id = page.page_category_id;
            debug!("Getting layout for page category ID {category_id}");
            let category =
                CategoryService::get(ctx, site_id, Reference::Id(category_id)).await?;

            if let Some(layout) = category.layout {
                debug!("Found category-level layout override: {layout}");
                return parse_layout(&layout);
            }
        }

        debug!("Getting layout for site ID {site_id}");
        let site = SiteService::get(ctx, Reference::Id(site_id)).await?;
        if let Some(layout) = site.layout {
            debug!("Found site-level layout override: {layout}");
            return parse_layout(&layout);
        }

        debug!("Using platform-level layout");
        Ok(ctx.config().default_page_layout)
    }

    /// Get the navigation pages for this page category.
    ///
    /// If this category has nav page overrides, then those
    /// are returned. Otherwise, the respective navigation
    /// pages for the site is used.
    ///
    /// If no category ID is specified, then searching
    /// starts with site nav page settings.
    ///
    /// Note that empty strings have a special meaning,
    /// specifying that this navigation element is not included.
    pub async fn get_nav_page_slugs(
        ctx: &ServiceContext<'_>,
        site_id: i64,
        category_id: Option<i64>,
    ) -> Result<NavigationPageSlugs> {
        let site = SiteService::get(ctx, Reference::Id(site_id)).await?;
        let (override_top_bar, override_side_bar) = match category_id {
            None => (None, None),
            Some(category_id) => {
                let category =
                    CategoryService::get(ctx, site_id, Reference::Id(category_id))
                        .await?;
                (category.top_bar_page, category.side_bar_page)
            }
        };

        Ok(NavigationPageSlugs {
            top_bar_page: override_top_bar.unwrap_or(site.top_bar_page).into(),
            side_bar_page: override_side_bar.unwrap_or(site.side_bar_page).into(),
        })
    }

    /// Get the current page wikitexts for the current navigation pages.
    ///
    /// This is essentially a convenience method for `get_nav_page_slugs()`
    /// to also fetch the page wikitext values as well.
    pub async fn get_nav_page_wikitext(
        ctx: &ServiceContext<'_>,
        site_id: i64,
        category_id: Option<i64>,
    ) -> Result<NavigationPageWikitext> {
        let NavigationPageSlugs {
            top_bar_page,
            side_bar_page,
        } = Self::get_nav_page_slugs(ctx, site_id, category_id).await?;

        // Helper function so we can do a clean try_join!
        async fn get_wikitext(
            ctx: &ServiceContext<'_>,
            site_id: i64,
            page: &NavigationPage,
        ) -> Result<Option<String>> {
            // If the nav page is disabled, then no data
            let page_slug = match page {
                NavigationPage::Enabled(page_slug) => page_slug,
                NavigationPage::Disabled => return Ok(None),
            };

            // If the nav page doesn't exist, also no data
            //
            // It is not an error for individual pages if a nav page is
            // set but does not exist at the time. When it is created it
            // can be picked up and will rerender affected pages.
            let page =
                match PageService::get_optional(ctx, site_id, Reference::Slug(page_slug))
                    .await?
                {
                    Some(page) => page,
                    None => return Ok(None),
                };

            let page_revision =
                PageRevisionService::get_latest(ctx, site_id, page.page_id).await?;

            let wikitext = TextService::get(ctx, &page_revision.wikitext_hash).await?;

            Ok(Some(wikitext))
        }

        let (top_bar_page_wikitext, side_bar_page_wikitext) = try_join!(
            get_wikitext(ctx, site_id, &top_bar_page),
            get_wikitext(ctx, site_id, &side_bar_page),
        )?;

        Ok(NavigationPageWikitext {
            top_bar_page_wikitext,
            side_bar_page_wikitext,
        })
    }
}
