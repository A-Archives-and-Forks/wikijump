/*
 * services/relation/user_bot_owner.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2024 Wikijump Team
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
use crate::utils::trim_spaces_in_place;
use time::OffsetDateTime;

// External structures

#[derive(Debug, Copy, Clone)]
pub struct CreateSingleUserBotOwner<'a> {
    pub bot_user_id: i64,
    pub owner_user_id: i64,
    pub created_by: i64,
    pub metadata: &'a UserBotMetadata,
}

// Relation structures

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBotOwner {
    pub created_by: i64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub overwritten_by: Option<i64>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub overwritten_at: Option<OffsetDateTime>,

    pub bot_user_id: i64,
    pub owner_user_id: i64,

    #[serde(flatten)]
    pub metadata: UserBotMetadata,
    pub relation_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBotMetadata {
    pub description: String,
    pub approval_url: Option<String>,
}

impl_relation!(
    UserBotOwner,
    User,
    bot_user,
    User,
    owner_user,
    UserBotMetadata,
    NO_CREATE_IMPL_OR_STRUCT,
);

impl RelationService {
    pub fn normalize_user_bot_metadata(metadata: &mut UserBotMetadata) {
        if let Some(ref mut approval_url) = metadata.approval_url {
            trim_spaces_in_place(approval_url);

            if approval_url.is_empty() {
                debug!("Replacing empty approval URL with null");
                metadata.approval_url = None;
            }
        }
    }

    pub async fn create_user_bot_owner(
        ctx: &ServiceContext<'_>,
        CreateSingleUserBotOwner {
            bot_user_id,
            owner_user_id,
            created_by,
            metadata,
        }: CreateSingleUserBotOwner<'_>,
    ) -> Result<()> {
        // Cannot be the owner if the bot is blocked
        Self::check_user_block(ctx, bot_user_id, owner_user_id, "follow").await?;

        create_operation!(
            ctx,
            UserBotOwner,
            User,
            bot_user_id,
            User,
            owner_user_id,
            created_by,
            metadata,
        )
    }

    #[inline]
    pub async fn get_bots_owned_by_user(
        ctx: &ServiceContext<'_>,
        owner_user_id: i64,
    ) -> Result<Vec<UserBotOwner>> {
        let models = RelationService::get_user_bot_owner_entries(
            ctx,
            RelationObject::User(owner_user_id),
            RelationDirection::From,
        )
        .await?;

        models_to_owners(models)
    }

    #[inline]
    pub async fn get_owners_for_bot(
        ctx: &ServiceContext<'_>,
        bot_user_id: i64,
    ) -> Result<Vec<UserBotOwner>> {
        let models = RelationService::get_user_bot_owner_entries(
            ctx,
            RelationObject::User(bot_user_id),
            RelationDirection::Dest,
        )
        .await?;

        models_to_owners(models)
    }
}

fn models_to_owners(models: Vec<RelationModel>) -> Result<Vec<UserBotOwner>> {
    let mut owners = Vec::with_capacity(models.len());

    for model in models {
        let metadata: UserBotMetadata = serde_json::from_value(model.metadata)?;

        assert_eq!(model.relation_type, "bot-owner");
        assert_eq!(model.dest_type, RelationObjectType::User);
        assert_eq!(model.from_type, RelationObjectType::User);

        owners.push(UserBotOwner {
            relation_id: model.relation_id,
            created_by: model.created_by,
            created_at: model.created_at,
            overwritten_by: model.overwritten_by,
            overwritten_at: model.overwritten_at,
            bot_user_id: model.dest_id,
            owner_user_id: model.from_id,
            metadata,
        });
    }

    Ok(owners)
}
