/*
 * services/context.rs
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

use crate::api::ServerState;
use crate::config::Config;
use crate::error::prelude::*;
use crate::locales::Localizations;
use crate::models::session::Model as SessionModel;
use crate::services::blob::MimeAnalyzer;
use crate::types::Reference;
use redis::aio::MultiplexedConnection as RedisMultiplexedConnection;
use rsmq_async::Rsmq;
use s3::bucket::Bucket;
use sea_orm::DatabaseTransaction;
use std::sync::Arc;

/// Per-request context derived from HTTP headers by the middleware layer.
#[derive(Debug, Clone, Default)]
pub struct RequestContext {
    pub session: Option<SessionModel>,
    pub user_id: Option<i64>,
    pub site_id: Option<i64>,
    pub page_reference: Option<Reference<'static>>,
}

impl RequestContext {
    #[inline]
    pub fn user_session(&self) -> Option<&SessionModel> {
        self.session.as_ref()
    }

    #[inline]
    pub fn user_id(&self) -> Option<i64> {
        self.user_id
    }

    #[inline]
    pub fn site_id(&self) -> Option<i64> {
        self.site_id
    }

    #[inline]
    pub fn page_reference(&self) -> Option<&Reference<'_>> {
        self.page_reference.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct ServiceContext<'txn> {
    state: ServerState,
    transaction: &'txn DatabaseTransaction,
    request_ctx: RequestContext,
}

impl<'txn> ServiceContext<'txn> {
    // NOTE: It is the responsibility of the caller to manage commit / rollback
    //       for transactions.
    //
    //       For our endpoints, this is managed in the wrapper macro in api.rs
    pub fn new(state: &ServerState, transaction: &'txn DatabaseTransaction) -> Self {
        ServiceContext {
            state: Arc::clone(state),
            transaction,
            request_ctx: RequestContext::default(),
        }
    }

    pub fn with_request_context(self, request_ctx: RequestContext) -> Self {
        Self {
            request_ctx,
            ..self
        }
    }

    // Getters
    #[inline]
    pub fn state(&self) -> ServerState {
        Arc::clone(&self.state)
    }

    #[inline]
    pub fn config(&self) -> &Config {
        &self.state.config
    }

    #[inline]
    pub fn redis(&self) -> RedisMultiplexedConnection {
        RedisMultiplexedConnection::clone(&self.state.redis)
    }

    #[inline]
    pub fn rsmq(&self) -> Rsmq {
        Rsmq::clone(&self.state.rsmq)
    }

    #[inline]
    pub fn localization(&self) -> &Localizations {
        &self.state.localizations
    }

    #[inline]
    pub fn mime(&self) -> &MimeAnalyzer {
        &self.state.mime_analyzer
    }

    #[inline]
    pub fn s3_files_bucket(&self) -> &Bucket {
        &self.state.s3_files_bucket
    }

    #[inline]
    pub fn s3_tblocks_bucket(&self) -> &Bucket {
        &self.state.s3_tblocks_bucket
    }

    #[inline]
    pub fn transaction(&self) -> &'txn DatabaseTransaction {
        self.transaction
    }

    #[inline]
    pub fn request_context(&self) -> &RequestContext {
        &self.request_ctx
    }
}
