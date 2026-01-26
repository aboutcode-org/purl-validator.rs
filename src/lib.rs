/*

Copyright (c) nexB Inc. and others. All rights reserved.
ScanCode is a trademark of nexB Inc.
SPDX-License-Identifier: Apache-2.0
See http://www.apache.org/licenses/LICENSE-2.0 for the license text.
See https://github.com/aboutcode-org/purl-validator-rust for support or download.
See https://aboutcode.org for more information about nexB OSS projects.

*/

//! A library to validate whether a PURL actually exists.
//!
//! **purl-validator** is a Rust library for validating
//! [`Package URLs` (PURLs)](https://github.com/package-url/purl-spec).
//! It works fully offline, including in **air-gapped** or **restricted environments**,
//! and answers one key question: **Does the package this PURL represents actually exist?**
//!
//!
//! # Examples
//!
//! Simplest way to use `validate` is as follows:
//!
//! ```
//! use purl_validator::validate;
//!
//! let result: bool = validate("pkg:nuget/FluentValidation");
//! ```
//!

use fst::Set;

use once_cell::sync::Lazy;
use std::env;

mod runtime;

static FST_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/purls.fst"));

/// Decide whether runtime mode is enabled.
///
/// Controlled by environment variable:
///   PURL_VALIDATOR_FETCH_LATEST=1  or  true
fn runtime_mode_enabled() -> bool {
    match env::var("PURL_VALIDATOR_FETCH_LATEST") {
        Ok(value) => value == "1" || value.eq_ignore_ascii_case("true"),
        Err(_) => false,
    }
}

/// Load FST bytes from the appropriate source.
///
/// Policy:
/// - Default: bundled FST
/// - If runtime mode enabled:
///     - Try runtime FST from disk
///     - Fallback to bundled on any failure
fn load_fst_bytes() -> &'static [u8] {
    if runtime_mode_enabled() {
        if let Some(bytes) = runtime::try_load_runtime_fst_bytes() {
            return bytes;
        }
        // Fallback to bundled if runtime fails
        FST_DATA
    } else {
        FST_DATA
    }
}

static VALIDATOR: Lazy<Set<&'static [u8]>> = Lazy::new(|| {
    let bytes = load_fst_bytes();
    Set::new(bytes).expect("Failed to load FST from embedded bytes")
});

fn strip_and_check_purl(packageurl: &str, fst_map: &Set<&[u8]>) -> bool {
    let trimmed_packageurl = packageurl.trim_end_matches("/");
    fst_map.contains(trimmed_packageurl)
}

/// Validate a Package URL (PURL)
///
/// Returns `true` if the given base PURL represents an existing package,
/// otherwise returns `false`.
///
/// Use pre-built FST (Finite State Transducer) to perform lookups and confirm whether
/// the **base PURL** exists.
pub fn validate(packageurl: &str) -> bool {
    strip_and_check_purl(packageurl, &VALIDATOR)
}

#[cfg(test)]
mod validate_tests;
