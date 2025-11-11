use fst::Set;
use memmap2::Mmap;
use once_cell::sync::Lazy;
use std::env;
use std::fs::File;
use std::path::Path;

static VALIDATOR: Lazy<Set<Mmap>> = Lazy::new(|| {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("purls.fst");
    let file = File::open(path).expect("Failed to open FST file");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to mmap FST file") };
    Set::new(mmap).expect("Failed to load FST from mmap")
});

pub fn validate(packageurl: &str) -> bool {
    let trimmed_packageurl = packageurl.trim_end_matches("/");
    VALIDATOR.contains(trimmed_packageurl)
}
