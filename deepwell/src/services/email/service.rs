/*
 * services/email/service.rs
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

use super::prelude::*;

#[derive(Debug)]
pub struct EmailService;

impl EmailService {
    /// Validates an email through the MailCheck API.
    pub async fn validate(email: &str) -> Result<EmailValidationOutput> {
        if email.is_empty() {
            bail!(Error::new(
                "cannot validate empty email string",
                ErrorType::BadRequest,
            ));
        }

        let make_error = || {
            Error::new(
                format!("failed to validate email '{email}'"),
                ErrorType::EmailVerification,
            )
        };

        // Sends a GET request to the MailCheck API and deserializes the response.
        let mailcheck = reqwest::get(format!("https://api.mailcheck.ai/email/{email}"))
            .await
            .or_raise(make_error)?
            .json::<MailCheckResponse>()
            .await
            .or_raise(make_error)?;

        let mailcheck = match mailcheck {
            MailCheckResponse::Success(data) => data,
            MailCheckResponse::Failure(MailCheckFailureResponse { status, error }) => {
                match status {
                    // Invalid request, bad email
                    400 => {
                        error!(
                            "MailCheck API request failed with bad response: {}",
                            error,
                        );
                        bail!(Error::new(
                            format!(
                                "failed to validate email, MailCheck API returned an error: {}",
                                error,
                            ),
                            ErrorType::EmailVerification,
                        ));
                    }

                    // Exceeded rate limit
                    429 => {
                        error!("MailCheck API hit ratelimit: {}", error);
                        bail!(Error::new(
                            "failed to validate email, MailCheck API hit ratelimit",
                            ErrorType::RateLimited,
                        ));
                    }

                    // Other statuses.
                    _ => {
                        error!(
                            "MailCheck API returned error status {}: {}",
                            status, error,
                        );
                        bail!(Error::new(
                            format!(
                                "failed to validate email, unexpected status {} from MailCheck: {}",
                                status, error
                            ),
                            ErrorType::EmailVerification
                        ));
                    }
                }
            }
        };

        if mailcheck.status != 200 {
            error!(
                "MailCheck API returned non-success status {} with no error message",
                mailcheck.status,
            );
            bail!(Error::new(
                format!(
                    "failed to validate email, unexpected non-success status {} from MailCheck, but no error message",
                    mailcheck.status,
                ),
                ErrorType::EmailVerification,
            ));
        }

        // Prepare output fields
        let mut valid = true;
        let mut classification = EmailClassification::Normal;
        let mut provider_classification = EmailProviderClassification::KnownProvider;
        let mut normalized_email = None;

        // Check if the email is a role email
        // This is for addresses like "info@example.com" or "webmaster@example.com"
        // which are likely managed by multiple people.
        if mailcheck.role_account {
            classification = EmailClassification::Role;
        }

        // Check if the email is an alias.
        // This was previously a boolean, now it is a derived property
        // from comparing the email with its derived counterpart.
        if mailcheck.email != mailcheck.normalized_email {
            classification = EmailClassification::Alias;
            normalized_email = Some(mailcheck.normalized_email);
        }

        if mailcheck.disposable {
            valid = false;
            classification = EmailClassification::Disposable;
        }

        // Check if the domain has any MX records.
        // If not, it's not a valid email (we cannot send anything there)
        if !mailcheck.mx {
            valid = false;
            classification = EmailClassification::Invalid;
        }

        if mailcheck.spam {
            valid = false;
            classification = EmailClassification::Spam;
        }

        // Determine email provider classification if no mx_providers
        if mailcheck.mx_providers.is_empty() {
            if mailcheck.public_domain {
                provider_classification = EmailProviderClassification::PublicEmail;
            } else {
                if mailcheck.mx {
                    provider_classification = EmailProviderClassification::SelfHosted;
                } else {
                    provider_classification = EmailProviderClassification::NoProvider;
                }
            }
        }

        // Move other fields to output
        Ok(EmailValidationOutput {
            valid,
            classification,
            provider_classification,
            email: mailcheck.email,
            normalized_email,
            domain: mailcheck.domain,
            domain_age_in_days: mailcheck.domain_age_in_days,
            mx: mailcheck.mx,
            mx_records: mailcheck.mx_records,
            mx_providers: mailcheck.mx_providers,
            disposable: mailcheck.disposable,
            public_domain: mailcheck.public_domain,
            relay_domain: mailcheck.relay_domain,
            role_account: mailcheck.role_account,
            spam: mailcheck.spam,
            did_you_mean: mailcheck.did_you_mean,
        })
    }
}
