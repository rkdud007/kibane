use std::sync::Arc;

use crate::store::Store;

#[derive(Debug)]
pub struct Syncer {
    store: Arc<Store>,
}

impl Syncer {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }
}
