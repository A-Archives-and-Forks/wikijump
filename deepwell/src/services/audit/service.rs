/*
 * services/audit/service.rs
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

use super::prelude::*;
use crate::models::audit_log::{self, Entity as AuditLog, Model as AuditLogModel};

#[derive(Debug)]
pub struct AuditService;

impl AuditService {
    /// Write a new event to the audit log.
    pub async fn log(ctx: &ServiceContext<'_>, event: AuditEvent) -> Result<i64> {
        let RawAuditEvent {
            event_type,
            ip_address,
            user_id,
            site_id,
            page_id,
            extra_id_1,
            extra_id_2,
        } = event.extract();

        let model = audit_log::ActiveModel {
            event_type: Set(str!(event_type)),
            ip_address: Set(str!(ip_address)),
            user_id: Set(user_id),
            site_id: Set(site_id),
            page_id: Set(page_id),
            extra_id_1: Set(extra_id_1),
            extra_id_2: Set(extra_id_2),
            ..Default::default()
        };

        let txn = ctx.transaction();
        let AuditLogModel { event_id, .. } = model.insert(txn).await?;
        info!("Adding audit log event '{event_type}' (ID {event_id})");
        Ok(event_id)
    }
}
