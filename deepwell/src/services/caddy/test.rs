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

const CADDYFILE_BASIC_PROD: &str = "\
# Global options
{
	metrics {
		per_host
	}
}

#
# MAIN
#

(serve_main) {
	# Redirect, route is on the files server
	@files {
		path /*/code/*
		path /*/html/*
		path /*/file/*  # for the /{slug}/file/{filename} convenience routes
		path /*/download/*
		path /local--files/*
		path /local--code/*
		path /local--html/*
		path /-/files/*
		path /-/file/*
		path /-/download/*
		path /-/code/*
		path /-/html/*
	}
	redir @files https://{vars.site_slug}.wjfiles.test{uri}

	# Enable default compression settings
	encode

	# Finally, proxy to framerail to get the actual HTML
	# Note, the x-wikijump-site-* headers have already been set at this point
	reverse_proxy http://framerail:3000
}

foo.wikijump.test {
	vars {
		site_id 1
		site_slug foo
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.foo.wikijump.test {
	redir https://foo.wikijump.test{uri}
}

bar.wikijump.test,
www.bar.wikijump.test {
	redir https://example.com{uri}
}

example.com {
	vars {
		site_id 2
		site_slug bar
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.example.com {
	redir https://example.com{uri}
}

#
# FILES
#

(serve_files) {
	encode
	reverse_proxy http://wws:7000
}

*.wjfiles.test {
	@foo host foo.wjfiles.test
	vars @foo site_id 1

	@bar host bar.wjfiles.test
	vars @bar site_id 2

	request_header X-Wikijump-Site-Slug {labels.2}
	request_header X-Wikijump-Site-Id {vars.site_id}

	import serve_files
}

#
# FALLBACK
#

http://,
https:// {
	request_header X-Wikijump-Special-Error 1
	rewrite * /-/special-error/missing-site
	reverse_proxy http://framerail:3000
}";

const CADDYFILE_BASIC_LOCAL: &str = "\
# Global options
{
	metrics {
		per_host
	}
	skip_install_trust
}

#
# MAIN
#

(serve_main) {
	# Redirect, route is on the files server
	@files {
		path /*/code/*
		path /*/html/*
		path /*/file/*  # for the /{slug}/file/{filename} convenience routes
		path /*/download/*
		path /local--files/*
		path /local--code/*
		path /local--html/*
		path /-/files/*
		path /-/file/*
		path /-/download/*
		path /-/code/*
		path /-/html/*
	}
	redir @files https://{vars.site_slug}.wjfiles.localhost{uri}

	# Enable default compression settings
	encode

	# Finally, proxy to framerail to get the actual HTML
	# Note, the x-wikijump-site-* headers have already been set at this point
	reverse_proxy http://framerail:3000
}

foo.wikijump.localhost {
	vars {
		site_id 1
		site_slug foo
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.foo.wikijump.localhost {
	redir https://foo.wikijump.localhost{uri}
}

bar.wikijump.localhost,
www.bar.wikijump.localhost {
	redir https://example.com{uri}
}

example.com {
	vars {
		site_id 2
		site_slug bar
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.example.com {
	redir https://example.com{uri}
}

#
# FILES
#

(serve_files) {
	encode
	reverse_proxy http://wws:7000
}

*.wjfiles.localhost {
	@foo host foo.wjfiles.localhost
	vars @foo site_id 1

	@bar host bar.wjfiles.localhost
	vars @bar site_id 2

	request_header X-Wikijump-Site-Slug {labels.2}
	request_header X-Wikijump-Site-Id {vars.site_id}

	import serve_files
}

#
# FALLBACK
#

http://,
https://,
localhost {
	request_header X-Wikijump-Special-Error 1
	rewrite * /-/special-error/missing-site
	reverse_proxy http://framerail:3000
}";

const CADDYFILE_BASIC_LOCAL_DEV: &str = "\
# Global options
{
	metrics {
		per_host
	}
	http_port 8000
	https_port 8443
	debug
	skip_install_trust
}

#
# MAIN
#

(serve_main) {
	# Redirect, route is on the files server
	@files {
		path /*/code/*
		path /*/html/*
		path /*/file/*  # for the /{slug}/file/{filename} convenience routes
		path /*/download/*
		path /local--files/*
		path /local--code/*
		path /local--html/*
		path /-/files/*
		path /-/file/*
		path /-/download/*
		path /-/code/*
		path /-/html/*
	}
	redir @files https://{vars.site_slug}.wjfiles.localhost{uri}

	# Enable default compression settings
	encode

	# Finally, proxy to framerail to get the actual HTML
	# Note, the x-wikijump-site-* headers have already been set at this point
	reverse_proxy http://framerail:3000
}

foo.wikijump.localhost {
	vars {
		site_id 1
		site_slug foo
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.foo.wikijump.localhost {
	redir https://foo.wikijump.localhost{uri}
}

bar.wikijump.localhost,
www.bar.wikijump.localhost {
	redir https://example.com{uri}
}

example.com {
	vars {
		site_id 2
		site_slug bar
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.example.com {
	redir https://example.com{uri}
}

#
# FILES
#

(serve_files) {
	encode
	reverse_proxy http://wws:7000
}

*.wjfiles.localhost {
	@foo host foo.wjfiles.localhost
	vars @foo site_id 1

	@bar host bar.wjfiles.localhost
	vars @bar site_id 2

	request_header X-Wikijump-Site-Slug {labels.2}
	request_header X-Wikijump-Site-Id {vars.site_id}

	import serve_files
}

#
# FALLBACK
#

http://,
https://,
localhost {
	request_header X-Wikijump-Special-Error 1
	rewrite * /-/special-error/missing-site
	reverse_proxy http://framerail:3000
}";

const CADDYFILE_BASIC_DIFFERENT_PROXIES: &str = "\
# Global options
{
	metrics {
		per_host
	}
}

#
# MAIN
#

(serve_main) {
	# Redirect, route is on the files server
	@files {
		path /*/code/*
		path /*/html/*
		path /*/file/*  # for the /{slug}/file/{filename} convenience routes
		path /*/download/*
		path /local--files/*
		path /local--code/*
		path /local--html/*
		path /-/files/*
		path /-/file/*
		path /-/download/*
		path /-/code/*
		path /-/html/*
	}
	redir @files https://{vars.site_slug}.wjfiles.test{uri}

	# Enable default compression settings
	encode

	# Finally, proxy to framerail to get the actual HTML
	# Note, the x-wikijump-site-* headers have already been set at this point
	reverse_proxy http://web_proxy_host
}

foo.wikijump.test {
	vars {
		site_id 1
		site_slug foo
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.foo.wikijump.test {
	redir https://foo.wikijump.test{uri}
}

bar.wikijump.test,
www.bar.wikijump.test {
	redir https://example.com{uri}
}

example.com {
	vars {
		site_id 2
		site_slug bar
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.example.com {
	redir https://example.com{uri}
}

#
# FILES
#

(serve_files) {
	encode
	reverse_proxy http://wws_proxy_host
}

*.wjfiles.test {
	@foo host foo.wjfiles.test
	vars @foo site_id 1

	@bar host bar.wjfiles.test
	vars @bar site_id 2

	request_header X-Wikijump-Site-Slug {labels.2}
	request_header X-Wikijump-Site-Id {vars.site_id}

	import serve_files
}

#
# FALLBACK
#

http://,
https:// {
	request_header X-Wikijump-Special-Error 1
	rewrite * /-/special-error/missing-site
	reverse_proxy http://web_proxy_host
}";

const CADDYFILE_FULL_PROD: &str = "\
# Global options
{
	metrics {
		per_host
	}
}

#
# MAIN
#

(serve_main) {
	# Redirect, route is on the files server
	@files {
		path /*/code/*
		path /*/html/*
		path /*/file/*  # for the /{slug}/file/{filename} convenience routes
		path /*/download/*
		path /local--files/*
		path /local--code/*
		path /local--html/*
		path /-/files/*
		path /-/file/*
		path /-/download/*
		path /-/code/*
		path /-/html/*
	}
	redir @files https://{vars.site_slug}.wjfiles.test{uri}

	# Enable default compression settings
	encode

	# Finally, proxy to framerail to get the actual HTML
	# Note, the x-wikijump-site-* headers have already been set at this point
	reverse_proxy http://framerail:3000
}

wikijump.test {
	vars {
		site_id 1
		site_slug www
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.wikijump.test {
	redir https://wikijump.test{uri}
}

empty.wikijump.test {
	vars {
		site_id 2
		site_slug empty
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.empty.wikijump.test {
	redir https://empty.wikijump.test{uri}
}

mytest.wikijump.test {
	vars {
		site_id 3
		site_slug mytest
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.mytest.wikijump.test {
	redir https://mytest.wikijump.test{uri}
}

example.com,
www.example.com {
	redir https://mytest.wikijump.test{uri}
}

example.net,
www.example.net {
	redir https://mytest.wikijump.test{uri}
}

check.wikijump.test,
www.check.wikijump.test {
	redir https://mytest.wikijump.test{uri}
}

wanderers-library.wikijump.test,
www.wanderers-library.wikijump.test {
	redir https://wandererslibrary.com{uri}
}

wandererslibrary.com {
	vars {
		site_id 4
		site_slug wanderers-library
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.wandererslibrary.com {
	redir https://wandererslibrary.com{uri}
}

scp-wiki.wikijump.test,
www.scp-wiki.wikijump.test {
	redir https://scpwiki.com{uri}
}

scpwiki.com {
	vars {
		site_id 5
		site_slug scp-wiki
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.scpwiki.com {
	redir https://scpwiki.com{uri}
}

scp-wiki.net,
www.scp-wiki.net {
	redir https://scpwiki.com{uri}
}

scp.foundation,
www.scp.foundation {
	redir https://scpwiki.com{uri}
}

foundation.scp,
www.foundation.scp {
	redir https://scpwiki.com{uri}
}

scpwiki.wikijump.test,
www.scpwiki.wikijump.test {
	redir https://scpwiki.com{uri}
}

#
# FILES
#

(serve_files) {
	encode
	reverse_proxy http://wws:7000
}

*.wjfiles.test {
	@www host www.wjfiles.test
	vars @www site_id 1

	@empty host empty.wjfiles.test
	vars @empty site_id 2

	@mytest host mytest.wjfiles.test
	vars @mytest site_id 3

	@wanderers-library host wanderers-library.wjfiles.test
	vars @wanderers-library site_id 4

	@scp-wiki host scp-wiki.wjfiles.test
	vars @scp-wiki site_id 5

	request_header X-Wikijump-Site-Slug {labels.2}
	request_header X-Wikijump-Site-Id {vars.site_id}

	import serve_files
}

#
# FALLBACK
#

http://,
https:// {
	request_header X-Wikijump-Special-Error 1
	rewrite * /-/special-error/missing-site
	reverse_proxy http://framerail:3000
}";

const CADDYFILE_LONG_DOMAIN: &str = "\
# Global options
{
	metrics {
		per_host
	}
}

#
# MAIN
#

(serve_main) {
	# Redirect, route is on the files server
	@files {
		path /*/code/*
		path /*/html/*
		path /*/file/*  # for the /{slug}/file/{filename} convenience routes
		path /*/download/*
		path /local--files/*
		path /local--code/*
		path /local--html/*
		path /-/files/*
		path /-/file/*
		path /-/download/*
		path /-/code/*
		path /-/html/*
	}
	redir @files https://{vars.site_slug}.wjfiles.host.site.somedomain.example.com{uri}

	# Enable default compression settings
	encode

	# Finally, proxy to framerail to get the actual HTML
	# Note, the x-wikijump-site-* headers have already been set at this point
	reverse_proxy http://framerail:3000
}

foo.site.wikijump.com {
	vars {
		site_id 1
		site_slug foo
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.foo.site.wikijump.com {
	redir https://foo.site.wikijump.com{uri}
}

bar.site.wikijump.com,
www.bar.site.wikijump.com {
	redir https://example.com{uri}
}

example.com {
	vars {
		site_id 2
		site_slug bar
	}

	request_header X-Wikijump-Site-Id {vars.site_id}
	request_header X-Wikijump-Site-Slug {vars.site_slug}

	import serve_main
}

www.example.com {
	redir https://example.com{uri}
}

#
# FILES
#

(serve_files) {
	encode
	reverse_proxy http://wws:7000
}

*.wjfiles.host.site.somedomain.example.com {
	@foo host foo.wjfiles.host.site.somedomain.example.com
	vars @foo site_id 1

	@bar host bar.wjfiles.host.site.somedomain.example.com
	vars @bar site_id 2

	request_header X-Wikijump-Site-Slug {labels.6}
	request_header X-Wikijump-Site-Id {vars.site_id}

	import serve_files
}

#
# FALLBACK
#

http://,
https:// {
	request_header X-Wikijump-Special-Error 1
	rewrite * /-/special-error/missing-site
	reverse_proxy http://framerail:3000
}";

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
            let expected = $expected;

            // Strip off trailing newlines, not something we care about,
            // and precisely managing them is a waste of time.
            while actual.ends_with('\n') {
                actual.pop();
            }

            // Meanwhile, if the 'expected' string ends with newline(s),
            // it's never going to match the above.
            // Such constant strings should be fixed.
            assert!(
                !expected.ends_with('\n'),
                "Expected test string {} ends in a newline! Fix the test case.",
                stringify!($expected),
            );

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
