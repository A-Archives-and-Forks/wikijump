/*
 * services/special_error.rs
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

//! The "special error" service.
//!
//! This produces localized HTML pages that correspond
//! to different special error conditions, such as a
//! missing site or unknown custom domain.

use super::prelude::*;
use fluent::{FluentArgs, FluentValue};
use unic_langid::LanguageIdentifier;

#[derive(Debug)]
pub struct SpecialErrorService;

impl SpecialErrorService {
    /// Produce output for a canonical site that does not exist.
    pub async fn missing_site_slug(
        ctx: &ServiceContext<'_>,
        locales: &[LanguageIdentifier],
        site_slug: &str,
    ) -> Result<String> {
        let config = ctx.config();
        let mut args = FluentArgs::new();
        args.set("slug", fluent_str!(site_slug));
        args.set("main_domain", fluent_str!(config.main_domain_no_dot));
        args.set("files_domain", fluent_str!(config.files_domain_no_dot));

        let html = ctx
            .localization()
            .translate(locales, "special-error-site-slug", &args)?;

        Ok(html.to_string())
    }

    /// Produce output for a custom domain that does not exist.
    pub async fn missing_custom_domain(
        ctx: &ServiceContext<'_>,
        locales: &[LanguageIdentifier],
        domain: &str,
    ) -> Result<String> {
        let config = ctx.config();
        let mut args = FluentArgs::new();
        args.set("custom_domain", fluent_str!(domain));
        args.set("main_domain", fluent_str!(config.main_domain_no_dot));
        args.set("files_domain", fluent_str!(config.files_domain_no_dot));

        let html =
            ctx.localization()
                .translate(locales, "special-error-site-custom", &args)?;

        Ok(html.to_string())
    }
}
