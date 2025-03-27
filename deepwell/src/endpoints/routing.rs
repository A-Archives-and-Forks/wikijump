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
use sea_orm::{EntityTrait, QuerySelect};
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
        .column(site::Column::CustomDomain)
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

    let main_domain = &config.main_domain;
    let files_domain = &config.files_domain;

    let mut caddyfile = str!(
        "\
# Globals option
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

    str_writeln!(
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

	reverse_proxy http://{framerail_host}
}}
"
    );

    // TODO generate main site sections

    str_writeln!(
        &mut caddyfile,
        "
#
# FILES
#

(serve_files) {{
	reverse_proxy http://{wws_host}
}}

*{files_domain} {{
"
    );

    // TODO generate *.wjfiles.com interior

    str_writeln!(
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
