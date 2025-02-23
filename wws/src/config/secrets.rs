/*
 * config/secrets.rs
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

use axum_client_ip::SecureClientIpSource;
use s3::{creds::Credentials, region::Region};

#[derive(Debug, Clone)]
pub struct Secrets {
    /// The URL of the DEEPWELL server to connect to.
    ///
    /// Set using environment variable `DEEPWELL_URL`.
    pub deepwell_url: String,

    /// The URL of the Redis cache to connect to.
    ///
    /// Set using environment variable `REDIS_URL`.
    pub redis_url: String,

    /// The host of the framerail server to reverse proxy from.
    /// This includes the port number, if it's not `80`.
    ///
    /// Set using environment variable `FRAMERAIL_HOST`.
    pub framerail_host: String,

    /// The name of the S3 bucket that file blobs are kept in.
    /// The bucket must already exist prior to program invocation.
    ///
    /// Set using environment variable `S3_BUCKET`.
    pub s3_bucket: String,

    /// The region to use for S3.
    ///
    /// Set using environment variable `S3_AWS_REGION` if standard,
    /// or `S3_REGION_NAME` and `S3_CUSTOM_ENDPOINT` if custom.
    pub s3_region: Region,

    /// Whether to use path style for S3.
    ///
    /// Set using environment variable `S3_PATH_STYLE`.
    pub s3_path_style: bool,

    /// The credentials to use for S3.
    ///
    /// Set using environment variable `S3_ACCESS_KEY_ID` and `S3_SECRET_ACCESS_KEY`.
    ///
    /// Alternatively you can have it read from the AWS credentials file.
    /// The profile to read from can be set in the `AWS_PROFILE_NAME` environment variable.
    pub s3_credentials: Credentials,

    /// Specify how client IP addresses are determined.
    ///
    /// In the crate `axum-client-ip`, you need to specify hoow `SecureClientIp` sources its
    /// information, since it depends on the exact stack your web application is in.
    ///
    /// Set using environment variable `CLIENT_IP_SOURCE`, must have one of the following values:
    /// (see [`SecureClientIpSource`])
    /// * `RightmostForwarded`
    /// * `RightmostXForwardedFor`
    /// * `XRealIp`
    /// * `FlyClientIp`
    /// * `TrueClientIp`
    /// * `CfConnectingIp`
    /// * `ConnectInfo`
    /// * `CloudFrontViewerAddress`
    ///
    /// [`SecureClientIpSource`]: https://docs.rs/axum-client-ip/latest/axum_client_ip/enum.SecureClientIpSource.html))
    pub client_ip_source: SecureClientIpSource,
}
