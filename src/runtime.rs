use memmap2::Mmap;
use once_cell::sync::OnceCell;
use std::fs::File;
use std::path::PathBuf;

/// Global storage for runtime FST bytes.
///
/// This ensures the bytes live for the entire program lifetime,
/// which is required because VALIDATOR borrows them as 'static.
static RUNTIME_FST: OnceCell<Mmap> = OnceCell::new();

/// Try to load runtime FST bytes from disk.
///
/// Returns:
/// - Some(&'static [u8]) if runtime FST is available
/// - None if not available or any error occurs
pub fn try_load_runtime_fst_bytes() -> Option<&'static [u8]> {
    // Initialize only once
    let mmap = RUNTIME_FST.get_or_try_init(|| {
        let path = runtime_fst_path();

        if !path.exists() {
            return Err("runtime FST not found");
        }

        let file = File::open(&path).map_err(|_| "failed to open runtime FST")?;

        // Safety:
        // - File is not modified after creation
        // - Mmap lives for entire program lifetime (stored in OnceCell)
        let mmap = unsafe { Mmap::map(&file).map_err(|_| "failed to mmap runtime FST")? };

        Ok(mmap)
    });

    match mmap {
        Ok(mmap) => Some(&mmap[..]),
        Err(_) => None,
    }
}

/// Compute the path of the runtime FST cache file.
///
/// Current design:
///   $HOME/.cache/purl-validator/purls.fst
fn runtime_fst_path() -> PathBuf {
    let mut base = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    base.push(".cache");
    base.push("purl-validator");
    base.push("purls.fst");

    base
}
