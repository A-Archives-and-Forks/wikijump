/*
 * main.rs
 *
 * Wilson's Web Server - Serves a zoo of content (framerail, user files, code, etc)
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

//! A server to handle incoming web requests.
//!
//! Depending on the hostname, requests are routed to either framerail
//! or given to logic to serve wjfiles data.

#[macro_use]
extern crate str_macro;

#[macro_use]
extern crate tracing;

#[macro_use]
mod macros;

mod cache;
mod config;
mod deepwell;
mod error;
mod handler;
mod host;
mod info;
mod path;
mod route;
mod state;
mod trace;

use self::config::load_config;
use self::route::build_router;
use self::state::build_server_state;
use self::trace::setup_tracing;
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::process;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let (config, secrets) = load_config();

    // Set up tracing
    if config.enable_trace {
        setup_tracing();
    }

    // Write PID file
    if let Some(ref path) = config.pid_file {
        debug!(pid = process::id(), "Writing PID file");
        let mut file = File::create(path)?;
        writeln!(&mut file, "{}", process::id())?;
    }

    // Connect to services, build server state and then run
    let state = build_server_state(secrets).await?;
    let app = build_router(state);
    let listener = TcpListener::bind(config.address).await?;

    // Begin listening
    info!(
        address = str!(config.address),
        "Listening to connections...",
    );

    axum::serve(listener, app).await?;
    Ok(())
}
