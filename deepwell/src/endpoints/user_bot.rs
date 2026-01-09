/*
 * endpoints/user_bot.rs
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
use crate::models::sea_orm_active_enums::UserType;
use crate::services::relation::{
    CreateSingleUserBotOwner, RelationService, RemoveUserBotOwner, UserBotMetadata,
    UserBotOwner,
};
use crate::services::user::{CreateUser, CreateUserOutput, GetUser, UpdateUserBody};
use crate::types::{Maybe, Reference};
use std::net::IpAddr;

// Structs

/// Input structure for creating a new bot user.
#[derive(Deserialize, Debug, Clone)]
pub struct CreateBotUser {
    pub name: String,
    pub email: String,
    pub locales: Vec<String>,
    pub purpose: String,
    pub owners: Vec<i64>,
    pub authorization_token: String,
    pub created_by: i64,

    #[serde(flatten)]
    pub metadata: UserBotMetadata,

    #[serde(default)]
    pub bypass_filter: bool,

    #[serde(default)]
    pub bypass_email_verification: bool,

    pub ip_address: IpAddr,
}

/// Input structure for adding new bot owners.
#[derive(Deserialize, Debug, Clone)]
pub struct CreateBotUserOwners {
    pub bot_user_id: i64,
    pub owners: Vec<i64>,

    #[serde(flatten)]
    pub metadata: UserBotMetadata,
    pub created_by: i64,
}

// Endpoints

pub async fn bot_user_create(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<CreateUserOutput> {
    let CreateBotUser {
        name,
        email,
        locales,
        purpose,
        mut metadata,
        owners,
        created_by,
        authorization_token,
        bypass_filter,
        bypass_email_verification,
        ip_address,
    } = params.parse()?;

    info!("Creating new bot user with name '{name}'");

    // TODO verify auth token
    // TODO add authorization token service
    // format: [flag]-[uuid]
    //         for instance B-1F305167-AE64-4486-809A-09D14659AB4A
    //
    //         B: create a bot user
    //         S: create a site
    let _ = authorization_token;

    // Create bot user
    let output = UserService::create(
        ctx,
        CreateUser {
            user_type: UserType::Bot,
            name,
            email,
            locales,
            password: String::new(), // TODO configure user-bot password
            bypass_filter,
            bypass_email_verification,
            ip_address,
        },
    )
    .await?;

    // Set description, get bot user
    let bot_user_id = output.user_id;
    let bot_user = UserService::update(
        ctx,
        Reference::Id(bot_user_id),
        ip_address,
        UpdateUserBody {
            biography: Maybe::Set(Some(purpose)),
            ..Default::default()
        },
    )
    .await?;

    // Normalize metadata field
    RelationService::normalize_user_bot_metadata(&mut metadata);

    // Add bot owners
    debug!(
        "Adding human owners for bot user '{}' ({})",
        bot_user.name, bot_user_id,
    );
    for owner_user_id in owners {
        let owner_user = UserService::get(ctx, Reference::Id(owner_user_id)).await?;
        debug!(
            "Adding human user '{}' (ID {}) as bot owner",
            owner_user.name, owner_user_id,
        );
        RelationService::create_user_bot_owner(
            ctx,
            CreateSingleUserBotOwner {
                bot_user: &bot_user,
                owner_user: &owner_user,
                created_by,
                metadata: &metadata,
            },
        )
        .await?;
    }

    Ok(output)
}

pub async fn bot_user_get_owners(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<Option<Vec<UserBotOwner>>> {
    let GetUser { user: reference } = params.parse()?;
    info!("Getting bot user {reference:?}");
    match UserService::get_optional(ctx, reference).await? {
        None => Ok(None),
        Some(bot_user) => {
            if bot_user.user_type != UserType::Bot {
                error!(
                    "Tried to get owners for non-bot user: '{}' (type {:?})",
                    bot_user.name, bot_user.user_type,
                );
                return Err(ServiceError::UserWrongType);
            }

            let owners =
                RelationService::get_owners_for_bot(ctx, bot_user.user_id).await?;

            Ok(Some(owners))
        }
    }
}

pub async fn bot_user_get_bots(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<Vec<UserBotOwner>> {
    let GetUser { user: reference } = params.parse()?;
    info!("Getting bot users owned by user {reference:?}");

    let owner_user = UserService::get(ctx, reference).await?;
    if owner_user.user_type != UserType::Regular {
        error!(
            "Tried to get bots for non-regular user: '{}' (type {:?})",
            owner_user.name, owner_user.user_type,
        );
        return Err(ServiceError::UserWrongType);
    }

    let owners = RelationService::get_bots_owned_by_user(ctx, owner_user.user_id).await?;
    Ok(owners)
}

pub async fn bot_user_owner_set(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<()> {
    let input: CreateBotUserOwners = params.parse()?;
    info!(
        "Adding or updating bot owners for {} ({} new owners)",
        input.bot_user_id,
        input.owners.len(),
    );

    let bot_user = UserService::get(ctx, Reference::Id(input.bot_user_id)).await?;
    for owner_user_id in input.owners {
        let owner_user = UserService::get(ctx, Reference::Id(owner_user_id)).await?;
        RelationService::create_user_bot_owner(
            ctx,
            CreateSingleUserBotOwner {
                bot_user: &bot_user,
                owner_user: &owner_user,
                created_by: input.created_by,
                metadata: &input.metadata,
            },
        )
        .await?;
    }

    Ok(())
}

pub async fn bot_user_owner_remove(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<()> {
    let input: RemoveUserBotOwner = params.parse()?;
    info!(
        "Remove bot owner ({} <- {})",
        input.bot_user, input.owner_user
    );
    RelationService::remove_user_bot_owner(ctx, input).await?;
    Ok(())
}
