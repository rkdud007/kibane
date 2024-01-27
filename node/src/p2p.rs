use std::sync::Arc;

use crate::store::Store;

#[derive(Debug)]
pub struct Peer2Peer {
    pub store: Arc<Store>,
}
impl Peer2Peer {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }
}
