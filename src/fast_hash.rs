//! Fast deterministic hash helpers for ordered tree content.
//!
//! This module is available with the `fast-hash` feature. It intentionally
//! exposes a small byte-sink wrapper instead of committing the public API to a
//! specific third-party hasher type.

use xxhash_rust::xxh3::Xxh3;

pub const DEFAULT_SEED: u64 = 0;

#[derive(Clone)]
pub struct FastKeyHasher {
    inner: Xxh3,
}

impl Default for FastKeyHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl FastKeyHasher {
    pub fn new() -> Self {
        Self::with_seed(DEFAULT_SEED)
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            inner: Xxh3::with_seed(seed),
        }
    }

    pub fn update(&mut self, bytes: impl AsRef<[u8]>) {
        self.inner.update(bytes.as_ref());
    }

    pub fn finish64(&self) -> u64 {
        self.inner.digest()
    }

    pub fn finish128(&self) -> u128 {
        self.inner.digest128()
    }
}
