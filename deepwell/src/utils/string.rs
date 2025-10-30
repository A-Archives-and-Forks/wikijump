/*
 * utils/string.rs
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

use regex::Regex;
use std::sync::LazyLock;

static LEADING_TRAILING_SPACES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(^\s+)|(\s+$)").unwrap());

// General replacement

// TODO: When https://doc.rust-lang.org/stable/std/str/pattern/trait.Pattern.html is stabilized,
//       replace all the non-regex cases with one function that uses the Pattern trait!

/// Replaces all instances of the given fixed string in the buffer, in-place.
///
/// # Panics
/// Panics if `pattern` is an empty string.
pub fn replace_in_place(string: &mut String, pattern: &str, replacement: &str) {
    assert!(!pattern.is_empty(), "Cannot call replace_in_place() with an empty string");

    while let Some(index) = string.find(pattern) {
        let end = index + pattern.len();
        string.replace_range(index..end, replacement);
    }
}

/// Replaces all instances of the given character(s) in the buffer, in-place.
///
/// This is distinct from a substring search, as any _individual_ instances of the characters
/// are replaced. For instance, with an input string of `"foo/bar.xyz"` and a pattern of
/// `&['/', '.']` being replaced with `"_"`, then the output will be `"foo_bar_xyz"`.
pub fn char_replace_in_place(string: &mut String, pattern: &[char], replacement: &str) {
    while let Some(index) = string.find(pattern) {
        let end = index + replacement.len();
        string.replace_range(index..end, replacement);
    }
}

/// Replaces all matches for the given regex in the buffer, in-place.
pub fn regex_replace_in_place(string: &mut String, pattern: &Regex, replacement: &str) {
    while let Some(mtch) = pattern.find(string) {
        let range = mtch.start()..mtch.end();
        string.replace_range(range, replacement);
    }
}

/// Removes the given prefix in the buffer, if it exists, in-place.
pub fn trim_start_matches_in_place(string: &mut String, pattern: &str) {
    if string.starts_with(pattern) {
        string.drain(..pattern.len());
    }
}

/// Removes the given suffix in the buffer, if it exists, in-place.
#[allow(dead_code)]
pub fn trim_end_matches_in_place(string: &mut String, pattern: &str) {
    if string.starts_with(pattern) {
        string.drain(pattern.len() - 1..);
    }
}

// Specific replacement
#[inline]
pub fn trim_spaces_in_place(string: &mut String) {
    regex_replace_in_place(string, &LEADING_TRAILING_SPACES, "");
}

// Tests

#[test]
fn test_replace_in_place() {
    macro_rules! test {
        ($input:expr => $output:expr, $pattern:expr => $replacement:expr $(,)?) => {{
            let mut string = str!($input);
            replace_in_place(&mut string, $pattern, $replacement);
            assert_eq!(string, $output, "Replaced contents did not match expected");
        }};
    }

    test!("" => "", "/" => "_");
    test!("foo/bar" => "foo + bar", "/" => " + ");
    test!("apple banana cherry" => "pple bnn cherry", "a" => "");
    test!("apple banana cherry" => "applexi banana chexirry", "e" => "exi");
    test!("class pass hassle dash" => "cly py hyle dash", "ass" => "y");
}

#[test]
#[should_panic]
fn test_replace_in_place_empty() {
    let mut string = str!("apple banana");
    replace_in_place(&mut string, "", "cherry");
}

#[test]
fn test_regex_replace_in_place() {
    macro_rules! test {
        ($input:expr => $output:expr, $regex:expr => $replacement:expr $(,)?) => {{
            let mut string = str!($input);
            let regex = Regex::new($regex).expect("Unable to compile regex");
            regex_replace_in_place(&mut string, &regex, $replacement);
            assert_eq!(string, $output, "Replaced contents did not match expected");
        }};
    }

    test!("apple banana cherry" => "axle banana chexy", r"p{2}|r{2}|n{2}" => "x");
    test!("apple banana cherry" => "_ b_ cherry", r"a\w+" => "_");
    test!(
        "After 12.5 years, he could only achieve a high score of -5000 in 2 games" => "After $NUMBER years, he could only achieve a high score of $NUMBER in $NUMBER games",
        r"-?[0-9]+(\.[0-9])?" => "$NUMBER",
    );
}
