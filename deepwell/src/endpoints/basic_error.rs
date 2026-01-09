/*
 * endpoints/basic_error.rs
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
use crate::services::basic_error::{BasicErrorOutput, BasicErrorService};
use crate::utils::parse_locales;

pub async fn basic_error_missing_site_slug(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<BasicErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
        site_slug: String,
    }

    let Input { locales, site_slug } = params.parse()?;
    let locales = parse_locales(&locales)?;
    BasicErrorService::missing_site_slug(ctx, &locales, &site_slug).await
}

pub async fn basic_error_missing_custom_domain(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<BasicErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
        domain: String,
    }

    let Input { locales, domain } = params.parse()?;
    let locales = parse_locales(&locales)?;
    BasicErrorService::missing_custom_domain(ctx, &locales, &domain).await
}

pub async fn basic_error_missing_page_slug(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<BasicErrorOutput> {
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
    BasicErrorService::missing_page_slug(ctx, &locales, site_id, &page_slug).await
}

pub async fn basic_error_page_fetch(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<BasicErrorOutput> {
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
    BasicErrorService::page_fetch(ctx, &locales, site_id, &page_slug).await
}

pub async fn basic_error_missing_file_name(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<BasicErrorOutput> {
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
    BasicErrorService::missing_file_name(ctx, &locales, site_id, &page_slug, &filename)
        .await
}

pub async fn basic_error_file_fetch(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<BasicErrorOutput> {
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
    BasicErrorService::file_fetch(ctx, &locales, site_id, &page_slug, &filename).await
}

pub async fn basic_error_text_block(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<BasicErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
        site_id: i64,
        index: String,
        block_type: String,
        reason: String,
    }

    let Input {
        locales,
        site_id,
        index,
        block_type,
        reason,
    } = params.parse()?;

    let locales = parse_locales(&locales)?;

    BasicErrorService::text_block(ctx, &locales, site_id, &index, &block_type, &reason)
        .await
}

pub async fn basic_error_file_root(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<BasicErrorOutput> {
    #[derive(Deserialize, Debug)]
    struct Input {
        locales: Vec<String>,
    }

    let Input { locales } = params.parse()?;
    let locales = parse_locales(&locales)?;
    BasicErrorService::file_root(ctx, &locales).await
}
