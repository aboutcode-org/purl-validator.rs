/*

Copyright (c) nexB Inc. and others. All rights reserved.
ScanCode is a trademark of nexB Inc.
SPDX-License-Identifier: Apache-2.0
See http://www.apache.org/licenses/LICENSE-2.0 for the license text.
See https://github.com/aboutcode-org/purl-validator-rust for support or download.
See https://aboutcode.org for more information about nexB OSS projects.

*/

use super::*;
use fst::Set;
use std::fs;
use std::path::Path;

#[test]
fn test_validate_with_custom_file() {
    let test_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/test_purls.fst");
    let data: Vec<u8> = fs::read(test_path).unwrap();
    let validator = Set::new(data).unwrap();
    assert!(strip_and_check_purl(
        "pkg:nuget/FluentUtils.EnumExtensions",
        &validator
    ));
    assert!(!strip_and_check_purl("pkg:example/nonexistent", &validator));
}

#[test]
fn test_validate_with_packageurl_trailing_slash() {
    let test_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/test_purls.fst");
    let data: Vec<u8> = fs::read(test_path).unwrap();
    let validator = Set::new(data).unwrap();

    assert!(validator.contains("pkg:nuget/FluentUtils.EnumExtensions"));
    assert!(strip_and_check_purl(
        "pkg:nuget/FluentUtils.EnumExtensions/",
        &validator
    ));
}

#[test]
fn test_default_behavior_without_fetch_env() {
    // When PURL_VALIDATOR_FETCH_LATEST is not set, should use embedded data
    // We can't actually unset env vars in tests without unsafe, so we just verify
    // that validation works by default
    let result = validate("pkg:nuget/FluentValidation");
    // Just ensure it completes without panic
    let _ = result;
}

#[test]
fn test_cache_path_generation() {
    // Test that cache_path returns a valid path
    let path = cache_path();
    assert!(path.to_string_lossy().contains("purl-validator-cache.fst"));
}

#[test]
fn test_validate_defaults_to_embedded() {
    // Test that validate always uses embedded FST when fetch-latest feature is not enabled
    // (or when fetch fails and falls back)
    // This should work without network access
    let result = validate("pkg:nuget/FluentValidation");
    // Just verify it completes without panic
    let _ = result;
}

#[cfg(feature = "fetch-latest")]
#[test]
fn test_fetch_latest_feature_enabled() {
    // This test only runs when fetch-latest feature is enabled
    // Verify the feature-gated functions compile correctly
    let cached = load_cached_fst();
    // cached may be None or Some, both are valid
    let _ = cached;
    
    // fetch_and_cache_fst should compile when feature is enabled
    let fetched = fetch_and_cache_fst();
    // fetched may be None or Some, both are valid
    let _ = fetched;
}

#[cfg(not(feature = "fetch-latest"))]
#[test]
fn test_fetch_latest_feature_disabled() {
    // When feature is disabled, fetch functions should return None
    let cached = load_cached_fst();
    assert!(cached.is_none(), "load_cached_fst should return None when feature is disabled");
    
    let fetched = fetch_and_cache_fst();
    assert!(fetched.is_none(), "fetch_and_cache_fst should return None when feature is disabled");
}

