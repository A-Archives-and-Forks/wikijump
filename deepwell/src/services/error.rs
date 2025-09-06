/*
 * services/error.rs
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

use crate::hash::{blob_hash_to_hex, BlobHash};
use filemagic::FileMagicError;
use jsonrpsee::types::error::ErrorObjectOwned;
use reqwest::Error as ReqwestError;
use s3::error::S3Error;
use sea_orm::{error::DbErr, TransactionError};
use thiserror::Error as ThisError;
use unic_langid::LanguageIdentifierError;

pub use std::error::Error as StdError;

pub type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = StdResult<T, Error>;

/// Wrapper error for possible failure modes from service methods.
#[derive(ThisError, Debug)]
pub enum Error {
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
    AuthenticationBackend(Box<Error>),

    #[error("Invalid session token, cannot be used for authentication")]
    InvalidSessionToken,

    #[error("User ID {session_user_id} associated with session does not match active user ID {active_user_id}")]
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

impl Error {
    /// Returns the code associated with this error.
    ///
    /// The JSON-RPC spec has each unique error case return its own integer error code.
    /// Some very negative codes are reserved for RPC internals, so we will only output
    /// positive values.
    ///
    /// Sort of similar to HTTP status codes, we are also dividing them into groups based
    /// generally on the kind of error it is.
    ///
    /// When an error case is removed, then its number should generally not be reused,
    /// just use the next available value in line. Also be sure to update framerail
    /// accordingly when error codes are added or removed.
    pub fn code(&self) -> i32 {
        match self {
            //
            // 1000 -- General Process Handling
            //

            // 1000 - Miscellaneous / Technical
            Error::Raw(_) => 1000,
            Error::AuthenticationBackend(_) => 1001,

            // 1100 - Rust Errors
            Error::Serde(_) => 1100,
            Error::Database(_) => 1101,
            Error::Cryptography(_) => 1102,
            Error::Magic(_) => 1103,
            Error::Otp(_) => 1104,
            Error::Redis(_) => 1105,
            Error::Rsmq(_) => 1106,

            // 1200 - Service Errors
            Error::RateLimited => 1200,
            Error::WebRequest(_) => 1201,
            Error::RenderTimeout => 1202,
            Error::EmailVerification(_) => 1203,
            Error::S3Service(_) => 1204,
            Error::S3Response => 1205,

            //
            // 2000 -- Data Consistency
            //

            // 2000 - Not Found
            Error::GeneralNotFound => 2000,
            Error::AliasNotFound => 2001,
            Error::RelationNotFound => 2002,
            Error::UserNotFound => 2003,
            Error::SiteNotFound => 2004,
            Error::PageNotFound => 2005,
            Error::PageCategoryNotFound => 2006,
            Error::PageParentNotFound => 2007,
            Error::PageRevisionNotFound => 2008,
            Error::FileNotFound => 2009,
            Error::FileRevisionNotFound => 2010,
            Error::VoteNotFound => 2011,
            Error::FilterNotFound => 2012,
            Error::CustomDomainNotFound => 2013,
            Error::MessageNotFound => 2014,
            Error::MessageDraftNotFound => 2015,
            Error::BlobNotFound => 2016,
            Error::TextNotFound => 2017,

            // 2100 - Already Exists
            Error::UserExists => 2100,
            Error::UserMfaExists => 2101,
            Error::SiteExists => 2102,
            Error::PageExists => 2103,
            Error::PageSlugExists => 2104,
            Error::PageParentExists => 2105,
            Error::FileExists => 2106,
            Error::FilterExists => 2107,
            Error::CustomDomainExists => 2108,

            //
            // 3000 -- Client / Protocol Errors
            //

            // 3000 - Authentication
            Error::InvalidAuthentication => 3000,
            Error::InvalidSessionToken => 3001,
            Error::SessionUserId { .. } => 3002,
            Error::EmptyPassword => 3003,

            // 3100 - Permission
            // TODO

            //
            // 4000, 5000, 6000 -- Client / Request Errors
            //

            //
            // 4000 -- Client / Request Errors - Core Data Objects
            //

            // 4000 - General
            //
            // Some of these requests are pretty general, unless it is a rare edge case,
            // consider adding a new error case when code to handle new fail states are
            // introduced.
            Error::BadRequest => 4000,
            Error::InvalidEnumValue => 4001,

            // 4100 - User
            Error::UserNameTooShort => 4100,
            Error::UserSlugEmpty => 4101,
            Error::UserEmailEmpty => 4102,
            Error::UserWrongType => 4103,
            Error::InsufficientNameChanges => 4104,
            Error::InvalidEmail => 4105,
            Error::DisallowedEmail => 4106,

            // 4200 - Site
            Error::SiteSlugEmpty => 4200,

            // 4300 - Page
            Error::PageSlugEmpty => 4300,
            Error::PageNotDeleted => 4301,
            Error::CannotHideLatestRevision => 4302,
            Error::NotLatestRevisionId => 4303,

            // 4400 - File
            Error::FileNameEmpty => 4400,
            Error::FileNameTooLong { .. } => 4401,
            Error::FileNameInvalidCharacters => 4402,
            Error::FileMimeEmpty => 4403,
            Error::FileNotDeleted => 4404,

            //
            // 5000 -- Client / Request Errors - Ancillary Data Objects
            //

            // 5000 - Locale
            Error::LocaleInvalid(_) => 5000,
            Error::LocaleMissing => 5001,
            Error::LocaleMessageMissing => 5002,
            Error::LocaleMessageValueMissing => 5003,
            Error::LocaleMessageAttributeMissing => 5004,
            Error::NoLocalesSpecified => 5005,

            // 5100 - Filter
            Error::FilterViolation => 5100,
            Error::FilterRegexInvalid(_) => 5101,
            Error::FilterNotDeleted => 5102,

            // 5200 - Blob
            Error::BlobNotUploaded => 5200,
            Error::BlobWrongUser => 5201,
            Error::BlobTooBig => 5202,
            Error::BlobSizeMismatch { .. } => 5204,
            Error::BlobBlacklisted(_) => 5205,
            Error::BlobCannotBlacklistExisting => 5206,

            // 5300 - Message
            Error::MessageSubjectEmpty => 5300,
            Error::MessageSubjectTooLong => 5301,
            Error::MessageBodyEmpty => 5302,
            Error::MessageBodyTooLong => 5303,
            Error::MessageNoRecipients => 5304,
            Error::MessageTooManyRecipients => 5305,

            // 5400 - Domains
            Error::CustomDomainWrongSite => 5400,
            Error::CustomDomainSubdomain => 5401,

            //
            // 6000 -- Client / Request Errors - Composite Data
            //

            // 6000 - Relations
            Error::SiteBlockedUser => 6000,
            Error::UserBlockedUser => 6001,
        }
    }

    /// Emit partial structured error data.
    ///
    /// Meant to be better than nothing and simply `Debug` but also not
    /// as much boilerplate as manually implementing `Serialize` on everything.
    /// This unwraps common cases and makes things generally clearer.
    fn data(&self) -> serde_json::Value {
        use serde_json::json;

        match self {
            // Message already has all the data
            Error::Raw(_) => json!(null),

            // Unwrap self-error
            Error::AuthenticationBackend(error) => error.data(),

            // Emit as structure
            Error::SessionUserId {
                active_user_id,
                session_user_id,
            } => json!({
                "active_user_id": active_user_id,
                "session_user_id": session_user_id,
            }),
            Error::BlobSizeMismatch { expected, actual } => json!({
                "expected": expected,
                "actual": actual,
            }),
            Error::FileNameTooLong { length, maximum } => json!({
                "length": length,
                "maximum": maximum,
            }),

            // Emit as-is
            Error::EmailVerification(value) => json!(value),

            // Emit as a Debug string
            Error::Cryptography(value) => json!(format!("{value:?}")),
            Error::Database(value) => json!(format!("{value:?}")),
            Error::LocaleInvalid(value) => json!(format!("{value:?}")),
            Error::Magic(value) => json!(format!("{value:?}")),
            Error::Otp(value) => json!(format!("{value:?}")),
            Error::Serde(value) => json!(format!("{value:?}")),
            Error::S3Service(value) => json!(format!("{value:?}")),
            Error::WebRequest(value) => json!(format!("{value:?}")),
            Error::FilterRegexInvalid(value) => json!(format!("{value:?}")),

            // Emit as hexadecimal bytes
            Error::BlobBlacklisted(bytes) => json!(*blob_hash_to_hex(bytes)),

            // Other cases are null enums or the values are ignored
            _ => json!(null),
        }
    }
}

// Error conversion implementations
//
// Required if the value doesn't implement StdError,
// or we want custom conversions.

impl From<argon2::password_hash::Error> for Error {
    #[inline]
    fn from(error: argon2::password_hash::Error) -> Error {
        match error {
            // Password is invalid, expected error
            argon2::password_hash::Error::Password => Error::InvalidAuthentication,

            // Problem with the password hashing process
            _ => Error::Cryptography(error),
        }
    }
}

impl From<DbErr> for Error {
    fn from(error: DbErr) -> Error {
        match error {
            DbErr::RecordNotFound(_) => Error::GeneralNotFound,
            _ => Error::Database(error),
        }
    }
}

// End-conversion for methods
//
// This is used to convert our ServiceError type into the RPC error type.

impl From<Error> for ErrorObjectOwned {
    fn from(error: Error) -> ErrorObjectOwned {
        // Return a raw error as-is
        if let Error::Raw(error) = error {
            return error;
        }

        // Build error object
        let error_code = error.code();
        let message = str!(error);
        let data = error.data();
        ErrorObjectOwned::owned(error_code, message, Some(data))
    }
}

// Helper function for unwrapping two layers of third party crate error wrapper types.

pub fn into_rpc_error(error: TransactionError<ErrorObjectOwned>) -> ErrorObjectOwned {
    match error {
        TransactionError::Connection(error) => Error::Database(error).into(),
        TransactionError::Transaction(error) => error,
    }
}
