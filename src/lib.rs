use fst::Set;
use once_cell::sync::Lazy;


static FST_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/purls.fst"));

static VALIDATOR: Lazy<Set<&[u8]>> = Lazy::new(|| {
    Set::new(FST_BYTES).expect("Failed to load FST from embedded bytes")
});


pub fn validate(word: &str) -> bool {
    VALIDATOR.contains(word)
}


#[pyo3::pymodule]
mod purl_validator {
    use pyo3::prelude::*;
    use crate::validate;

    #[pyfunction(name = "validate")]
    fn py_validate(word: &str) -> PyResult<bool> {
        Ok(validate(word))
    }
}
