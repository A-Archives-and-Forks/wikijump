/*
 * endpoints/page_attribution.rs
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
use crate::services::relation::{
    GetPageAttributions, PageAttribution, RelationService, RemovePageAttribution,
    UpdatePageAttribution,
};

pub async fn page_attribution_get_page(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<Vec<PageAttribution>> {
    let input: GetPageAttributions = params.parse()?;

    info!("Getting page attributions for page {}", input.page_id);

    RelationService::get_page_attributions(ctx, input).await
}

pub async fn page_attribution_update(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<PageAttribution> {
    let input: UpdatePageAttribution = params.parse()?;

    info!(
        "Updating page attribution for page {} user {} ({:?} @ {}) (relation {:?})",
        input.page_id,
        input.user_id,
        input.metadata.attribution_type,
        input.metadata.attribution_date,
        input.relation_id,
    );

    RelationService::update_page_attribution(ctx, input).await
}

pub async fn page_attribution_delete(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<PageAttribution> {
    let input: RemovePageAttribution = params.parse()?;

    info!(
        "Deleting page attribution relation {} (requested by {})",
        input.relation_id, input.removed_by,
    );

    RelationService::remove_page_attribution(ctx, input).await
}
