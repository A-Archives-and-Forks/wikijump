/*
 * services/authorization_token/service.rs
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
use crate::types::ArrayLength;
use uuid::Uuid;
use uuid::fmt::Hyphenated;

pub const AUTHORIZATION_TOKEN_LENGTH: usize = 38;

#[derive(Debug)]
pub struct AuthorizationTokenService;

impl AuthorizationTokenService {
    pub async fn create(
        ctx: &ServiceContext<'_>,
        object_type: AuthorizedObject,
    ) -> Result<String> {
        todo!()
    }

    fn generate(object_type: AuthorizedObject) -> String {
        type TokenBuffer = [u8; 36];
        const_assert_eq!(TokenBuffer::LENGTH, Hyphenated::LENGTH);

        let mut buffer: TokenBuffer = [0; 36];
        Uuid::new_v4().hyphenated().encode_upper(&mut buffer);
        let uuid_str = str::from_utf8(&buffer)
            .expect("UUID hyphenated formatter produced non-UTF-8 output");

        format!("{}-{}", object_type.code(), uuid_str)
    }

    pub async fn verify(
        ctx: &ServiceContext<'_>,
        token: &str,
        object_type: AuthorizedObject,
    ) -> Result<()> {
        todo!()
    }
}

#[test]
fn generate_token() {
    use regex::Regex;
    use std::sync::LazyLock;

    static REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"^[A-Z]-[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$",
        )
        .unwrap()
    });

    fn first_char(string: &str) -> char {
        string.chars().next().expect("empty string")
    }

    fn test(object_type: AuthorizedObject) {
        let token = AuthorizationTokenService::generate(object_type);
        assert_eq!(token.len(), AUTHORIZATION_TOKEN_LENGTH);
        assert_eq!(first_char(&token), object_type.code());
        assert!(REGEX.is_match(&token));
    }

    test(AuthorizedObject::Site);
    test(AuthorizedObject::User);
    test(AuthorizedObject::BotUser);
}
