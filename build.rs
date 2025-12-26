/*

Copyright (c) nexB Inc. and others. All rights reserved.
ScanCode is a trademark of nexB Inc.
SPDX-License-Identifier: Apache-2.0
See http://www.apache.org/licenses/LICENSE-2.0 for the license text.
See https://github.com/aboutcode-org/purl-validator-rust for support or download.
See https://aboutcode.org for more information about nexB OSS projects.

*/

use reqwest::blocking::get;
use std::env;
use std::fs::write;
use std::path::Path;

fn main() {
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let fst_path = Path::new(&out_dir).join("purls.fst");

    if !fst_path.exists() {
        println!("cargo::warning=Downloading PURL v{} FST map.", version);
        download_purl_fst(&fst_path, version);
    }

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.toml");
}

fn download_purl_fst(path: &Path, version: String) {
    let url = &format!(
        "https://raw.githubusercontent.com/aboutcode-org/purl-validator.rs/refs/tags/v{}/purls.fst",
        version
    );

    match get(url) {
        Ok(response) => {
            let status = response.status();

            if status.is_success() {
                let content = response.bytes().expect("Failed to read response body");
                write(path, &content).expect("Failed to write");
            } else {
                println!(
                    "cargo::error=Failed to fetch purls.fst: {}",
                    response.status()
                );
            }
        }
        Err(e) => println!("cargo::error=Failed to request: {}", e),
    }
}
