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
use std::path::PathBuf;

static FST_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/purls.fst"));

fn load_purl_fst_data() -> &'static [u8] {
    FST_DATA
}

fn should_fetch_latest() -> bool {
    env::var("PURL_VALIDATOR_FETCH_LATEST")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false)
}

#[allow(dead_code)]
fn cache_path() -> PathBuf {
    env::temp_dir().join("purl-validator-cache.fst")
}

#[cfg(feature = "fetch-latest")]
fn load_cached_fst() -> Option<Vec<u8>> {
    let path = cache_path();
    std::fs::read(path).ok()
}

#[cfg(not(feature = "fetch-latest"))]
fn load_cached_fst() -> Option<Vec<u8>> {
    None
}

#[cfg(feature = "fetch-latest")]
fn fetch_and_cache_fst() -> Option<Vec<u8>> {
    let version = env!("CARGO_PKG_VERSION");
    let url = format!(
        "https://raw.githubusercontent.com/aboutcode-org/purl-validator.rs/refs/tags/v{}/purls.fst",
        version
    );

    match reqwest::blocking::get(&url) {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes() {
                    Ok(bytes) => {
                        let data = bytes.to_vec();
                        if let Err(_) = std::fs::write(cache_path(), &data) {
                            // Cache write failed, but we still have the data in memory
                        }
                        return Some(data);
                    }
                    Err(_) => {
                        return None;
                    }
                }
            }
            None
        }
        Err(_) => None,
    }
}

#[cfg(not(feature = "fetch-latest"))]
fn fetch_and_cache_fst() -> Option<Vec<u8>> {
    None
}

static VALIDATOR: Lazy<Set<Vec<u8>>> = Lazy::new(|| {
    let fst_data = if should_fetch_latest() {
        // Try to load from cache first
        if let Some(cached_data) = load_cached_fst() {
            cached_data
        } else if let Some(fetched_data) = fetch_and_cache_fst() {
            // Fetch and cache succeeded
            fetched_data
        } else {
            // Fallback to embedded data
            load_purl_fst_data().to_vec()
        }
    } else {
        // Use embedded data by default
        load_purl_fst_data().to_vec()
    };

    Set::new(fst_data).expect("Failed to load FST")
});

fn strip_and_check_purl(packageurl: &str, fst_map: &Set<Vec<u8>>) -> bool {
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
///
/// By default, uses the embedded (offline) PURL database. To enable runtime fetching of the
/// latest PURL data, set the `PURL_VALIDATOR_FETCH_LATEST` environment variable to `true`
/// (only works if the `fetch-latest` feature is enabled).
pub fn validate(packageurl: &str) -> bool {
    strip_and_check_purl(packageurl, &VALIDATOR)
}

#[cfg(test)]
mod validate_tests;
