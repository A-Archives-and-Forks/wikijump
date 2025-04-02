/*
 * handler/error.rs
 *
 * Wilson's Web Server - Serves a zoo of user-generated content
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

//! Basic error emission for low-level failures.
//!
//! When something goes very wrong, and we cannot contact
//! DEEPWELL or read relevant data from the cache in order
//! to give a useful response, an error from here is returned.
//!
//! This is to aid users in reporting the specific issue which
//! occurred, while minimizing the dump of non-localizable text.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FallbackError {
    RedirectMain,
}

impl FallbackError {
    /// Gives a unique error code for this case.
    ///
    /// When adding new error, add to the bottom with a new number.
    /// We should generally avoid reusing prior error codes.
    pub fn error_code(self) -> u32 {
        match self {
            FallbackError::RedirectMain => 1000,
        }
    }
}

impl IntoResponse for FallbackError {
    fn into_response(self) -> Response {
        let message = format!("ERROR XF-{}", self.error_code());
        (StatusCode::GATEWAY_TIMEOUT, message).into_response()
    }
}
