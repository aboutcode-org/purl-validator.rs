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

static FST_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/purls.fst"));

static VALIDATOR: Lazy<Set<&'static [u8]>> =
    Lazy::new(|| Set::new(FST_DATA).expect("Failed to load FST from embedded bytes"));

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
