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
        if !download_purl_fst(&fst_path, version.clone()) {
            // Fallback to local purls.fst if download fails
            if let Ok(content) = std::fs::read("purls.fst") {
                let _ = write(&fst_path, &content);
                println!("cargo::warning=Using local purls.fst file as fallback");
            } else {
                println!("cargo::error=Failed to download and no local purls.fst found");
            }
        }
    }

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.toml");
}

fn download_purl_fst(path: &Path, version: String) -> bool {
    let url = &format!(
        "https://raw.githubusercontent.com/aboutcode-org/purl-validator.rs/refs/tags/v{}/purls.fst",
        version
    );

    match get(url) {
        Ok(response) => {
            let status = response.status();

            if status.is_success() {
                match response.bytes() {
                    Ok(content) => {
                        let _ = write(path, &content);
                        true
                    }
                    Err(e) => {
                        println!("cargo::warning=Failed to read response body: {}", e);
                        false
                    }
                }
            } else {
                println!(
                    "cargo::warning=Failed to fetch purls.fst: {}",
                    response.status()
                );
                false
            }
        }
        Err(e) => {
            println!("cargo::warning=Failed to request purls.fst: {}", e);
            false
        }
    }
}
