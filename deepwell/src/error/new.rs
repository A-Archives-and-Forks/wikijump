/*
 * error/new.rs
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
use exn::Exn;
use jsonrpsee::types::error::ErrorObjectOwned;
use serde_json::Value as JsonValue;
use std::error::Error as StdError;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub error_type: ErrorType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    /// Application failed to start.
    ApplicationStart,

    /// A request returned an error.
    Request,

    /// A database transaction was aborted due to an error.
    DatabaseTransaction,

    /// Seeding the database failed.
    DatabaseSeeder,

    /// Failed to set up server internal state.
    ServerSetup,

    /// Failed to set up the database connection.
    DatabaseSetup,

    /// Failed to set up the Redis connection.
    RedisSetup,

    /// An external API has ratelimited us.
    RateLimited,

    /// Attempting to perform a wikitext parse and render has timed out.
    RenderTimeout,

    /// Unable to perform email verification.
    EmailVerification,

    /// Unspecified entity not found.
    GeneralNotFound,

    /// Alias does not exist.
    AliasNotFound,

    /// Relation value does not exist.
    RelationNotFound,

    /// User does not exist.
    UserNotFound,

    /// Site does not exist.
    SiteNotFound,

    /// Page does not exist.
    PageNotFound,

    /// Page category does not exist.
    PageCategoryNotFound,

    /// Page parent does not exist.
    PageParentNotFound,

    /// Page revision does not exist.
    PageRevisionNotFound,

    /// File does not exist.
    FileNotFound,

    /// File revision does not exist.
    FileRevisionNotFound,

    /// Vote does not exist.
    VoteNotFound,

    /// Filter does not exist.
    FilterNotFound,

    /// Custom domain does not exist.
    CustomDomainNotFound,

    /// Message does not exist.
    MessageNotFound,

    /// Message draft does not exist.
    MessageDraftNotFound,

    /// Blob item does not exist.
    BlobNotFound,

    /// Text item does not exist.
    TextNotFound,

    /// Cannot perform, user already exists.
    UserExists,

    /// Cannot set up user MFA, already set up.
    UserMfaExists,

    /// Cannot perform, site already exists.
    SiteExists,

    /// Cannot perform, page already exists.
    PageExists,

    /// Cannot perform, page slug already exists.
    PageSlugExists,

    /// Cannot perform, page parent already exists.
    PageParentExists,

    /// Cannot perform, file already exists.
    FileExists,

    /// Cannot perform, filter already exists.
    FilterExists,

    /// Cannot perform, custom domain already exists.
    CustomDomainExists,

    /// Invalid username, password, or TOTP code.
    InvalidAuthentication,

    /// A server error occurred while attempting to authenticate.
    ///
    /// High-level wrapper error for any other server error occurring
    /// while attempting to perform authentication, to avoid leaking
    /// server state.
    AuthenticationBackend,

    /// Invalid session token, cannot be used for authentication.
    InvalidSessionToken,

    /// User associated with the session does not match the active user.
    SessionUserId {
        active_user_id: i64,
        session_user_id: i64,
    },

    /// A password is required, but was not provided.
    EmptyPassword,

    /// The request is in some way malformed or incorrect.
    BadRequest,

    /// Invalid enum serialization value.
    InvalidEnumValue,

    /// User name is too short.
    UserNameTooShort,

    /// User slug cannot be empty.
    UserSlugEmpty,

    /// User email cannot be empty.
    UserEmailEmpty,

    /// Wrong user type for this operation.
    UserWrongType,

    /// The user cannot rename as they do not have enough name change tokens.
    InsufficientNameChanges,

    /// The user's email is disallowed.
    DisallowedEmail,

    /// The user's email is invalid.
    InvalidEmail,

    /// Site slug cannot be empty.
    SiteSlugEmpty,

    /// Page slug cannot be empty.
    PageSlugEmpty,

    /// Cannot restore a non-deleted page.
    PageNotDeleted,

    /// Cannot hide the wikitext for the latest page revision.
    CannotHideLatestRevision,

    /// Revision ID passed for this operation is not the latest.
    NotLatestRevisionId,

    /// File name cannot be empty.
    FileNameEmpty,

    /// File name too long.
    FileNameTooLong { length: usize, maximum: usize },

    /// File name contains invalid characters (control chars or slashes).
    FileNameInvalidCharacters,

    /// File MIME type cannot be empty.
    FileMimeEmpty,

    /// Cannot restore a non-deleted file.
    FileNotDeleted,

    /// Invalid locale name.
    LocaleInvalid { locale: String },

    /// No messages are available for this locale.
    LocaleMissing { locale: String },

    /// Message key not found for this locale.
    LocaleMessageMissing { message_key: String },

    /// Message key was found, but has no value.
    LocaleMessageValueMissing { message_key: String },

    /// Message key was found, but does not have this attribute.
    LocaleMessageAttributeMissing {
        message_key: String,
        attribute: String,
    },

    /// No locales were specified in the request.
    NoLocalesSpecified,

    /// The request violates a configured content filter.
    FilterViolation,

    /// Cannot restore a non-deleted filter.
    FilterNotDeleted,

    /// Blob not uploaded.
    BlobNotUploaded,

    /// Cannot use blob uploaded by different user.
    BlobWrongUser,

    /// Uploaded blob is too big for this operation.
    BlobTooBig,

    /// Uploaded blob does not match expected length.
    BlobSizeMismatch { expected: usize, actual: usize },

    /// Uploaded blob content is blacklisted.
    BlobBlacklisted(BlobHash),

    /// "Cannot blacklist a blob which is already in use, you must do a hard deletion".
    BlobCannotBlacklistExisting,

    /// Message subject cannot be empty.
    MessageSubjectEmpty,

    /// Message subject too long.
    MessageSubjectTooLong,

    /// Message body cannot be empty.
    MessageBodyEmpty,

    /// Message body too long.
    MessageBodyTooLong,

    /// Message cannot have no recipients.
    MessageNoRecipients,

    /// Message has too many recipients.
    MessageTooManyRecipients,

    /// Custom domains may not be subdomains of the Wikijump main or file domains.
    CustomDomainSubdomain,

    /// Cannot use custom domain, as it belongs to a different site.
    CustomDomainWrongSite,

    /// Cannot perform this action because you are blocked by the user.
    UserBlockedUser,

    /// Cannot perform this action because you are blocked by the site.
    SiteBlockedUser,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:04}] {}", self.code(), self.message)
    }
}

impl Error {
    #[inline]
    pub fn new<S: Into<String>>(message: S, error_type: ErrorType) -> Self {
        Error {
            message: message.into(),
            error_type,
        }
    }

    /// Returns a unique integer code for this type of error.
    ///
    /// See `ErrorType::code()` for details.
    #[inline]
    pub fn code(&self) -> i32 {
        self.error_type.code()
    }

    /// Returns auxiliary data for this error.
    ///
    /// See `ErrorType::data()` for details.
    #[inline]
    pub fn data(&self) -> JsonValue {
        self.error_type.data()
    }
}

impl ErrorType {
    /// Returns a unique integer code for this type of error.
    ///
    /// Errors are divided into groups:
    /// * 1000 - High-level
    ///   * 1000 - General
    ///   * 1100 - Intermediate Setup
    /// * 2000 - Data-consistency
    ///   * 2000 - Not Found
    ///   * 2100 - Already Exists
    /// * 3000 - Client / Protocol Errors
    ///   * 3000 - Authentication
    ///   * 3100 - Permissions
    ///   * 3200 - Server-side
    /// * 4000 - Client / Request Errors / Core Data Objects
    ///   * 4000 - General
    ///   * 4100 - User
    ///   * 4200 - Site
    ///   * 4300 - Page
    ///   * 4400 - File
    /// * 5000 - Client / Request Errors / Ancillary Data Objects
    ///   * 5000 - Locale
    ///   * 5100 - Filter
    ///   * 5200 - Blob
    ///   * 5300 - Message
    ///   * 5400 - Domains
    /// * 6000 - Client / Request Errors / Composite Data
    ///   * 6000 - Relations
    pub fn code(&self) -> i32 {
        match self {
            //
            // 1000 -- High-Level
            //

            // 1000 - General
            ErrorType::ApplicationStart => 1000,
            ErrorType::Request => 1001,
            ErrorType::DatabaseTransaction => 1002,
            ErrorType::DatabaseSeeder => 1003,

            // 1100 - Intermediate Setup
            ErrorType::ServerSetup => 1100,
            ErrorType::DatabaseSetup => 1101,
            ErrorType::RedisSetup => 1101,

            //
            // 2000 -- Data Consistency
            //

            // 2000 - Not Found
            ErrorType::GeneralNotFound => 2000,
            ErrorType::AliasNotFound => 2001,
            ErrorType::RelationNotFound => 2002,
            ErrorType::UserNotFound => 2003,
            ErrorType::SiteNotFound => 2004,
            ErrorType::PageNotFound => 2005,
            ErrorType::PageCategoryNotFound => 2006,
            ErrorType::PageParentNotFound => 2007,
            ErrorType::PageRevisionNotFound => 2008,
            ErrorType::FileNotFound => 2009,
            ErrorType::FileRevisionNotFound => 2010,
            ErrorType::VoteNotFound => 2011,
            ErrorType::FilterNotFound => 2012,
            ErrorType::CustomDomainNotFound => 2013,
            ErrorType::MessageNotFound => 2014,
            ErrorType::MessageDraftNotFound => 2015,
            ErrorType::BlobNotFound => 2016,
            ErrorType::TextNotFound => 2017,

            // 2100 - Already Exists
            ErrorType::UserExists => 2100,
            ErrorType::UserMfaExists => 2101,
            ErrorType::SiteExists => 2102,
            ErrorType::PageExists => 2103,
            ErrorType::PageSlugExists => 2104,
            ErrorType::PageParentExists => 2105,
            ErrorType::FileExists => 2106,
            ErrorType::FilterExists => 2107,
            ErrorType::CustomDomainExists => 2108,

            //
            // 3000 -- Client / Protocol Errors
            //

            // 3000 - Authentication
            ErrorType::InvalidAuthentication => 3000,
            ErrorType::InvalidSessionToken => 3001,
            ErrorType::SessionUserId { .. } => 3002,
            ErrorType::EmptyPassword => 3003,

            // 3100 - Permissions
            // TODO

            // 3200 - Server-side
            ErrorType::AuthenticationBackend => 3200,
            ErrorType::RenderTimeout => 3201,
            ErrorType::RateLimited => 3202,
            ErrorType::EmailVerification => 3203,

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
            ErrorType::BadRequest => 4000,
            ErrorType::InvalidEnumValue => 4001,

            // 4100 - User
            ErrorType::UserNameTooShort => 4100,
            ErrorType::UserSlugEmpty => 4101,
            ErrorType::UserEmailEmpty => 4102,
            ErrorType::UserWrongType => 4103,
            ErrorType::InsufficientNameChanges => 4104,
            ErrorType::InvalidEmail => 4105,
            ErrorType::DisallowedEmail => 4106,

            // 4200 - Site
            ErrorType::SiteSlugEmpty => 4200,

            // 4300 - Page
            ErrorType::PageSlugEmpty => 4300,
            ErrorType::PageNotDeleted => 4301,
            ErrorType::CannotHideLatestRevision => 4302,
            ErrorType::NotLatestRevisionId => 4303,

            // 4400 - File
            ErrorType::FileNameEmpty => 4400,
            ErrorType::FileNameTooLong { .. } => 4401,
            ErrorType::FileNameInvalidCharacters => 4402,
            ErrorType::FileMimeEmpty => 4403,
            ErrorType::FileNotDeleted => 4404,

            //
            // 5000 -- Client / Request Errors - Ancillary Data Objects
            //

            // 5000 - Locale
            ErrorType::LocaleInvalid { .. } => 5000,
            ErrorType::LocaleMissing { .. } => 5001,
            ErrorType::LocaleMessageMissing { .. } => 5002,
            ErrorType::LocaleMessageValueMissing { .. } => 5003,
            ErrorType::LocaleMessageAttributeMissing { .. } => 5004,
            ErrorType::NoLocalesSpecified => 5005,

            // 5100 - Filter
            ErrorType::FilterViolation => 5100,
            ErrorType::FilterNotDeleted => 5102,

            // 5200 - Blob
            ErrorType::BlobNotUploaded => 5200,
            ErrorType::BlobWrongUser => 5201,
            ErrorType::BlobTooBig => 5202,
            ErrorType::BlobSizeMismatch { .. } => 5204,
            ErrorType::BlobBlacklisted(_) => 5205,
            ErrorType::BlobCannotBlacklistExisting => 5206,

            // 5300 - Message
            ErrorType::MessageSubjectEmpty => 5300,
            ErrorType::MessageSubjectTooLong => 5301,
            ErrorType::MessageBodyEmpty => 5302,
            ErrorType::MessageBodyTooLong => 5303,
            ErrorType::MessageNoRecipients => 5304,
            ErrorType::MessageTooManyRecipients => 5305,

            // 5400 - Domains
            ErrorType::CustomDomainWrongSite => 5400,
            ErrorType::CustomDomainSubdomain => 5401,

            //
            // 6000 -- Client / Request Errors - Composite Data
            //

            // 6000 - Relations
            ErrorType::SiteBlockedUser => 6000,
            ErrorType::UserBlockedUser => 6001,
        }
    }

    /// Returns auxiliary data for this error.
    ///
    /// In effect, this serializes any contents of this error.
    /// For instance, if it refers to a particular user ID
    /// which caused an issue then this value would be
    /// returned in the JSON output.
    pub fn data(&self) -> JsonValue {
        use crate::hash::blob_hash_to_hex;
        use serde_json::json;

        match self {
            ErrorType::SessionUserId {
                active_user_id,
                session_user_id,
            } => json!({
                "active_user_id": active_user_id,
                "session_user_id": session_user_id,
            }),
            ErrorType::BlobSizeMismatch { expected, actual } => json!({
                "expected": expected,
                "actual": actual,
            }),
            ErrorType::FileNameTooLong { length, maximum } => json!({
                "length": length,
                "maximum": maximum,
            }),
            ErrorType::BlobBlacklisted(bytes) => json!(*blob_hash_to_hex(bytes)),
            _ => json!(null),
        }
    }
}

// End-conversion for methods

/// Converts an `Exn<deepwell::error::Error>` to a JSONRPC object type.
///
/// This is not a `From` implementation since, technically, `Exn<T>` is a
/// foreign type. 🙁
pub fn exn_error_to_rpc_error(exn_error: Exn<Error>) -> ErrorObjectOwned {
    use exn::Frame;

    // Traverse the tree until we hit the highest-level Error
    fn walk(frame: &Frame) -> Option<&Error> {
        match frame.as_any().downcast_ref::<Error>() {
            Some(err) if err.error_type != ErrorType::Request => Some(err),
            _ => frame.children().iter().find_map(walk),
        }
    }

    let error: &Error = walk(exn_error.as_frame())
        .expect("Missing outer wrapped error from JSONRPC request handler");

    let message = str!(exn_error);
    let error_code = error.code();
    let data = error.data();
    ErrorObjectOwned::owned(error_code, message, Some(data))
}
