/*
 * handler/fallback_error.rs
 *
 * Wilson's Web Server - Serves a zoo of user-generated content
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

//! Fallback errors, or what wws returns when there are no better errors to emit.
//!
//! When something goes very wrong, and we cannot contact
//! DEEPWELL or read relevant data from the cache in order
//! to give a useful response, an error from here is returned.
//!
//! This is to aid users in reporting the specific issue which
//! occurred, while minimizing the dump of non-localizable text.
//!
//! As a suggestion, leave a comment with the code (e.g. `XF-1003`)
//! whenever `FallbackError::into_response()` is used, enabling
//! future debuggers to simply grep for the code and find where
//! the error is occurring.
//!
//! This is distinct from the concept of the basic error, which is
//! part of DEEPWELL and also used here, which refers to localized error
//! messages built-in to Wikijump which are used when no better error
//! message can be used.
//!
//! As opposed to fallback errors which are specific to WWS, and occur
//! when no better error code can be returned due to unexpected issues.
//! In this sense, fallback errors are "more primordial" than basic errors,
//! since one cause is WWS failing to look up a basic error from DEEPWELL.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FallbackError {
    /// No such basic error code.
    BasicErrorCode,

    /// Attempting to access the basic error route directly.
    /// Or with a missing `X-Wikijump-Basic-Error` internal header.
    BasicErrorDirect,

    /// Unable to retrieve a basic error response from DEEPWELL.
    BasicErrorFetch,

    /// Unable to determine the preferred domain to redirect to for a site.
    RedirectMain,

    /// Unable to fetch a hosted text block from S3.
    TextBlockS3Fetch,
}

impl FallbackError {
    /// Gives a unique error code for this case.
    ///
    /// When adding new error, add to the bottom with a new number.
    /// We should generally avoid reusing prior error codes.
    pub fn error_code(self) -> u32 {
        match self {
            FallbackError::BasicErrorCode => 1000,
            FallbackError::BasicErrorFetch => 1001,
            FallbackError::BasicErrorDirect => 1002,
            FallbackError::RedirectMain => 1003,
            FallbackError::TextBlockS3Fetch => 1004,
        }
    }

    pub fn status_code(self) -> StatusCode {
        match self {
            FallbackError::BasicErrorCode => StatusCode::BAD_REQUEST,
            FallbackError::BasicErrorDirect => StatusCode::FORBIDDEN,
            FallbackError::BasicErrorFetch | FallbackError::RedirectMain => {
                StatusCode::GATEWAY_TIMEOUT
            }
            FallbackError::TextBlockS3Fetch => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for FallbackError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let message = format!("ERROR XF-{}", self.error_code());
        (status_code, message).into_response()
    }
}
