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
    let data_slice: &[u8] = &data;
    let validator = Set::new(data_slice).unwrap();

    let result = strip_and_check_purl("pkg:nuget/FluentUtils.EnumExtensions", &validator).unwrap();
    assert!(result);

    let result = strip_and_check_purl("pkg:example/nonexistent", &validator).unwrap();
    assert!(!result);
}

#[test]
fn test_validate_with_packageurl_trailing_slash() {
    let test_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/test_purls.fst");
    let data: Vec<u8> = fs::read(test_path).unwrap();
    let data_slice: &[u8] = &data;
    let validator = Set::new(data_slice).unwrap();

    assert!(validator.contains("pkg:nuget/FluentUtils.EnumExtensions"));
    let result = strip_and_check_purl("pkg:nuget/FluentUtils.EnumExtensions/", &validator).unwrap();
    assert!(result);
}

#[test]
fn test_validate_with_packageurl_invalid_purl() {
    let test_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/test_purls.fst");
    let data: Vec<u8> = fs::read(test_path).unwrap();
    let data_slice: &[u8] = &data;
    let validator = Set::new(data_slice).unwrap();

    let result = strip_and_check_purl("nuget/foobar", &validator);
    assert!(matches!(result, Err(ValidateError::InvalidPurl(_))));
}

#[test]
fn test_validate_with_packageurl_unsupported_purl() {
    let test_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/test_purls.fst");
    let data: Vec<u8> = fs::read(test_path).unwrap();
    let data_slice: &[u8] = &data;
    let validator = Set::new(data_slice).unwrap();

    let result = strip_and_check_purl("pkg:nuget/FluentUtils.EnumExtensions@1.0.0", &validator);
    assert!(matches!(result, Err(ValidateError::UnsupportedPurl(_))));
}
