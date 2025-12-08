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
use time::Date;

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

#[derive(Deserialize, Debug, Clone)]
pub struct CreatePageAttribution {
    pub page_id: i64,
    pub user_id: i64,
    pub metadata: PageAttributionMetadata,
    pub created_by: i64,
}

impl RelationService {
    /// Creates a page attribution relation without overwriting other attributions
    /// for the same page / user combination. This mirrors the unique constraint of
    /// (page_id, user_id, attribution_type, attribution_date) from the former
    /// dedicated table.
    #[allow(dead_code)] // TEMP
    pub async fn create_page_attribution(
        ctx: &ServiceContext<'_>,
        CreatePageAttribution {
            page_id,
            user_id,
            metadata,
            created_by,
        }: CreatePageAttribution,
    ) -> Result<()> {
        let txn = ctx.transaction();

        let metadata_json = serde_json::to_value(&metadata)?;
        let already_exists = Relation::find()
            .filter(
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
                    .add(relation::Column::DeletedAt.is_null()),
            )
            .one(txn)
            .await?
            .is_some();

        if already_exists {
            debug!(
                "Page attribution already exists for page {} user {} ({:?} @ {})",
                page_id, user_id, metadata.attribution_type, metadata.attribution_date,
            );
            return Ok(());
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

        model.insert(txn).await?;
        Ok(())
    }

    #[allow(dead_code)] // TEMP
    pub async fn page_attribution_exists(
        ctx: &ServiceContext<'_>,
        page_id: i64,
        user_id: i64,
        metadata: &PageAttributionMetadata,
    ) -> Result<bool> {
        let txn = ctx.transaction();
        let metadata_json = serde_json::to_value(metadata)?;
        let exists = Relation::find()
            .filter(
                Condition::all()
                    .add(relation::Column::RelationType.eq(
                        RelationType::PageAttribution.value(),
                    ))
                    .add(relation::Column::DestType.eq(RelationObjectType::Page))
                    .add(relation::Column::DestId.eq(page_id))
                    .add(relation::Column::FromType.eq(RelationObjectType::User))
                    .add(relation::Column::FromId.eq(user_id))
                    .add(relation::Column::Metadata.eq(metadata_json))
                    .add(relation::Column::OverwrittenAt.is_null())
                    .add(relation::Column::DeletedAt.is_null()),
            )
            .one(txn)
            .await?
            .is_some();

        Ok(exists)
    }
}
