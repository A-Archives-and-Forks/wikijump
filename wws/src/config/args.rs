/*
 * config/args.rs
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

use crate::info;
use clap::builder::{BoolishValueParser, NonEmptyStringValueParser};
use clap::{value_parser, Arg, ArgAction, Command};
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, Clone)]
pub struct Arguments {
    pub enable_trace: bool,
    pub address: SocketAddr,
}

impl Default for Arguments {
    fn default() -> Arguments {
        Arguments {
            enable_trace: true,
            address: "[::]:80".parse().unwrap(),
        }
    }
}

impl Arguments {
    pub fn parse() -> Self {
        let mut matches = Command::new("wws")
            .author(info::PKG_AUTHORS)
            .version(info::PKG_VERSION)
            .about(info::PKG_DESCRIPTION)
            .arg(
                Arg::new("disable-trace")
                    .short('q')
                    .long("quiet")
                    .long("disable-trace")
                    .action(ArgAction::SetTrue)
                    .help("Disable trace output."),
            )
            .arg(
                Arg::new("host")
                    .short('H')
                    .long("host")
                    .long("hostname")
                    .value_name("HOST")
                    .value_parser(value_parser!(IpAddr))
                    .action(ArgAction::Set)
                    .help("What host to listen on."),
            )
            .arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .value_name("PORT")
                    .value_parser(value_parser!(u16))
                    .action(ArgAction::Set)
                    .help("What port to listen on."),
            )
            .get_matches();

        let mut args = Arguments::default();

        if matches.remove_one::<bool>("disable-trace") == Some(true) {
            args.enable_trace = false;
        }

        if let Some(value) = matches.remove_one::<IpAddr>("host") {
            args.address.set_ip(value);
        }

        if let Some(value) = matches.remove_one::<u16>("port") {
            args.address.set_port(value);
        }

        args
    }
}
