/*
 * tests/misc.rs
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
use serde_json::json;
use time::OffsetDateTime;

#[tokio::test]
async fn misc() {
    let (state, txn) = common::setup().await;
    let ctx = ServiceContext::new(&state, &txn);

    // ping
    run_endpoint!(endpoints::health::ping, ctx);

    // echo
    let object = run_endpoint!(endpoints::misc::echo, ctx, r#"["foo bar"]"#);
    assert_eq!(object, json!(["foo bar"]));

    let object = run_endpoint!(
        endpoints::misc::echo,
        ctx,
        r#"{"apple": "red", "banana": "yellow"}"#,
    );
    assert_eq!(object, json!({"apple": "red", "banana": "yellow"}));

    // yield_error
    let error = run_endpoint_err!(endpoints::misc::yield_error, ctx);
    assert!(matches!(error, ServiceError::BadRequest));

    // config_dump
    let config = run_endpoint!(endpoints::misc::config_dump, ctx);
    assert_eq!(config, state.config.raw_toml);

    // normalize_method
    let normalized =
        run_endpoint!(endpoints::misc::normalize_method, ctx, r#"["SCP-001"]"#);
    assert_eq!(normalized, "scp-001");

    let normalized = run_endpoint!(
        endpoints::misc::normalize_method,
        ctx,
        r#"["Wanderer's Library"]"#,
    );
    assert_eq!(normalized, "wanderer-s-library");

    let normalized =
        run_endpoint!(endpoints::misc::normalize_method, ctx, r#"["abc-xyz"]"#);
    assert_eq!(normalized, "abc-xyz");

    // Invalid arguments
    run_endpoint_err!(endpoints::misc::normalize_method, ctx, r#"{"foo": "bar"}"#);

    // info
    let info = run_endpoint!(endpoints::info::server_info, ctx);
    assert_eq!(info.package.name, deepwell::info::PKG_NAME);
    assert_eq!(info.package.version, *deepwell::info::VERSION_INFO);
    assert_eq!(info.package.description, deepwell::info::PKG_DESCRIPTION);
    assert_eq!(info.package.license, deepwell::info::PKG_LICENSE);
    assert_eq!(info.package.repository, deepwell::info::PKG_REPOSITORY);
    assert_eq!(info.compile_info.built_at, *deepwell::info::BUILT_TIME_UTC);
    assert_eq!(info.compile_info.rustc_version, deepwell::info::RUSTC_VERSION);
    assert_eq!(info.compile_info.endian, deepwell::info::CFG_ENDIAN);
    assert_eq!(info.compile_info.target, deepwell::info::TARGET);
    assert_eq!(info.compile_info.threads, deepwell::info::NUM_JOBS);
    assert_eq!(info.compile_info.git_commit, deepwell::info::GIT_COMMIT_HASH);
    assert_eq!(info.config_path, state.config.raw_toml_path);
    assert_eq!(info.hostname, *deepwell::info::HOSTNAME);
    assert!(info.current_time > OffsetDateTime::UNIX_EPOCH);

    cleanup!(state, txn, ctx);
}
