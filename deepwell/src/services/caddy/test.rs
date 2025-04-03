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

//! Unit testing for our generated `Caddyfile`s.

use super::prelude::*;
use crate::config::Config;
use crate::services::CaddyService;
use maplit::hashmap;
use pretty_assertions::assert_eq;

fn build_config(main_domain: &str, files_domain: &str) -> Config {
    use femme::LevelFilter;
    use ftml::layout::Layout;
    use std::num::NonZeroU16;
    use std::path::PathBuf;
    use std::time::Duration as StdDuration;
    use time::Duration as TimeDuration;

    assert!(!main_domain.starts_with('.'));
    assert!(!files_domain.starts_with('.'));

    Config {
        main_domain_no_dot: str!(main_domain),
        main_domain: format!(".{main_domain}"),
        files_domain_no_dot: str!(files_domain),
        files_domain: format!(".{files_domain}"),

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

fn build_site_data() -> (SiteData, SiteData) {
    let basic = SiteData {
        sites: vec![
            (1, str!("foo"), None),
            (2, str!("bar"), Some(str!("example.com"))),
        ],
        domains: hashmap! {
            1 => SiteDomainData::default(),
            2 => SiteDomainData {
                aliases: vec![],
                custom_domains: vec![str!("example.com")],
            },
        },
    };

    let full = SiteData {
        sites: vec![
            (1, str!("www"), None),
            (2, str!("empty"), None),
            (3, str!("mytest"), None),
            (
                4,
                str!("wanderers-library"),
                Some(str!("wandererslibrary.com")),
            ),
            (5, str!("scp-wiki"), Some(str!("scpwiki.com"))),
        ],
        domains: hashmap! {
            1 => SiteDomainData::default(),
            2 => SiteDomainData::default(),
            3 => SiteDomainData {
                aliases: vec![str!("check")],
                custom_domains: vec![str!("example.com"), str!("example.net")],
            },
            4 => SiteDomainData {
                aliases: vec![],
                custom_domains: vec![str!("wandererslibrary.com")],
            },
            5 => SiteDomainData {
                aliases: vec![str!("scpwiki")],
                custom_domains: vec![str!("scpwiki.com"), str!("scp-wiki.net"), str!("scp.foundation"), str!("foundation.scp")],
            },
        },
    };

    (basic, full)
}

macro_rules! test_output {
    ($suffix:expr) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test/Caddyfile.",
            $suffix,
        ))
    };
}

const CADDYFILE_BASIC_PROD: &str = test_output!("basic_prod");
const CADDYFILE_BASIC_LOCAL: &str = test_output!("basic_local");
const CADDYFILE_BASIC_LOCAL_DEV: &str = test_output!("basic_localdev");
const CADDYFILE_BASIC_DIFFERENT_PROXIES: &str = test_output!("proxies");
const CADDYFILE_FULL_PROD: &str = test_output!("full_prod");
const CADDYFILE_LONG_DOMAIN: &str = test_output!("long");

#[test]
fn generate_caddyfiles() {
    const FRAMERAIL_HOST: &str = "framerail:3000";
    const WWS_HOST: &str = "wws:7000";

    let config_basic = build_config("wikijump.test", "wjfiles.test");
    let config_local = build_config("wikijump.localhost", "wjfiles.localhost");
    let config_long = build_config(
        "site.wikijump.com",
        "wjfiles.host.site.somedomain.example.com",
    );
    let (sites_basic, sites_full) = build_site_data();

    macro_rules! check {
        ($expected:expr, $config:expr, $sites:expr, $options:expr $(,)?) => {{
            let mut actual = CaddyService::generate_custom(&$config, &$options, &$sites);

            // Strip off trailing newlines, not something we care about,
            // and precisely managing them is a waste of time.

            let expected = $expected.trim();
            while actual.ends_with('\n') {
                actual.pop();
            }

            assert_eq!(
                expected,
                actual,
                "\
Generated Caddy file did not match!


UNIT TEST INFO:
* Expected output: {}
* Main domain: {}
* Files domain: {}
* Site data: {}
* Options: {:#?}
",
                stringify!($expected),
                $config.main_domain_no_dot,
                $config.files_domain_no_dot,
                stringify!($sites),
                $options,
            );
        }};
    }

    check!(
        CADDYFILE_BASIC_PROD,
        config_basic,
        sites_basic,
        CaddyfileOptions {
            debug: false,
            local: false,
            http_port: None,
            https_port: None,
            framerail_host: cow!(FRAMERAIL_HOST),
            wws_host: cow!(WWS_HOST),
        },
    );

    check!(
        CADDYFILE_BASIC_LOCAL,
        config_local,
        sites_basic,
        CaddyfileOptions {
            debug: false,
            local: true,
            http_port: None,
            https_port: None,
            framerail_host: cow!(FRAMERAIL_HOST),
            wws_host: cow!(WWS_HOST),
        },
    );

    check!(
        CADDYFILE_BASIC_LOCAL_DEV,
        config_local,
        sites_basic,
        CaddyfileOptions {
            debug: true,
            local: true,
            http_port: Some(8000),
            https_port: Some(8443),
            framerail_host: cow!(FRAMERAIL_HOST),
            wws_host: cow!(WWS_HOST),
        },
    );

    check!(
        CADDYFILE_BASIC_DIFFERENT_PROXIES,
        config_basic,
        sites_basic,
        CaddyfileOptions {
            debug: false,
            local: false,
            http_port: None,
            https_port: None,
            framerail_host: cow!("web_proxy_host"),
            wws_host: cow!("wws_proxy_host"),
        },
    );

    check!(
        CADDYFILE_FULL_PROD,
        config_basic,
        sites_full,
        CaddyfileOptions {
            debug: false,
            local: false,
            http_port: None,
            https_port: None,
            framerail_host: cow!(FRAMERAIL_HOST),
            wws_host: cow!(WWS_HOST),
        },
    );

    check!(
        CADDYFILE_LONG_DOMAIN,
        config_long,
        sites_basic,
        CaddyfileOptions {
            debug: false,
            local: false,
            http_port: None,
            https_port: None,
            framerail_host: cow!(FRAMERAIL_HOST),
            wws_host: cow!(WWS_HOST),
        },
    );
}
