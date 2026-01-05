/*
 * tests/locale.rs
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

#[macro_use]
mod common;

use deepwell::endpoints;
use deepwell::services::ServiceContext;

#[tokio::test]
async fn locale() {
    let (state, txn) = common::setup().await;
    let ctx = ServiceContext::new(&state, &txn);

    // locale_info
    let info = run_endpoint!(endpoints::locale::locale_info, ctx, r#"["en-gb"]"#);
    assert_eq!(info.language, "en");
    assert!(info.region.is_some());
    assert_eq!(info.region.unwrap(), "GB");
    assert!(info.script.is_none());
    assert!(info.variants.is_empty());

    let info = run_endpoint!(endpoints::locale::locale_info, ctx, r#"["fr_Latn-FR-MACOS"]"#);
    assert_eq!(info.language, "fr");
    assert!(info.region.is_some());
    assert_eq!(info.region.unwrap(), "FR");
    assert!(info.script.is_some());
    assert_eq!(info.script.unwrap(), "Latn");
    assert_eq!(info.variants.len(), 1);
    assert_eq!(info.variants[0], "macos");

    // translate_strings

    cleanup!(state, txn, ctx);
}
