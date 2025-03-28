/*
 * services/caddy/test.rs
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
use crate::config::Config;

/// Produce a `Config` struct for use in `CaddyService` testing.
fn build_config() -> Config {
    use femme::LevelFilter;
    use ftml::layout::Layout;
    use std::num::NonZeroU16;
    use std::path::PathBuf;
    use std::time::Duration as StdDuration;
    use time::Duration as TimeDuration;

    const MAIN_DOMAIN: &str = "wikijump.test";
    const FILES_DOMAIN: &str = "wjfiles.test";

    Config {
        main_domain_no_dot: str!(MAIN_DOMAIN),
        main_domain: format!(".{MAIN_DOMAIN}"),
        files_domain_no_dot: str!(FILES_DOMAIN),
        files_domain: format!(".{FILES_DOMAIN}"),

        // Unused fields
        raw_toml: String::new(),
        raw_toml_path: PathBuf::new(),
        logger: false,
        logger_level: LevelFilter::Off,
        address: "[::]:2747".parse().unwrap(),
        pid_file: None,
        watch_files: false,
        run_seeder: false,
        seeder_path: PathBuf::new(),
        localization_path: PathBuf::new(),
        authentication_fail_delay: StdDuration::from_secs(0),
        session_token_prefix: String::new(),
        session_token_length: 0,
        normal_session_duration: TimeDuration::seconds(0),
        restricted_session_duration: TimeDuration::seconds(0),
        recovery_code_count: 0,
        recovery_code_length: 0,
        totp_time_step: 0,
        totp_time_skew: 0,
        job_workers: NonZeroU16::new(1).unwrap(),
        job_max_attempts: 0,
        job_work_delay: StdDuration::from_secs(0),
        job_min_poll_delay: StdDuration::from_secs(0),
        job_max_poll_delay: StdDuration::from_secs(0),
        job_prune_session: StdDuration::from_secs(0),
        job_prune_text: StdDuration::from_secs(0),
        job_name_change_refill: StdDuration::from_secs(0),
        job_lift_expired_punishments: StdDuration::from_secs(0),
        render_timeout: StdDuration::from_secs(0),
        rerender_skip: Vec::new(),
        message_layout: Layout::Wikijump,
        default_page_layout: Layout::Wikijump,
        special_page_prefix: String::new(),
        special_page_template: String::new(),
        special_page_missing: String::new(),
        special_page_private: String::new(),
        special_page_banned: String::new(),
        default_name_changes: 0,
        maximum_name_changes: 0,
        refill_name_change: None,
        minimum_name_bytes: 0,
        presigned_path_length: 0,
        presigned_expiry_secs: 0,
        maximum_blob_size: 0,
        maximum_avatar_size: 0,
        maximum_message_subject_bytes: 0,
        maximum_message_body_bytes: 0,
        maximum_message_recipients: 0,
    }
}

#[test]
fn test_caddyfile_gen() {
    // TODO
    todo!()
}
