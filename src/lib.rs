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
//! let result: bool = validate("pkg:nuget/FluentValidation")
//!     .expect("only fails if PURL is invalid or contains version, qualifier, or subpath");
//! ```
//!

use fst::Set;
use once_cell::sync::Lazy;
use packageurl::PackageUrl;
use std::env;
use std::str::FromStr;

static FST_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/purls.fst"));

static VALIDATOR: Lazy<Set<&'static [u8]>> =
    Lazy::new(|| Set::new(FST_DATA).expect("Failed to load FST from embedded bytes"));

fn strip_and_check_purl(packageurl: &str, fst_map: &Set<&[u8]>) -> Result<bool, ValidateError> {
    let purl = PackageUrl::from_str(packageurl).map_err(ValidateError::InvalidPurl)?;
    if purl.version().is_some() || !purl.qualifiers().is_empty() || purl.subpath().is_some() {
        return Err(ValidateError::UnsupportedPurl(
            "only base PURL is supported (no version, qualifiers, or subpath)",
        ));
    }

    let trimmed_packageurl = packageurl.trim_end_matches("/");
    Ok(fst_map.contains(trimmed_packageurl))
}

/// Validate a Package URL (PURL)
///
/// Return `Ok(true)` if given **base PURL** represents an existing package,
/// `Ok(false)` if it does not, or `Err` if the PURL is invalid or contains
/// unsupported fields (version, qualifiers, or subpath).
///
/// A **base PURL** is a PURL without a version, qualifiers, or subpath.
/// PURLs containing a version, qualifiers, or subpath are **not supported**
/// and will cause the validator to return an error.
///
/// Use pre-built FST (Finite State Transducer) to perform lookups and confirm whether
/// the **base PURL** exists.
pub fn validate(packageurl: &str) -> Result<bool, ValidateError> {
    strip_and_check_purl(packageurl, &VALIDATOR)
}

#[derive(Debug)]
pub enum ValidateError {
    InvalidPurl(packageurl::Error),
    UnsupportedPurl(&'static str),
}

#[cfg(test)]
mod validate_tests;
