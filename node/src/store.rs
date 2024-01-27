use crate::common::{ExtendedHeader, Hash};
use std::sync::atomic::AtomicU64;

use dashmap::DashMap;

#[derive(Debug)]
pub struct Store {
    headers: DashMap<Hash, ExtendedHeader>,
    height_to_hash: DashMap<u64, Hash>,
    height: AtomicU64,
}

impl Store {
    pub fn new() -> Self {
        Self {
            headers: DashMap::new(),
            height_to_hash: DashMap::new(),
            height: AtomicU64::new(0),
        }
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
