/*
 * services/settings/structs.rs
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

/// Describes a navigation page slug.
///
/// This can either be `Enabled(_)`, containing the page slug to use (if it exists),
/// or `Disabled`, which means this navigation element should *not* be rendered
/// for this category.
///
/// # Invariants
/// * `Enabled(_)` never contains an empty string.
#[derive(Debug)]
pub enum NavigationPage {
    Enabled(String),
    Disabled,
}

impl From<String> for NavigationPage {
    fn from(page_slug: String) -> NavigationPage {
        if page_slug.is_empty() {
            NavigationPage::Disabled
        } else {
            NavigationPage::Enabled(page_slug)
        }
    }
}

/// Describes the navigation pages to be used for a category.
///
/// The top item
#[derive(Debug)]
pub struct NavigationPages {
    pub top_bar_page: NavigationPage,
    pub side_bar_page: NavigationPage,
}
