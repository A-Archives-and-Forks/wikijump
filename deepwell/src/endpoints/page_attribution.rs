/*
 * endpoints/page_attribution.rs
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
use crate::services::relation::{
    ClearPageAttributions, GetPageAttributions, PageAttribution, RelationService,
    SetPageAttributions,
};

pub async fn page_attribution_get_page(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> OldResult<Vec<PageAttribution>> {
    let input: GetPageAttributions<'_> = params.parse()?;

    info!(
        "Getting page attributions for page {:?} on site {}",
        input.page, input.site_id,
    );

    RelationService::get_page_attributions(ctx, input).await
}

pub async fn page_attribution_update(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> OldResult<Vec<PageAttribution>> {
    let input: SetPageAttributions<'_> = params.parse()?;

    info!(
        "Setting {} page attributions for page {:?} on site {} (requested by {})",
        input.attributions.len(),
        input.page,
        input.site_id,
        input.updated_by,
    );

    RelationService::set_page_attributions(ctx, input).await
}

pub async fn page_attribution_delete(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> OldResult<()> {
    let input: ClearPageAttributions<'_> = params.parse()?;

    info!(
        "Clearing page attributions for page {:?} on site {} (requested by {})",
        input.page, input.site_id, input.removed_by,
    );

    RelationService::clear_page_attributions(ctx, input).await
}
