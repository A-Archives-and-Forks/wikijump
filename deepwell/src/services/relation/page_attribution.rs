/*
 * services/relation/page_attribution.rs
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
use time::{Date, OffsetDateTime};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PageAttributionKind {
    Author,
    Rewrite,
    Translator,
    Maintainer,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageAttributionMetadata {
    pub attribution_type: PageAttributionKind,
    pub attribution_date: Date,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PageAttribution {
    pub relation_id: i64,
    pub page_id: i64,
    pub user_id: i64,
    pub created_by: i64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub overwritten_by: Option<i64>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub overwritten_at: Option<OffsetDateTime>,
    pub deleted_by: Option<i64>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
    pub metadata: PageAttributionMetadata,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreatePageAttribution {
    pub page_id: i64,
    pub user_id: i64,
    pub metadata: PageAttributionMetadata,
    pub created_by: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdatePageAttribution {
    pub relation_id: Option<i64>,
    pub page_id: i64,
    pub user_id: i64,
    pub metadata: PageAttributionMetadata,
    pub created_by: i64,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct GetPageAttributions {
    pub page_id: i64,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct RemovePageAttribution {
    pub relation_id: i64,
    pub removed_by: i64,
}

impl RelationService {
    /// Creates a page attribution relation without overwriting other attributions
    /// for the same page / user combination. This mirrors the unique constraint of
    /// (page_id, user_id, attribution_type, attribution_date) from the former
    /// dedicated table.
    pub async fn create_page_attribution(
        ctx: &ServiceContext<'_>,
        CreatePageAttribution {
            page_id,
            user_id,
            metadata,
            created_by,
        }: CreatePageAttribution,
    ) -> Result<PageAttribution> {
        let metadata_json = serde_json::to_value(&metadata)?;

        if let Some(model) =
            find_page_attribution(ctx, page_id, user_id, &metadata_json).await?
        {
            debug!(
                "Page attribution already exists for page {} user {} ({:?} @ {})",
                page_id, user_id, metadata.attribution_type, metadata.attribution_date,
            );

            return PageAttribution::try_from_model(model);
        }

        debug!(
            "Creating page attribution for page {} user {} ({:?} @ {})",
            page_id, user_id, metadata.attribution_type, metadata.attribution_date,
        );

        RelationType::PageAttribution
            .types()
            .check(RelationObjectType::Page, RelationObjectType::User);

        let model = relation::ActiveModel {
            relation_type: Set(str!(RelationType::PageAttribution.value())),
            dest_type: Set(RelationObjectType::Page),
            dest_id: Set(page_id),
            from_type: Set(RelationObjectType::User),
            from_id: Set(user_id),
            metadata: Set(metadata_json),
            created_by: Set(created_by),
            ..Default::default()
        };

        let model = model.insert(ctx.transaction()).await?;
        PageAttribution::try_from_model(model)
    }

    #[allow(dead_code)]
    pub async fn page_attribution_exists(
        ctx: &ServiceContext<'_>,
        page_id: i64,
        user_id: i64,
        metadata: &PageAttributionMetadata,
    ) -> Result<bool> {
        let metadata_json = serde_json::to_value(metadata)?;
        find_page_attribution(ctx, page_id, user_id, &metadata_json)
            .await
            .map(|relation| relation.is_some())
    }

    pub async fn get_page_attributions(
        ctx: &ServiceContext<'_>,
        GetPageAttributions { page_id }: GetPageAttributions,
    ) -> Result<Vec<PageAttribution>> {
        debug!("Getting page attributions for page {}", page_id);

        let txn = ctx.transaction();
        let models = Relation::find()
            .filter(
                Condition::all()
                    .add(relation::Column::RelationType.eq(
                        RelationType::PageAttribution.value(),
                    ))
                    .add(relation::Column::DestType.eq(RelationObjectType::Page))
                    .add(relation::Column::FromType.eq(RelationObjectType::User))
                    .add(relation::Column::DestId.eq(page_id))
                    .add(relation::Column::OverwrittenAt.is_null())
                    .add(relation::Column::DeletedAt.is_null()),
            )
            .order_by_asc(relation::Column::CreatedAt)
            .all(txn)
            .await?;

        models
            .into_iter()
            .map(PageAttribution::try_from_model)
            .collect()
    }

    pub async fn update_page_attribution(
        ctx: &ServiceContext<'_>,
        UpdatePageAttribution {
            relation_id,
            page_id,
            user_id,
            metadata,
            created_by,
        }: UpdatePageAttribution,
    ) -> Result<PageAttribution> {
        let metadata_json = serde_json::to_value(&metadata)?;

        let previous = match relation_id {
            Some(relation_id) => {
                let model = get_page_attribution_by_id(ctx, relation_id).await?;
                debug!(
                    "Updating existing page attribution relation {} for page {}",
                    relation_id, model.dest_id,
                );
                Some(model)
            }
            None => None,
        };

        if let Some(model) =
            find_page_attribution(ctx, page_id, user_id, &metadata_json).await?
        {
            if let Some(previous) = previous.as_ref() {
                if previous.relation_id != model.relation_id {
                    overwrite_page_attribution(ctx, previous.relation_id, created_by)
                        .await?;
                }
            }

            debug!(
                "Page attribution already exists for page {} user {} ({:?} @ {})",
                page_id, user_id, metadata.attribution_type, metadata.attribution_date,
            );

            return PageAttribution::try_from_model(model);
        }

        if let Some(previous) = previous {
            overwrite_page_attribution(ctx, previous.relation_id, created_by).await?;
        }

        Self::create_page_attribution(
            ctx,
            CreatePageAttribution {
                page_id,
                user_id,
                metadata,
                created_by,
            },
        )
        .await
    }

    pub async fn remove_page_attribution(
        ctx: &ServiceContext<'_>,
        RemovePageAttribution {
            relation_id,
            removed_by,
        }: RemovePageAttribution,
    ) -> Result<PageAttribution> {
        let existing = get_page_attribution_by_id(ctx, relation_id).await?;
        debug!(
            "Removing page attribution relation {} for page {}",
            relation_id, existing.dest_id,
        );

        let model = relation::ActiveModel {
            relation_id: Set(relation_id),
            deleted_by: Set(Some(removed_by)),
            deleted_at: Set(Some(now())),
            ..Default::default()
        };

        let model = model.update(ctx.transaction()).await?;
        PageAttribution::try_from_model(model)
    }
}

fn page_attribution_condition(
    page_id: i64,
    user_id: i64,
    metadata_json: &serde_json::Value,
) -> Condition {
    Condition::all()
        .add(relation::Column::RelationType.eq(
            RelationType::PageAttribution.value(),
        ))
        .add(relation::Column::DestType.eq(RelationObjectType::Page))
        .add(relation::Column::DestId.eq(page_id))
        .add(relation::Column::FromType.eq(RelationObjectType::User))
        .add(relation::Column::FromId.eq(user_id))
        .add(relation::Column::Metadata.eq(metadata_json.clone()))
        .add(relation::Column::OverwrittenAt.is_null())
        .add(relation::Column::DeletedAt.is_null())
}

fn convert_model(model: RelationModel) -> Result<PageAttribution> {
    assert_eq!(model.relation_type, RelationType::PageAttribution.value());
    assert_eq!(model.dest_type, RelationObjectType::Page);
    assert_eq!(model.from_type, RelationObjectType::User);

    let metadata: PageAttributionMetadata = serde_json::from_value(model.metadata)?;

    Ok(PageAttribution {
        relation_id: model.relation_id,
        page_id: model.dest_id,
        user_id: model.from_id,
        created_by: model.created_by,
        created_at: model.created_at,
        overwritten_by: model.overwritten_by,
        overwritten_at: model.overwritten_at,
        deleted_by: model.deleted_by,
        deleted_at: model.deleted_at,
        metadata,
    })
}

impl PageAttribution {
    fn try_from_model(model: RelationModel) -> Result<Self> {
        convert_model(model)
    }
}

async fn find_page_attribution(
    ctx: &ServiceContext<'_>,
    page_id: i64,
    user_id: i64,
    metadata_json: &serde_json::Value,
) -> Result<Option<RelationModel>> {
    let txn = ctx.transaction();
    Relation::find()
        .filter(page_attribution_condition(page_id, user_id, metadata_json))
        .one(txn)
        .await
        .map_err(Into::into)
}

async fn get_page_attribution_by_id(
    ctx: &ServiceContext<'_>,
    relation_id: i64,
) -> Result<RelationModel> {
    let txn = ctx.transaction();
    let model = Relation::find()
        .filter(
            Condition::all()
                .add(relation::Column::RelationId.eq(relation_id))
                .add(relation::Column::DestType.eq(RelationObjectType::Page))
                .add(relation::Column::FromType.eq(RelationObjectType::User))
                .add(relation::Column::RelationType.eq(
                    RelationType::PageAttribution.value(),
                ))
                .add(relation::Column::OverwrittenAt.is_null())
                .add(relation::Column::DeletedAt.is_null()),
        )
        .one(txn)
        .await?;

    match model {
        Some(model) => Ok(model),
        None => Err(Error::RelationNotFound),
    }
}

async fn overwrite_page_attribution(
    ctx: &ServiceContext<'_>,
    relation_id: i64,
    overwritten_by: i64,
) -> Result<()> {
    let model = relation::ActiveModel {
        relation_id: Set(relation_id),
        overwritten_by: Set(Some(overwritten_by)),
        overwritten_at: Set(Some(now())),
        ..Default::default()
    };

    model.update(ctx.transaction()).await?;
    Ok(())
}
