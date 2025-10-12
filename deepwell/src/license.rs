/*
 * license.rs
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

//! Constant data for licenses usable by Wikijump sites.

use crate::models::sea_orm_active_enums::License;

impl License {
    pub fn slug(self) -> &'static str {
        match self {
            License::CcBySa40 => "cc-by-sa-4.0",
            License::CcBy40 => "cc-by-4.0",
            License::CcByNd40 => "cc-by-nd-4.0",
            License::CcByNc40 => "cc-by-nc-4.0",
            License::CcByNcSa40 => "cc-by-nc-sa-4.0",
            License::CcByNcNd40 => "cc-by-nc-nd-4.0",
            License::CcBySa30 => "cc-by-sa-3.0",
            License::CcBy30 => "cc-by-3.0",
            License::CcByNd30 => "cc-by-nd-3.0",
            License::CcByNc30 => "cc-by-nc-3.0",
            License::CcByNcSa30 => "cc-by-nc-sa-3.0",
            License::CcByNcNd30 => "cc-by-nc-nd-3.0",
            License::CcBySa25 => "cc-by-sa-2.5",
            License::CcBy25 => "cc-by-2.5",
            License::CcByNd25 => "cc-by-nd-2.5",
            License::CcByNc25 => "cc-by-nc-2.5",
            License::CcByNcSa25 => "cc-by-nc-sa-2.5",
            License::CcByNcNd25 => "cc-by-nc-nd-2.5",
            License::GnuFdl13 => "gnu-fdl-1.3",
            License::GnuFdl12 => "gnu-fdl-1.2",
            License::GnuFdl11 => "gnu-fdl-1.1",
        }
    }

    pub fn url(self) -> &'static str {
        match self {
            License::CcBySa40 => "https://creativecommons.org/licenses/by-sa/4.0/",
            License::CcBy40 => "https://creativecommons.org/licenses/by/4.0/",
            License::CcByNd40 => "https://creativecommons.org/licenses/by-nd/4.0/",
            License::CcByNc40 => "https://creativecommons.org/licenses/by-nc/4.0/",
            License::CcByNcSa40 => "https://creativecommons.org/licenses/by-nc-sa/4.0/",
            License::CcByNcNd40 => "https://creativecommons.org/licenses/by-nc-nd/4.0/",
            License::CcBySa30 => "https://creativecommons.org/licenses/by-sa/3.0/",
            License::CcBy30 => "https://creativecommons.org/licenses/by/3.0/",
            License::CcByNd30 => "https://creativecommons.org/licenses/by-nd/3.0/",
            License::CcByNc30 => "https://creativecommons.org/licenses/by-nc/3.0/",
            License::CcByNcSa30 => "https://creativecommons.org/licenses/by-nc-sa/3.0/",
            License::CcByNcNd30 => "https://creativecommons.org/licenses/by-nc-nd/3.0/",
            License::CcBySa25 => "https://creativecommons.org/licenses/by-sa/2.5/",
            License::CcBy25 => "https://creativecommons.org/licenses/by/2.5/",
            License::CcByNd25 => "https://creativecommons.org/licenses/by-nd/2.5/",
            License::CcByNc25 => "https://creativecommons.org/licenses/by-nc/2.5/",
            License::CcByNcSa25 => "https://creativecommons.org/licenses/by-nc-sa/2.5/",
            License::CcByNcNd25 => "https://creativecommons.org/licenses/by-nc-nd/2.5/",
            License::GnuFdl13 => "https://www.gnu.org/licenses/fdl-1.3.html",
            License::GnuFdl12 => "https://www.gnu.org/licenses/old-licenses/fdl-1.2.html",
            License::GnuFdl11 => "https://www.gnu.org/licenses/old-licenses/fdl-1.1.html",
        }
    }
}
