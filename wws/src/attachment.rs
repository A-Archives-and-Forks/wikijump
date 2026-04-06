/*
 * attachment.rs
 *
 * Wilson's Web Server - Serves a zoo of user-generated content
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

use crate::percent::percent_encode_rfc5987;
use axum::http::HeaderValue;

pub fn content_disposition_attachment(filename: &str) -> HeaderValue {
    if filename.is_ascii() {
        let escaped = filename.replace('\\', "\\\\").replace('"', "\\\"");
        let value = format!("attachment; filename=\"{escaped}\"");
        HeaderValue::from_str(&value)
            .unwrap_or_else(|_| HeaderValue::from_static("attachment"))
    } else {
        let ascii_fallback: String = filename
            .chars()
            .map(|c| {
                if c.is_ascii_graphic() && c != '"' && c != '\\' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let encoded = percent_encode_rfc5987(filename);
        let value = format!(
            "attachment; filename=\"{ascii_fallback}\"; filename*=UTF-8''{encoded}"
        );
        HeaderValue::from_bytes(value.as_bytes())
            .unwrap_or_else(|_| HeaderValue::from_static("attachment"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------ Content-Disposition tests ------------

    #[test]
    fn disposition_ascii() {
        let val = content_disposition_attachment("report.pdf");
        assert_eq!(val.to_str().unwrap(), "attachment; filename=\"report.pdf\"");
    }

    #[test]
    fn disposition_escapes_quotes() {
        let val = content_disposition_attachment("say\"hello\".txt");
        assert_eq!(
            val.to_str().unwrap(),
            "attachment; filename=\"say\\\"hello\\\".txt\"",
        );
    }

    #[test]
    fn disposition_escapes_backslash() {
        let val = content_disposition_attachment("back\\slash.txt");
        assert_eq!(
            val.to_str().unwrap(),
            "attachment; filename=\"back\\\\slash.txt\"",
        );
    }

    #[test]
    fn disposition_non_ascii() {
        let val = content_disposition_attachment("données.csv");
        let s = std::str::from_utf8(val.as_bytes()).unwrap();
        assert!(s.contains("filename=\"donn_es.csv\""));
        assert!(s.contains("filename*=UTF-8''donn%C3%A9es.csv"));
    }
}