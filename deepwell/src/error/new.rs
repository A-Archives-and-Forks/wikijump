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

use crate::hash::{BlobHash, blob_hash_to_hex};
use serde_json::Value as JsonValue;
use std::error::Error as StdError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct NewError {
    pub message: String,
    pub r#type: NewErrorType,
}

#[derive(Debug)]
pub enum NewErrorType {
    /// Application failed to start.
    ApplicationStart,

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

impl StdError for NewError {}

impl Display for NewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:04}] {}", self.code(), self.message)
    }
}

impl NewError {
    #[inline]
    pub fn new<S: Into<String>>(message: S, error_type: NewErrorType) -> Self {
        NewError {
            message: message.into(),
            r#type: error_type,
        }
    }

    pub fn code(&self) -> i32 {
        match self.r#type {
            //
            // 1000 -- Top-Level
            //

            // 1000 - General
            NewErrorType::ApplicationStart => 1000,

            //
            // 2000 -- Data Consistency
            //

            // 2000 - Not Found
            NewErrorType::GeneralNotFound => 2000,
            NewErrorType::AliasNotFound => 2001,
            NewErrorType::RelationNotFound => 2002,
            NewErrorType::UserNotFound => 2003,
            NewErrorType::SiteNotFound => 2004,
            NewErrorType::PageNotFound => 2005,
            NewErrorType::PageCategoryNotFound => 2006,
            NewErrorType::PageParentNotFound => 2007,
            NewErrorType::PageRevisionNotFound => 2008,
            NewErrorType::FileNotFound => 2009,
            NewErrorType::FileRevisionNotFound => 2010,
            NewErrorType::VoteNotFound => 2011,
            NewErrorType::FilterNotFound => 2012,
            NewErrorType::CustomDomainNotFound => 2013,
            NewErrorType::MessageNotFound => 2014,
            NewErrorType::MessageDraftNotFound => 2015,
            NewErrorType::BlobNotFound => 2016,
            NewErrorType::TextNotFound => 2017,

            // 2100 - Already Exists
            NewErrorType::UserExists => 2100,
            NewErrorType::UserMfaExists => 2101,
            NewErrorType::SiteExists => 2102,
            NewErrorType::PageExists => 2103,
            NewErrorType::PageSlugExists => 2104,
            NewErrorType::PageParentExists => 2105,
            NewErrorType::FileExists => 2106,
            NewErrorType::FilterExists => 2107,
            NewErrorType::CustomDomainExists => 2108,

            //
            // 3000 -- Client / Protocol Errors
            //

            // 3000 - Authentication
            NewErrorType::InvalidAuthentication => 3000,
            NewErrorType::InvalidSessionToken => 3001,
            NewErrorType::SessionUserId { .. } => 3002,
            NewErrorType::EmptyPassword => 3003,

            // 3100 - Permission
            // TODO

            // 3200 - Server-side
            NewErrorType::AuthenticationBackend => 3200,
            NewErrorType::RenderTimeout => 3201,
            NewErrorType::RateLimited => 3202,
            NewErrorType::EmailVerification => 3203,

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
            NewErrorType::BadRequest => 4000,
            NewErrorType::InvalidEnumValue => 4001,

            // 4100 - User
            NewErrorType::UserNameTooShort => 4100,
            NewErrorType::UserSlugEmpty => 4101,
            NewErrorType::UserEmailEmpty => 4102,
            NewErrorType::UserWrongType => 4103,
            NewErrorType::InsufficientNameChanges => 4104,
            NewErrorType::InvalidEmail => 4105,
            NewErrorType::DisallowedEmail => 4106,

            // 4200 - Site
            NewErrorType::SiteSlugEmpty => 4200,

            // 4300 - Page
            NewErrorType::PageSlugEmpty => 4300,
            NewErrorType::PageNotDeleted => 4301,
            NewErrorType::CannotHideLatestRevision => 4302,
            NewErrorType::NotLatestRevisionId => 4303,

            // 4400 - File
            NewErrorType::FileNameEmpty => 4400,
            NewErrorType::FileNameTooLong { .. } => 4401,
            NewErrorType::FileNameInvalidCharacters => 4402,
            NewErrorType::FileMimeEmpty => 4403,
            NewErrorType::FileNotDeleted => 4404,

            //
            // 5000 -- Client / Request Errors - Ancillary Data Objects
            //

            // 5000 - Locale
            NewErrorType::LocaleInvalid { .. } => 5000,
            NewErrorType::LocaleMissing { .. } => 5001,
            NewErrorType::LocaleMessageMissing { .. } => 5002,
            NewErrorType::LocaleMessageValueMissing { .. } => 5003,
            NewErrorType::LocaleMessageAttributeMissing { .. } => 5004,
            NewErrorType::NoLocalesSpecified => 5005,

            // 5100 - Filter
            NewErrorType::FilterViolation => 5100,
            NewErrorType::FilterNotDeleted => 5102,

            // 5200 - Blob
            NewErrorType::BlobNotUploaded => 5200,
            NewErrorType::BlobWrongUser => 5201,
            NewErrorType::BlobTooBig => 5202,
            NewErrorType::BlobSizeMismatch { .. } => 5204,
            NewErrorType::BlobBlacklisted(_) => 5205,
            NewErrorType::BlobCannotBlacklistExisting => 5206,

            // 5300 - Message
            NewErrorType::MessageSubjectEmpty => 5300,
            NewErrorType::MessageSubjectTooLong => 5301,
            NewErrorType::MessageBodyEmpty => 5302,
            NewErrorType::MessageBodyTooLong => 5303,
            NewErrorType::MessageNoRecipients => 5304,
            NewErrorType::MessageTooManyRecipients => 5305,

            // 5400 - Domains
            NewErrorType::CustomDomainWrongSite => 5400,
            NewErrorType::CustomDomainSubdomain => 5401,

            //
            // 6000 -- Client / Request Errors - Composite Data
            //

            // 6000 - Relations
            NewErrorType::SiteBlockedUser => 6000,
            NewErrorType::UserBlockedUser => 6001,
            //
            // 7000, 8000, 9000 -- (RESERVED)
            //
        }
    }
}
