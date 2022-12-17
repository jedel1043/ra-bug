#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

#[cfg(feature = "std")]
pub fn data_root() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data")
}

mod baked {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/baked/mod.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/baked/any.rs"));
}
