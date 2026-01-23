/*
 * utils/locale.rs
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

use crate::error::prelude::*;
use unic_langid::LanguageIdentifier;

/// Ensure the given locale string is valid, returning the parsed locale.
/// If it is invalid, then the appropriate `Error` variant is returned.
pub fn validate_locale(locale_str: &str) -> OldResult<LanguageIdentifier> {
    LanguageIdentifier::from_bytes(locale_str.as_bytes()).map_err(|error| {
        warn!("Invalid locale '{locale_str}' passed: {error:?}");
        OldError::LocaleInvalid(error)
    })
}

/// Helper function to convert an array of strings to a list of locales.
///
/// Empty locales lists _are_ allowed, since we have not
/// yet checked the user's locale preferences.
pub fn parse_locales<S: AsRef<str>>(
    locales_str: &[S],
) -> OldResult<Vec<LanguageIdentifier>> {
    let mut locales = Vec::with_capacity(locales_str.len());
    for locale_str in locales_str {
        let locale = LanguageIdentifier::from_bytes(locale_str.as_ref().as_bytes())?;
        locales.push(locale);
    }
    Ok(locales)
}
