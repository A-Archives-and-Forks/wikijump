/*
 * endpoints/routing.rs
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
use crate::models::alias::Model as AliasModel;
use crate::models::sea_orm_active_enums::AliasType;
use crate::models::site::{self, Entity as Site};
use crate::models::site_domain::Model as SiteDomainModel;
use crate::services::domain::DEFAULT_SITE_SLUG;
use sea_orm::{EntityTrait, QuerySelect};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct CaddyfileOptions {
    #[serde(default)]
    pub debug: bool,

    #[serde(default)]
    pub local: bool,

    #[serde(default)]
    pub http_port: Option<i64>,

    #[serde(default)]
    pub https_port: Option<i64>,

    // Infra information
    pub framerail_host: String,
    pub wws_host: String,
}

#[derive(Debug)]
pub struct SiteDomainData {
    sites: Vec<(i64, String, Option<String>)>,
    domains: HashMap<i64, (Vec<AliasModel>, Vec<SiteDomainModel>)>,
}

pub async fn caddyfile_endpoint(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<String> {
    let options: CaddyfileOptions = params.parse()?;
    let config = ctx.config();

    // Gather necessary site data
    let txn = ctx.transaction();

    let sites: Vec<(i64, String, Option<String>)> = Site::find()
        .select_only()
        .column(site::Column::SiteId)
        .column(site::Column::Slug)
        .column(site::Column::PreferredDomain)
        .into_tuple()
        .all(txn)
        .await?;

    let domains = {
        let mut extras = HashMap::with_capacity(sites.len());

        for &(site_id, _, _) in &sites {
            let site_aliases =
                AliasService::get_all(ctx, AliasType::Site, site_id).await?;

            let site_domains = DomainService::list_custom(ctx, site_id).await?;
            extras.insert(site_id, (site_aliases, site_domains));
        }

        extras
    };

    Ok(generate_caddyfile(
        config,
        options,
        &SiteDomainData { sites, domains },
    ))
}

pub fn generate_caddyfile(
    config: &Config,
    CaddyfileOptions {
        debug,
        local,
        http_port,
        https_port,
        framerail_host,
        wws_host,
    }: CaddyfileOptions,
    SiteDomainData { sites, domains }: &SiteDomainData,
) -> String {
    info!("Generating Caddyfile for {} sites", sites.len());

    let main_domain_no_dot = &config.main_domain_no_dot;
    let files_domain = &config.files_domain;

    let mut caddyfile = str!(
        "\
# Global options
{
	metrics {
		per_host
	}
"
    );

    if let Some(port) = http_port {
        str_writeln!(&mut caddyfile, "\thttp_port {port}");
    }

    if let Some(port) = https_port {
        str_writeln!(&mut caddyfile, "\thttps_port {port}");
    }

    if debug {
        str_writeln!(&mut caddyfile, "\tdebug");
    }

    if local {
        str_writeln!(&mut caddyfile, "\tskip_install_trust");
    }

    str_write!(
        &mut caddyfile,
        "\
}}

#
# MAIN
#

(serve_main) {{
	# Redirect, route is on the files server
	@files {{
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
	redir @files https://{{vars.site_slug}}{files_domain}{{uri}}

	# Finally, proxy to framerail to get the actual HTML
	# Note, the x-wikijump-site-* headers have already been set at this point
	reverse_proxy http://{framerail_host}
}}
"
    );

    for (site_id, site_slug, preferred_domain) in sites {
        let (aliases, domains) = &domains[site_id];

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
        // This also naturally has the benefit of capturing www.wikijump.com -> wikijump.com.
        let mut generate_entry = |domain: &str| {
            if domain == preferred_domain {
                // Main content, for a preferred domain.
                // This is where the request is actually reverse proxied through.
                str_write!(
                    &mut caddyfile,
                    "
{domain} {{
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
        for model in domains {
            generate_entry(&model.domain);
        }

        // Aliases (all redirects)
        for alias in aliases {
            let domain = DomainService::get_canonical(config, &alias.slug);
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
	reverse_proxy http://{wws_host}
}}

*{files_domain} {{"
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
        // What part of the domain to split
        //
        // So if the files domain (with dot) is ".wjfiles.com", there are 2 periods.
        // Any site slugs would be before that first dot, such as in "foo.wjfiles.com",
        // which would be index 2 using Caddy's domain addressing system:
        //
        // 0 - "com"
        // 1 - "wjfiles"
        // 2 - "foo"      <-- what we want
        //
        // An additional example, say the files domain is ".host.wikijump.example.com",
        // then there are 4 dots in the files domain, and thus the zero-based index is 4:
        //
        // 0 - "com"
        // 1 - "example"
        // 2 - "wikijump"
        // 3 - "host"
        // 4 - "foo"      <-- what we want
        files_domain.chars().filter(|&c| c == '.').count(),
    );

    str_write!(
        &mut caddyfile,
        "
#
# FALLBACK
#

{} {{
	request_header X-Wikijump-Special-Error 1
	rewrite * /-/special-error/missing-site
	reverse_proxy http://{framerail_host}
}}",
        if local {
            "http://,\nhttps://,\nlocalhost"
        } else {
            "http://,\nhttps://"
        }
    );

    caddyfile
}

#[test]
fn test_caddyfile_gen() {
    // TODO
    todo!()
}
