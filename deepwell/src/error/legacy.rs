/*
 * error/legacy.rs
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

use crate::error::Error as NewError;
use crate::types::EnumConversionError;
use exn::Exn;
use filemagic::FileMagicError;
use jsonrpsee::types::error::ErrorObjectOwned;
use s3::error::S3Error;
use sea_orm::error::DbErr;
use thiserror::Error as ThisError;
use unic_langid::LanguageIdentifierError;

/// Wrapper error for possible failure modes from service methods.
#[derive(ThisError, Debug)]
pub enum OldError {
    // Error passed straight to ErrorObjectOwned without conversion
    #[error("{0}")]
    Raw(#[from] ErrorObjectOwned),

    // Error passed from updated code
    #[error("{0}")]
    Exn(Exn<NewError>),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Database(DbErr),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Redis Simple Message Queue (RSMQ) error: {0}")]
    Rsmq(#[from] rsmq_async::RsmqError),

    #[error("Magic library error: {0}")]
    Magic(#[from] FileMagicError),

    #[error("One-time password error: {0}")]
    Otp(#[from] rust_otp::Error),

    #[error("S3 service returned error: {0}")]
    S3Service(#[from] S3Error),

    #[error("Unspecified entity not found")]
    GeneralNotFound,

    #[error("Alias does not exist")]
    AliasNotFound,

    #[error("Relation value does not exist")]
    RelationNotFound,

    #[error("User does not exist")]
    UserNotFound,

    #[error("Site does not exist")]
    SiteNotFound,

    #[error("Page does not exist")]
    PageNotFound,

    #[error("Page revision does not exist")]
    PageRevisionNotFound,

    #[error("File does not exist")]
    FileNotFound,

    #[error("File revision does not exist")]
    FileRevisionNotFound,

    #[error("Filter does not exist")]
    FilterNotFound,

    #[error("Custom domain does not exist")]
    CustomDomainNotFound,

    #[error("Cannot perform, user already exists")]
    UserExists,

    #[error("Cannot perform, site already exists")]
    SiteExists,

    #[error("Cannot perform, file already exists")]
    FileExists,

    #[error("Cannot perform, filter already exists")]
    FilterExists,

    #[error("Cannot perform, custom domain already exists")]
    CustomDomainExists,

    #[error("Invalid session token, cannot be used for authentication")]
    InvalidSessionToken,

    #[error(
        "User ID {session_user_id} associated with session does not match active user ID {active_user_id}"
    )]
    SessionUserId {
        active_user_id: i64,
        session_user_id: i64,
    },

    #[error("The request is in some way malformed or incorrect")]
    BadRequest,

    #[error("Invalid enum serialization value")]
    InvalidEnumValue(#[from] EnumConversionError),

    #[error("User name is too short")]
    UserNameTooShort,

    #[error("Wrong user type for this operation")]
    UserWrongType,

    #[error("Site slug cannot be empty")]
    SiteSlugEmpty,

    #[error("Cannot hide the wikitext for the latest page revision")]
    CannotHideLatestRevision,

    #[error("Revision ID passed for this operation is not the latest")]
    NotLatestRevisionId,

    #[error("File name cannot be empty")]
    FileNameEmpty,

    #[error("File name too long")]
    FileNameTooLong { length: usize, maximum: usize },

    #[error("File name contains invalid characters (control chars or slashes)")]
    FileNameInvalidCharacters,

    #[error("File MIME type cannot be empty")]
    FileMimeEmpty,

    #[error("Cannot restore a non-deleted file")]
    FileNotDeleted,

    #[error("Invalid locale: {0}")]
    LocaleInvalid(#[from] LanguageIdentifierError),

    #[error("No locales were specified in the request")]
    NoLocalesSpecified,

    #[error("Cannot restore a non-deleted filter")]
    FilterNotDeleted,

    #[error("Custom domains may not be subdomains of the Wikijump main or file domains")]
    CustomDomainSubdomain,

    #[error("Cannot use custom domain, as it belongs to a different site")]
    CustomDomainWrongSite,

    #[error("Cannot perform this action because you are blocked by the user")]
    UserBlockedUser,

    #[error("Cannot perform this action because you are blocked by the site")]
    SiteBlockedUser,
}

// Error conversion implementations
//
// Required if the value doesn't implement StdError,
// or we want custom conversions.

impl From<DbErr> for OldError {
    fn from(error: DbErr) -> OldError {
        match error {
            DbErr::RecordNotFound(_) => OldError::GeneralNotFound,
            _ => OldError::Database(error),
        }
    }
}

// Temporary while we convert stuff to the new error type
impl From<Exn<NewError>> for OldError {
    #[inline]
    fn from(error: Exn<NewError>) -> OldError {
        OldError::Exn(error)
    }
}
