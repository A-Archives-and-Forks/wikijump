/*
 * services/caddy/service.rs
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

//! Service to handle the [Caddy webserver](https://caddyserver.com/docs/).
//!
//! This is primarily concerned with generating the `Caddyfile` that
//! powers the server, which is where host → site mapping is performed.
//!
//! NOTE: This file contains hard tabs, as this is what we want to use for
//!       `Caddyfile` generation. If you're opening this file, mind the git
//!       diff!
//!
//!       If your editor munges the tabs please discard those changes.
//!       Remember, `git add -p` is your friend!

use super::prelude::*;
use crate::models::alias::Model as AliasModel;
use crate::models::sea_orm_active_enums::AliasType;
use crate::models::site::{self, Entity as Site};
use crate::models::site_domain::{self, Entity as SiteDomain};
use crate::services::domain::DEFAULT_SITE_SLUG;
use crate::services::{AliasService, DomainService};
use sea_orm::{EntityTrait, QuerySelect};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CaddyService;

impl CaddyService {
    pub async fn generate(
        ctx: &ServiceContext<'_>,
        options: &CaddyfileOptions<'_>,
    ) -> Result<String> {
        let config = ctx.config();
        let txn = ctx.transaction();

        let make_error = || {
            Error::new(
                format!("failed to generate Caddyfile with options {:#?}", options),
                ErrorType::Caddyfile,
            )
        };

        let sites: Vec<(i64, String, Option<String>)> = Site::find()
            .select_only()
            .column(site::Column::SiteId)
            .column(site::Column::Slug)
            .column(site::Column::PreferredDomain)
            .order_by_asc(site::Column::SiteId)
            .into_tuple()
            .all(txn)
            .await
            .or_raise(make_error)?;

        let domains = {
            let mut domains = HashMap::with_capacity(sites.len());

            for &(site_id, _, _) in &sites {
                let aliases = AliasService::get_all(ctx, AliasType::Site, site_id)
                    .await
                    .or_raise(make_error)?
                    .into_iter()
                    .map(|AliasModel { slug, .. }| slug)
                    .collect();

                let custom_domains = SiteDomain::find()
                    .order_by_asc(site_domain::Column::Domain)
                    .select_only()
                    .column(site_domain::Column::Domain)
                    .filter(site_domain::Column::SiteId.eq(site_id))
                    .into_tuple()
                    .all(txn)
                    .await
                    .or_raise(make_error)?;

                domains.insert(
                    site_id,
                    SiteDomainData {
                        aliases,
                        custom_domains,
                    },
                );
            }

            domains
        };

        Ok(Self::generate_custom(
            config,
            options,
            &SiteData { sites, domains },
        ))
    }

    pub fn generate_custom(
        config: &Config,
        CaddyfileOptions {
            debug,
            local,
            http_port,
            https_port,
            deploy_host,
            framerail_host,
            wws_host,
        }: &CaddyfileOptions<'_>,
        SiteData { sites, domains }: &SiteData,
    ) -> String {
        info!("Generating Caddyfile for {} sites", sites.len());

        let files_domain = &config.files_domain;
        let files_domain_no_dot = &config.files_domain_no_dot;
        let main_domain = &config.main_domain;
        let main_domain_no_dot = &config.main_domain_no_dot;

        let mut caddyfile = str!(
            "\
# Global options
{
	metrics {
		per_host
	}
"
        );

        if let Some(port) = *http_port {
            str_writeln!(&mut caddyfile, "\thttp_port {port}");
        }

        if let Some(port) = *https_port {
            str_writeln!(&mut caddyfile, "\thttps_port {port}");
        }

        if *debug {
            str_writeln!(&mut caddyfile, "\tdebug");
        }

        if *local {
            str_writeln!(&mut caddyfile, "\tlocal_certs\n\tskip_install_trust");
        }

        str_write!(
            &mut caddyfile,
            "\
}}

(strip_headers) {{
	# Strip internal headers used by Wikijump
	request_header -X-Wikijump-*
}}

"
        );

        if let Some(deploy_host) = deploy_host {
            str_write!(
                &mut caddyfile,
                "\
#
# INFRASTRUCTURE
#

deploy{main_domain} {{
	reverse_proxy {deploy_host}
}}

deploy{files_domain} {{
	redir https://deploy{main_domain}
}}

"
            );
        }

        str_write!(
            &mut caddyfile,
            "\
#
# MAIN
#

(serve_main) {{
	# Special routes
	respond /-/health-check/caddy ✅ 200
	respond /-/teapot             🫖 418

	# wjfiles-managed routes
	# These are proxied to wws for it to handle, but shouldn't be redirected
	@proxy {{
		path /robots.txt
		path /.well-known
		path /-/health-check
	}}
	request_header @proxy X-Wikijump-Target-Server main
	reverse_proxy @proxy {wws_host}

	# Redirect, true route is on the files server
	@redirect {{
		path /*/code/*
		path /*/html/*
		path /*/file/*  # for the /{{slug}}/file/{{filename}} convenience routes
		path /*/download/*
		path /local--files/*
		path /local--code/*
		path /local--html/*
		path /-/files/*
		path /-/file/*
		path /-/download/*
		path /-/code/*
		path /-/html/*
	}}
	redir @redirect https://{{vars.site_slug}}{files_domain}{{uri}}

	# Enable default compression settings
	encode

	# Finally, proxy to framerail to get the actual HTML
	# Note, the x-wikijump-site-* headers have already been set at this point
	reverse_proxy {framerail_host}
}}
"
        );

        for (site_id, site_slug, preferred_domain) in sites {
            let SiteDomainData {
                aliases,
                custom_domains,
            } = &domains[site_id];

            // Get canonical and preferred domains, for later generation
            let canonical_domain = if site_slug == DEFAULT_SITE_SLUG {
                Cow::Borrowed(main_domain_no_dot)
            } else {
                Cow::Owned(DomainService::get_canonical(config, site_slug))
            };

            let preferred_domain: &str =
                preferred_domain.as_ref().unwrap_or(&canonical_domain);

            // Closure to generate a domain entry
            //
            // Then, generate a redirect for the corresponding "www" subdomain.
            // This shouldn't be used so we should just have it point away to
            // the right location.
            //
            // This also has the benefit of naturally capturing www.wikijump.com -> wikijump.com.
            let mut generate_entry = |domain: &str| {
                if domain == preferred_domain {
                    // Main content, for a preferred domain.
                    // This is where the request is actually reverse proxied through.
                    str_write!(
                        &mut caddyfile,
                        "
{domain} {{
	import strip_headers

	vars {{
		site_id {site_id}
		site_slug {site_slug}
	}}

	request_header X-Wikijump-Site-Id {{vars.site_id}}
	request_header X-Wikijump-Site-Slug {{vars.site_slug}}
	import serve_main
}}

www.{domain} {{
	redir https://{preferred_domain}{{uri}}
}}
"
                    );
                } else {
                    // Generate a redirect to the preferred domain.
                    str_write!(
                        &mut caddyfile,
                        "
{domain},
www.{domain} {{
	redir https://{preferred_domain}{{uri}}
}}
"
                    );
                }
            };

            // Canonical domain
            generate_entry(&canonical_domain);

            // Custom domains
            for domain in custom_domains {
                generate_entry(domain);
            }

            // Aliases (all redirects)
            for alias_slug in aliases {
                let domain = DomainService::get_canonical(config, alias_slug);
                generate_entry(&domain);
            }
        }

        str_write!(
            &mut caddyfile,
            "
#
# FILES
#

(serve_files) {{
	# Special routes
	respond /-/health-check/caddy ✅ 200
	respond /-/teapot             🫖 418

	# Enable default compression settings
	encode

	# Reverse proxy
	request_header X-Wikijump-Target-Server files
	reverse_proxy {wws_host}
}}

{files_domain_no_dot} {{
	import strip_headers
	request_header X-Wikijump-Basic-Error 1
	rewrite * /-/basic-error/file-root
	reverse_proxy {wws_host}
}}

*{files_domain} {{
	import strip_headers
"
        );

        for (site_id, site_slug, _) in sites {
            str_write!(
                &mut caddyfile,
                "
	@{site_slug} host {site_slug}{files_domain}
	vars @{site_slug} site_id {site_id}
"
            );
        }

        str_write!(
            &mut caddyfile,
            "
	request_header X-Wikijump-Site-Slug {{labels.{}}}
	request_header X-Wikijump-Site-Id {{vars.site_id}}
	import serve_files
}}
",
            site_slug_split_index(files_domain)
        );

        str_write!(
            &mut caddyfile,
            "
#
# FALLBACK
# (i.e. \"no such site\")
#

# Missing canonical domain
*{main_domain} {{
	import strip_headers
	request_header X-Wikijump-Basic-Error 1
	request_header X-Wikijump-Site-Slug {{labels.{}}}
	rewrite * /-/basic-error/site-slug
	reverse_proxy {wws_host}
}}

# Missing custom domain
{} {{
	import strip_headers
	request_header X-Wikijump-Basic-Error 1
	rewrite * /-/basic-error/site-custom
	reverse_proxy {wws_host}
}}",
            site_slug_split_index(main_domain),
            if *local {
                "http://,\nhttps://,\nlocalhost"
            } else {
                "http://,\nhttps://"
            }
        );

        caddyfile
    }
}

/// Determines the index to give to Caddy to get the site slug from a domain.
///
/// Caddy enables us to extract parts of a domain via indices, with 0 being
/// the top-level domain (TLD).
///
/// So if the files domain (with dot) is `.wjfiles.com`, there are 2 periods.
/// Any site slugs would be before that first dot, such as in `foo.wjfiles.com`,
/// which would be index 2 using Caddy's domain addressing system:
///
/// 0 - "com"
/// 1 - "wjfiles"
/// 2 - "foo"      <-- what we want
///
/// An additional example, say the domain is `.host.wikijump.example.com`,
/// then there are 4 dots in the domain, and thus the zero-based index is 4:
///
/// 0 - "com"
/// 1 - "example"
/// 2 - "wikijump"
/// 3 - "host"
/// 4 - "foo"      <-- what we want
fn site_slug_split_index(domain: &str) -> usize {
    debug_assert_eq!(
        domain.chars().next(),
        Some('.'),
        "Didn't pass a dot-prefixed domain",
    );
    domain.chars().filter(|&c| c == '.').count()
}

#[test]
fn test_site_slug_split_index() {
    const TEST_SITE_SLUG: &str = "XYZ";

    macro_rules! check {
        ($domain:expr, $expected_index:expr) => {{
            // Get and validate index
            let index = site_slug_split_index($domain);
            assert_eq!(
                index, $expected_index,
                "Caddy site slug extraction index does not match",
            );

            // Test it on a real string
            let domain = format!("{}{}", TEST_SITE_SLUG, $domain);
            let parts = domain.split('.').rev().collect::<Vec<_>>();
            assert_eq!(parts[index], TEST_SITE_SLUG, "Wrong substring extracted");
        }};
    }

    check!(".wikijump.com", 2);
    check!(".foo.example.org", 3);
    check!(".bar.foo.example.org", 4);
    check!(".alpha.beta.gamma.delta.epsilon.zeta.eta.wjfiles.com", 9);
}
