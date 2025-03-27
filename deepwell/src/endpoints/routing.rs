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

#[derive(Deserialize, Debug)]
struct CaddyfileOptions {
    #[serde(default)]
    debug: bool,

    #[serde(default)]
    local: bool,

    #[serde(default)]
    http_port: Option<i64>,

    #[serde(default)]
    https_port: Option<i64>,

    // Infra information
    framerail_host: String,
}

pub async fn generate_caddyfile(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<String> {
    // TODO split into pure function
    info!("Generating Caddyfile for current sites");

    let CaddyfileOptions {
        debug,
        local,
        http_port,
        https_port,
        framerail_host,
    } = params.parse()?;

    let config = ctx.config();
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

	reverse_proxy http://localhost:3000
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
	reverse_proxy http://{framerail_host}
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
	reverse_proxy http://localhost:3000
}}",
        if local {
            "http://,\nhttps://,\nlocalhost"
        } else {
            "http://,\nhttps://"
        }
    );

    Ok(caddyfile)
}
