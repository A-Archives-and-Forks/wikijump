/*
 * error/old.rs
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

use crate::hash::BlobHash;
use filemagic::FileMagicError;
use jsonrpsee::types::error::ErrorObjectOwned;
use reqwest::Error as ReqwestError;
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

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Database(DbErr),

    #[error("Cryptography error: {0}")]
    Cryptography(argon2::password_hash::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Redis Simple Message Queue (RSMQ) error: {0}")]
    Rsmq(#[from] rsmq_async::RsmqError),

    #[error("Magic library error: {0}")]
    Magic(#[from] FileMagicError),

    #[error("One-time password error: {0}")]
    Otp(#[from] rust_otp::Error),

    #[error("The rate limit for an external API has been reached")]
    RateLimited,

    #[error("Web request error: {0}")]
    WebRequest(#[from] ReqwestError),

    #[error("Attempting to perform a wikitext parse and render has timed out")]
    RenderTimeout,

    #[error("Email verification error: {}", .0.as_ref().unwrap_or(&str!("<unspecified>")))]
    EmailVerification(Option<String>),

    #[error("S3 service returned error: {0}")]
    S3Service(#[from] S3Error),

    #[error("S3 service failed to respond properly")]
    S3Response,

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

    #[error("Page category does not exist")]
    PageCategoryNotFound,

    #[error("Page parent does not exist")]
    PageParentNotFound,

    #[error("Page revision does not exist")]
    PageRevisionNotFound,

    #[error("File does not exist")]
    FileNotFound,

    #[error("File revision does not exist")]
    FileRevisionNotFound,

    #[error("Vote does not exist")]
    VoteNotFound,

    #[error("Filter does not exist")]
    FilterNotFound,

    #[error("Custom domain does not exist")]
    CustomDomainNotFound,

    #[error("Message does not exist")]
    MessageNotFound,

    #[error("Message draft does not exist")]
    MessageDraftNotFound,

    #[error("Blob item does not exist")]
    BlobNotFound,

    #[error("Text item does not exist")]
    TextNotFound,

    #[error("Cannot perform, user already exists")]
    UserExists,

    #[error("Cannot set up user MFA, already set up")]
    UserMfaExists,

    #[error("Cannot perform, site already exists")]
    SiteExists,

    #[error("Cannot perform, page already exists")]
    PageExists,

    #[error("Cannot perform, page slug already exists")]
    PageSlugExists,

    #[error("Cannot perform, page parent already exists")]
    PageParentExists,

    #[error("Cannot perform, file already exists")]
    FileExists,

    #[error("Cannot perform, filter already exists")]
    FilterExists,

    #[error("Cannot perform, custom domain already exists")]
    CustomDomainExists,

    #[error("Invalid username, password, or TOTP code")]
    InvalidAuthentication,

    #[error("Backend error while trying to authenticate")]
    AuthenticationBackend(Box<OldError>),

    #[error("Invalid session token, cannot be used for authentication")]
    InvalidSessionToken,

    #[error(
        "User ID {session_user_id} associated with session does not match active user ID {active_user_id}"
    )]
    SessionUserId {
        active_user_id: i64,
        session_user_id: i64,
    },

    #[error("A password is required")]
    EmptyPassword,

    #[error("The request is in some way malformed or incorrect")]
    BadRequest,

    #[error("Invalid enum serialization value")]
    InvalidEnumValue,

    #[error("User name is too short")]
    UserNameTooShort,

    #[error("User slug cannot be empty")]
    UserSlugEmpty,

    #[error("User email cannot be empty")]
    UserEmailEmpty,

    #[error("Wrong user type for this operation")]
    UserWrongType,

    #[error("The user cannot rename as they do not have enough name change tokens")]
    InsufficientNameChanges,

    #[error("The user's email is disallowed")]
    DisallowedEmail,

    #[error("The user's email is invalid")]
    InvalidEmail,

    #[error("Site slug cannot be empty")]
    SiteSlugEmpty,

    #[error("Page slug cannot be empty")]
    PageSlugEmpty,

    #[error("Cannot restore a non-deleted page")]
    PageNotDeleted,

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

    #[error("No messages are available for this locale")]
    LocaleMissing,

    #[error("Message key not found for this locale")]
    LocaleMessageMissing,

    #[error("Message key was found, but has no value")]
    LocaleMessageValueMissing,

    #[error("Message key was found, but does not have this attribute")]
    LocaleMessageAttributeMissing,

    #[error("No locales were specified in the request")]
    NoLocalesSpecified,

    #[error("The request violates a configured content filter")]
    FilterViolation,

    #[error("The regular expression found in the database is invalid")]
    FilterRegexInvalid(regex::Error),

    #[error("Cannot restore a non-deleted filter")]
    FilterNotDeleted,

    #[error("Blob not uploaded")]
    BlobNotUploaded,

    #[error("Cannot use blob uploaded by different user")]
    BlobWrongUser,

    #[error("Uploaded blob is too big for this operation")]
    BlobTooBig,

    #[error("Uploaded blob does not match expected length")]
    BlobSizeMismatch { expected: usize, actual: usize },

    #[error("Uploaded blob content is blacklisted")]
    BlobBlacklisted(BlobHash),

    #[error(
        "Cannot blacklist a blob which is already in use, you must do a hard deletion"
    )]
    BlobCannotBlacklistExisting,

    #[error("Message subject cannot be empty")]
    MessageSubjectEmpty,

    #[error("Message subject too long")]
    MessageSubjectTooLong,

    #[error("Message body cannot be empty")]
    MessageBodyEmpty,

    #[error("Message body too long")]
    MessageBodyTooLong,

    #[error("Message cannot have no recipients")]
    MessageNoRecipients,

    #[error("Message has too many recipients")]
    MessageTooManyRecipients,

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

impl From<argon2::password_hash::Error> for OldError {
    #[inline]
    fn from(error: argon2::password_hash::Error) -> OldError {
        match error {
            // Password is invalid, expected error
            argon2::password_hash::Error::Password => OldError::InvalidAuthentication,

            // Problem with the password hashing process
            _ => OldError::Cryptography(error),
        }
    }
}

impl From<DbErr> for OldError {
    fn from(error: DbErr) -> OldError {
        match error {
            DbErr::RecordNotFound(_) => OldError::GeneralNotFound,
            _ => OldError::Database(error),
        }
    }
}
