/*
 * services/audit/structs.rs
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

use std::net::IpAddr;

/// An event on the audit log.
///
/// Each type of event has a different set of fields that it provides
#[derive(Deserialize, Debug, Copy, Clone)]
pub enum AuditEvent<'a> {
    UserCreate {
        ip_address: IpAddr,
        user_id: i64,
    },
    SiteCreate {
        ip_address: IpAddr,
        site_id: i64,
    },
    PageCreate {
        ip_address: IpAddr,
        user_id: i64,
        site_id: i64,
        page_id: i64,
        revision_id: i64,
        category_id: i64,
    },
    PageEdit {
        ip_address: IpAddr,
        user_id: i64,
        site_id: i64,
        page_id: i64,
        revision_id: Option<i64>,
    },
    PageMove {
        ip_address: IpAddr,
        user_id: i64,
        site_id: i64,
        page_id: i64,
        revision_id: i64,
        old_slug: &'a str,
        new_slug: &'a str,
    },
    PageDelete {
        ip_address: IpAddr,
        user_id: i64,
        site_id: i64,
        page_id: i64,
        revision_id: i64,
        page_slug: &'a str,
    },
    PageUndelete {
        ip_address: IpAddr,
        user_id: i64,
        site_id: i64,
        page_id: i64,
        revision_id: i64,
        category_id: i64,
        page_slug: &'a str,
    },
}

impl<'a> AuditEvent<'a> {
    pub fn extract(&self) -> RawAuditEvent<'a> {
        match *self {
            AuditEvent::UserCreate {
                ip_address,
                user_id,
            } => RawAuditEvent {
                event_type: "user.create",
                ip_address,
                user_id: Some(user_id),
                site_id: None,
                page_id: None,
                extra_id_1: None,
                extra_id_2: None,
                extra_string_1: None,
                extra_string_2: None,
            },
            AuditEvent::SiteCreate {
                ip_address,
                site_id,
            } => RawAuditEvent {
                event_type: "site.create",
                ip_address,
                user_id: None,
                site_id: Some(site_id),
                page_id: None,
                extra_id_1: None,
                extra_id_2: None,
                extra_string_1: None,
                extra_string_2: None,
            },
            AuditEvent::PageCreate {
                ip_address,
                user_id,
                site_id,
                page_id,
                revision_id,
                category_id,
            } => RawAuditEvent {
                event_type: "page.create",
                ip_address,
                user_id: Some(user_id),
                site_id: Some(site_id),
                page_id: Some(page_id),
                extra_id_1: Some(revision_id),
                extra_id_2: Some(category_id),
                extra_string_1: None,
                extra_string_2: None,
            },
            AuditEvent::PageEdit {
                ip_address,
                user_id,
                site_id,
                page_id,
                revision_id,
            } => RawAuditEvent {
                event_type: "page.edit",
                ip_address,
                user_id: Some(user_id),
                site_id: Some(site_id),
                page_id: Some(page_id),
                extra_id_1: revision_id,
                extra_id_2: None,
                extra_string_1: None,
                extra_string_2: None,
            },
            AuditEvent::PageMove {
                ip_address,
                user_id,
                site_id,
                page_id,
                revision_id,
                old_slug,
                new_slug,
            } => RawAuditEvent {
                event_type: "page.move",
                ip_address,
                user_id: Some(user_id),
                site_id: Some(site_id),
                page_id: Some(page_id),
                extra_id_1: Some(revision_id),
                extra_id_2: None,
                extra_string_1: Some(old_slug),
                extra_string_2: Some(new_slug),
            },
            AuditEvent::PageDelete {
                ip_address,
                user_id,
                site_id,
                page_id,
                revision_id,
                page_slug,
            } => RawAuditEvent {
                event_type: "page.delete",
                ip_address,
                user_id: Some(user_id),
                site_id: Some(site_id),
                page_id: Some(page_id),
                extra_id_1: Some(revision_id),
                extra_id_2: None,
                extra_string_1: Some(page_slug),
                extra_string_2: None,
            },
            AuditEvent::PageUndelete {
                ip_address,
                user_id,
                site_id,
                page_id,
                revision_id,
                category_id,
                page_slug,
            } => RawAuditEvent {
                event_type: "page.undelete",
                ip_address,
                user_id: Some(user_id),
                site_id: Some(site_id),
                page_id: Some(page_id),
                extra_id_1: Some(revision_id),
                extra_id_2: Some(category_id),
                extra_string_1: Some(page_slug),
                extra_string_2: None,
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RawAuditEvent<'a> {
    pub event_type: &'static str,
    pub ip_address: IpAddr,
    pub user_id: Option<i64>,
    pub site_id: Option<i64>,
    pub page_id: Option<i64>,
    pub extra_id_1: Option<i64>,
    pub extra_id_2: Option<i64>,
    pub extra_string_1: Option<&'a str>,
    pub extra_string_2: Option<&'a str>,
}
