/*
 * config/mod.rs
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

mod args;
mod object;
mod secrets;

pub use self::object::Config;
pub use self::secrets::Secrets;

use self::args::Arguments;
use cfg_if::cfg_if;
use dotenvy::dotenv;
use ref_map::*;
use s3::{creds::Credentials, region::Region};
use std::path::PathBuf;
use std::{env, process};

pub fn load_config() -> (Config, Secrets) {
    dotenv().ok();

    // Essentially .expect(), but allows printing the environment variable name in the message.
    macro_rules! get_env {
        ($name:expr) => {
            match env::var($name) {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("Unable to read environment variable {}: {}", $name, error);
                    process::exit(1);
                }
            }
        };
    }

    // The OsString version of get_env!()
    #[cfg(feature = "tls")]
    macro_rules! get_env_os {
        ($name:expr) => {
            match env::var_os($name) {
                Some(value) => value,
                None => {
                    eprintln!("Unable to read environment variable {}", $name);
                    process::exit(1);
                }
            }
        };
    }

    // Process arguments and overrides
    let Arguments {
        enable_trace,
        mut pid_file,
        mut address,
    } = Arguments::parse();

    if let Some(value) = env::var_os("PID_FILE") {
        pid_file = Some(PathBuf::from(value));
    }

    if let Ok(value) = env::var("ADDRESS") {
        address = value.parse().expect("Unable to parse socket address");
    }

    // Process secrets
    let deepwell_url = get_env!("DEEPWELL_URL");
    let redis_url = get_env!("REDIS_URL");
    let framerail_host = get_env!("FRAMERAIL_HOST");

    let s3_bucket = get_env!("S3_BUCKET");
    let s3_region = match env::var("S3_AWS_REGION") {
        // Standard AWS S3 region, parse out into enum.
        Ok(value) => match value.parse() {
            Ok(region) => region,
            Err(error) => {
                eprintln!("S3_AWS_REGION variable is not a valid AWS region ID: {error}");
                process::exit(1);
            }
        },

        // Custom region, with a specific S3 endpoint.
        Err(_) => {
            let region = get_env!("S3_REGION_NAME");
            let endpoint = get_env!("S3_CUSTOM_ENDPOINT");

            Region::Custom { region, endpoint }
        }
    };

    let s3_path_style = match get_env!("S3_PATH_STYLE").parse() {
        Ok(path_style) => path_style,
        Err(_) => {
            eprintln!("S3_PATH_STYLE variable is not a valid boolean");
            process::exit(1);
        }
    };

    let s3_credentials = {
        // Try to read from environment
        // Reads from S3_ACCESS_KEY_ID and S3_SECRET_ACCESS_KEY
        let env_creds = Credentials::from_env_specific(
            Some("S3_ACCESS_KEY_ID"),
            Some("S3_SECRET_ACCESS_KEY"),
            None,
            None,
        );

        match env_creds {
            Ok(credentials) => credentials,
            Err(_) => {
                // Try to read from profile
                let profile_name = env::var("AWS_PROFILE_NAME").ok();
                let profile_name = profile_name.ref_map(|s| s.as_str());

                match Credentials::from_profile(profile_name) {
                    Ok(credentials) => credentials,
                    Err(error) => {
                        eprintln!("Unable to read AWS credentials file: {error}");
                        process::exit(1);
                    }
                }
            }
        }
    };

    let client_ip_source = match get_env!("CLIENT_IP_SOURCE").parse() {
        Ok(ip_source) => ip_source,
        Err(_) => {
            eprintln!("CLIENT_IP_SOURCE variable does not have a valid enum value");
            process::exit(1);
        }
    };

    cfg_if! {
        if #[cfg(feature = "tls")] {
            let tls_certificate = PathBuf::from(get_env_os!("TLS_CERTIFICATE"));
            let tls_secret_key = PathBuf::from(get_env_os!("TLS_SECRET_KEY"));
        }
    }

    // Build and return

    cfg_if! {
        if #[cfg(feature = "tls")] {
            let config = Config {
                enable_trace,
                pid_file,
                address,
                tls_certificate,
                tls_secret_key,
            };
        } else {
            let config = Config {
                enable_trace,
                pid_file,
                address,
            };
        }
    }

    let secrets = Secrets {
        deepwell_url,
        redis_url,
        framerail_host,
        s3_bucket,
        s3_region,
        s3_path_style,
        s3_credentials,
        client_ip_source,
    };

    (config, secrets)
}
