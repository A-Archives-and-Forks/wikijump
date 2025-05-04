/*
 * endpoints/special_error.rs
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

use super::prelude::*;
use crate::services::special_error::{SpecialErrorOutput, SpecialErrorService};
use crate::utils::parse_locales;

pub async fn special_error_missing_site_slug(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<SpecialErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
        site_slug: String,
    }

    let Input { locales, site_slug } = params.parse()?;
    let locales = parse_locales(&locales)?;
    SpecialErrorService::missing_site_slug(ctx, &locales, &site_slug).await
}

pub async fn special_error_missing_custom_domain(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<SpecialErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
        domain: String,
    }

    let Input { locales, domain } = params.parse()?;
    let locales = parse_locales(&locales)?;
    SpecialErrorService::missing_custom_domain(ctx, &locales, &domain).await
}

pub async fn special_error_missing_page_slug(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<SpecialErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
        site_id: i64,
        page_slug: String,
    }

    let Input {
        locales,
        site_id,
        page_slug,
    } = params.parse()?;

    let locales = parse_locales(&locales)?;
    SpecialErrorService::missing_page_slug(ctx, &locales, site_id, &page_slug).await
}

pub async fn special_error_page_fetch(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<SpecialErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
        site_id: i64,
        page_slug: String,
    }

    let Input {
        locales,
        site_id,
        page_slug,
    } = params.parse()?;

    let locales = parse_locales(&locales)?;
    SpecialErrorService::page_fetch(ctx, &locales, site_id, &page_slug).await
}

pub async fn special_error_missing_file_name(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<SpecialErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
        site_id: i64,
        page_slug: String,
        filename: String,
    }

    let Input {
        locales,
        site_id,
        page_slug,
        filename,
    } = params.parse()?;

    let locales = parse_locales(&locales)?;
    SpecialErrorService::missing_file_name(ctx, &locales, site_id, &page_slug, &filename)
        .await
}

pub async fn special_error_file_fetch(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<SpecialErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
        site_id: i64,
        page_slug: String,
        filename: String,
    }

    let Input {
        locales,
        site_id,
        page_slug,
        filename,
    } = params.parse()?;

    let locales = parse_locales(&locales)?;
    SpecialErrorService::file_fetch(ctx, &locales, site_id, &page_slug, &filename).await
}

pub async fn special_error_text_block(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<SpecialErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
        site_id: i64,
        reason: String,
        index: String,
    }

    let Input {
        locales,
        site_id,
        reason,
        index,
    } = params.parse()?;

    let locales = parse_locales(&locales)?;
    SpecialErrorService::text_block(ctx, &locales, site_id, &reason, &index).await
}

pub async fn special_error_file_root(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<SpecialErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
    }

    let Input { locales } = params.parse()?;
    let locales = parse_locales(&locales)?;
    SpecialErrorService::file_root(ctx, &locales).await
}
