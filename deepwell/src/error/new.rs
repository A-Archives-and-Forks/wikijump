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
    // 1000
    ApplicationStart,
    Request,
    Authentication,
    DatabaseTransaction,
    DatabaseSeeder,

    // 1100
    ServerSetup,
    DatabaseSetup,
    RedisSetup,
    RenderTimeout,
    RateLimited,
    EmailVerification,

    // 2000
    GeneralNotFound,
    AliasNotFound,
    RelationNotFound,
    UserNotFound,
    SiteNotFound,
    PageNotFound,
    PageCategoryNotFound,
    PageParentNotFound,
    PageRevisionNotFound,
    FileNotFound,
    FileRevisionNotFound,
    VoteNotFound,
    FilterNotFound,
    CustomDomainNotFound,
    MessageNotFound,
    MessageDraftNotFound,
    BlobNotFound,
    TextNotFound,

    // 2100
    UserExists,
    UserMfaExists,
    SiteExists,
    PageExists,
    PageSlugExists,
    PageParentExists,
    FileExists,
    FilterExists,
    CustomDomainExists,

    // 3000
    InvalidAuthentication,
    InvalidSessionToken,
    SessionUserId {
        active_user_id: i64,
        session_user_id: i64,
    },
    EmptyPassword,

    // 4000
    BadRequest,
    InvalidEnumValue,

    // 4100
    UserNameTooShort,
    UserSlugEmpty,
    UserEmailEmpty,
    UserWrongType,
    InsufficientNameChanges,
    DisallowedEmail,
    InvalidEmail,

    // 4200
    SiteSlugEmpty,

    // 4300
    PageSlugEmpty,
    PageNotDeleted,
    CannotHideLatestRevision,
    NotLatestRevisionId,

    // 4400
    FileNameEmpty,
    FileNameTooLong {
        length: usize,
        maximum: usize,
    },
    FileNameInvalidCharacters,
    FileMimeEmpty,
    FileNotDeleted,

    // 5000
    LocaleInvalid {
        locale: String,
    },
    LocaleMissing {
        locale: String,
    },
    LocaleMessageMissing {
        message_key: String,
    },
    LocaleMessageValueMissing {
        message_key: String,
    },
    LocaleMessageAttributeMissing {
        message_key: String,
        attribute: String,
    },
    NoLocalesSpecified,

    // 5100
    FilterViolation,
    FilterNotDeleted,

    // 5200
    BlobNotUploaded,
    BlobWrongUser,
    BlobTooBig,
    BlobSizeMismatch {
        expected: usize,
        actual: usize,
    },
    BlobBlacklisted(BlobHash),
    BlobCannotBlacklistExisting,

    // 5300
    MessageSubjectEmpty,
    MessageSubjectTooLong,
    MessageBodyEmpty,
    MessageBodyTooLong,
    MessageNoRecipients,
    MessageTooManyRecipients,

    // 5400
    CustomDomainSubdomain,
    CustomDomainWrongSite,

    // 6000
    UserBlockedUser,
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

    /// Returns a basic summary of what this error is meant to represent.
    ///
    /// See `ErrorType::summary()` for details.
    #[inline]
    pub fn summary(&self) -> &'static str {
        self.error_type.summary()
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
            ErrorType::Authentication => 1002,
            ErrorType::DatabaseTransaction => 1003,
            ErrorType::DatabaseSeeder => 1004,

            // 1100 - Intermediate Setup
            ErrorType::ServerSetup => 1100,
            ErrorType::DatabaseSetup => 1101,
            ErrorType::RedisSetup => 1102,
            ErrorType::RenderTimeout => 1103,
            ErrorType::RateLimited => 1104,
            ErrorType::EmailVerification => 1105,

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

    /// Returns a basic summary of what this error is meant to represent.
    ///
    /// This is always a `&'static str`, so this lookup is cheap and has
    /// no effect of memory consumption.
    pub fn summary(&self) -> &'static str {
        match self {
            // 1000
            ErrorType::ApplicationStart => "Application failed to start",
            ErrorType::Request => "This request returned an error",
            ErrorType::Authentication => {
                "A server error occurred while attempting to authenticate"
            }
            ErrorType::DatabaseTransaction => {
                "Database transaction was aborted due to error"
            }
            ErrorType::DatabaseSeeder => "Database seeding failed",

            // 1100
            ErrorType::ServerSetup => "Failed to set up server internal state",
            ErrorType::DatabaseSetup => "Failed to set up the database connection",
            ErrorType::RedisSetup => "Failed to set up the Redis connection",
            ErrorType::RateLimited => "An external API has ratelimited us",
            ErrorType::RenderTimeout => "Wikitext parsing and rendering has timed out",
            ErrorType::EmailVerification => "Email verification failed",

            // 2000
            ErrorType::GeneralNotFound => "Unspecified entity does not exist",
            ErrorType::AliasNotFound => "Alias does not exist",
            ErrorType::RelationNotFound => "Relation value does not exist",
            ErrorType::UserNotFound => "User does not exist",
            ErrorType::SiteNotFound => "Site does not exist",
            ErrorType::PageNotFound => "Page does not exist",
            ErrorType::PageCategoryNotFound => "Page category does not exist",
            ErrorType::PageParentNotFound => "Page parent does not exist",
            ErrorType::PageRevisionNotFound => "Page revision does not exist",
            ErrorType::FileNotFound => "File does not exist",
            ErrorType::FileRevisionNotFound => "File revision does not exist",
            ErrorType::VoteNotFound => "Vote does not exist",
            ErrorType::FilterNotFound => "Filter does not exist",
            ErrorType::CustomDomainNotFound => "Custom domain does not exist",
            ErrorType::MessageNotFound => "Message does not exist",
            ErrorType::MessageDraftNotFound => "Message draft does not exist",
            ErrorType::BlobNotFound => "Blob item does not exist",
            ErrorType::TextNotFound => "Text item does not exist",

            // 2100
            ErrorType::UserExists => "Cannot perform, user already exists",
            ErrorType::UserMfaExists => "Cannot set up user MFA, already set up",
            ErrorType::SiteExists => "Cannot perform, site already exists",
            ErrorType::PageExists => "Cannot perform, page already exists",
            ErrorType::PageSlugExists => "Cannot perform, page slug already exists",
            ErrorType::PageParentExists => "Cannot perform, page parent already exists",
            ErrorType::FileExists => "Cannot perform, file already exists",
            ErrorType::FilterExists => "Cannot perform, filter already exists",
            ErrorType::CustomDomainExists => {
                "Cannot perform, custom domain already exists"
            }

            // 3000
            ErrorType::InvalidAuthentication => {
                "Invalid username, password, or TOTP code"
            }
            ErrorType::InvalidSessionToken => {
                "Invalid session token, cannot be used for authentication"
            }
            ErrorType::SessionUserId { .. } => {
                "User associated with the session does not match the active user"
            }
            ErrorType::EmptyPassword => "A password was required, but not provided",

            // 4000
            ErrorType::BadRequest => "The request is in some way malformed or incorrect",
            ErrorType::InvalidEnumValue => "Invalid enum serialization value",

            // 4100
            ErrorType::UserNameTooShort => "User name is too short",
            ErrorType::UserSlugEmpty => "User slug cannot be empty",
            ErrorType::UserEmailEmpty => "User email cannot be empty",
            ErrorType::UserWrongType => "Wrong user type for this operation",
            ErrorType::InsufficientNameChanges => {
                "The user cannot rename as they do not have enough name change tokens"
            }
            ErrorType::DisallowedEmail => "The user's email is disallowed",
            ErrorType::InvalidEmail => "The user's email is invalid",

            // 4200
            ErrorType::SiteSlugEmpty => "Site slug cannot be empty",

            // 4300
            ErrorType::PageSlugEmpty => "Page slug cannot be empty",
            ErrorType::PageNotDeleted => "Cannot restore a non-deleted page",
            ErrorType::CannotHideLatestRevision => {
                "Cannot hide the wikitext for the latest page revision"
            }
            ErrorType::NotLatestRevisionId => {
                "Revision ID passed for this operation is not the latest"
            }

            // 4400
            ErrorType::FileNameEmpty => "File name cannot be empty",
            ErrorType::FileNameTooLong { .. } => "File name too long",
            ErrorType::FileNameInvalidCharacters => {
                "File name contains invalid characters (control chars or slashes)"
            }
            ErrorType::FileMimeEmpty => "File MIME type cannot be empty",
            ErrorType::FileNotDeleted => "Cannot restore a non-deleted file",

            // 5000
            ErrorType::LocaleInvalid { .. } => "Invalid locale name",
            ErrorType::LocaleMissing { .. } => {
                "No messages are available for this locale"
            }
            ErrorType::LocaleMessageMissing { .. } => {
                "Message key not found for this locale"
            }
            ErrorType::LocaleMessageValueMissing { .. } => {
                "Message key was found, but has no value"
            }
            ErrorType::LocaleMessageAttributeMissing { .. } => {
                "Message key was found, but does not have this attribute"
            }
            ErrorType::NoLocalesSpecified => "No locales were specified in the request",

            // 5100
            ErrorType::FilterViolation => {
                "The request violates a configured content filter"
            }
            ErrorType::FilterNotDeleted => "Cannot restore a non-deleted filter",

            // 5200
            ErrorType::BlobNotUploaded => "Blob not uploaded",
            ErrorType::BlobWrongUser => "Cannot use blob uploaded by different user",
            ErrorType::BlobTooBig => "Uploaded blob is too big for this operation",
            ErrorType::BlobSizeMismatch { .. } => {
                "Uploaded blob does not match expected length"
            }
            ErrorType::BlobBlacklisted(_) => "Uploaded blob content is blacklisted",
            ErrorType::BlobCannotBlacklistExisting => {
                "Cannot blacklist a blob which is already in use, you must do a hard deletion"
            }

            // 5300
            ErrorType::MessageSubjectEmpty => "Message subject cannot be empty",
            ErrorType::MessageSubjectTooLong => "Message subject too long",
            ErrorType::MessageBodyEmpty => "Message body cannot be empty",
            ErrorType::MessageBodyTooLong => "Message body too long",
            ErrorType::MessageNoRecipients => "Message cannot have no recipients",
            ErrorType::MessageTooManyRecipients => "Message has too many recipients",

            // 5400
            ErrorType::CustomDomainSubdomain => {
                "Custom domains may not be subdomains of the Wikijump main or file domains"
            }
            ErrorType::CustomDomainWrongSite => {
                "Cannot use custom domain, as it belongs to a different site"
            }

            // 6000
            ErrorType::UserBlockedUser => {
                "Cannot perform this action because you are blocked by the user"
            }
            ErrorType::SiteBlockedUser => {
                "Cannot perform this action because you are blocked by the site"
            }
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
    use serde_json::json;

    // Traverse the tree until we hit the highest-level Error
    // that is not a 'request' type. As a wrapper, it's not going
    // to be the most useful high-level Error.
    fn walk(frame: &Frame) -> Option<&Error> {
        match frame.as_any().downcast_ref::<Error>() {
            Some(err) if err.error_type != ErrorType::Request => Some(err),
            _ => frame.children().iter().find_map(walk),
        }
    }

    let error: &Error = walk(exn_error.as_frame())
        .expect("Missing outer wrapped error from JSONRPC request handler");

    let error_code = error.code();
    let message = error.summary();
    let data = json!({
        "call_trace": str!(exn_error),
        "data": error.data(),
    });
    ErrorObjectOwned::owned(error_code, message, Some(data))
}
