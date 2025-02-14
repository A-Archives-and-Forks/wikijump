/*
 * handler/misc.rs
 *
 * Wilson's Web Server - Serves a zoo of content (framerail, user files, code, etc)
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

use axum::{
    body::Body,
    http::{header, status::StatusCode},
    response::Response,
};

pub async fn handle_teapot() -> Response {
    Response::builder()
        .status(StatusCode::IM_A_TEAPOT)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Body::from("🫖"))
        .expect("Unable to convert response data")
}

pub async fn handle_invalid_method() -> StatusCode {
    StatusCode::METHOD_NOT_ALLOWED
}
