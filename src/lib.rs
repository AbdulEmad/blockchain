type BlockHash = Vec<u8>;
type Address = String;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u128 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

pub fn difficulty_bytes_as_u128(v: &[u8]) -> u128 {
    assert!(v.len() >= 32, "Not enought bytes in the input vector");

    let mut buf = [0u8; 16];
    buf.copy_from_slice(&v[16..32]);

    u128::from_le_bytes(buf)
}

mod block;
pub use crate::block::Block;

mod hashable;
pub use crate::hashable::Hashable;

mod blockchain;
pub use crate::blockchain::Blockchain;

pub mod transaction;
pub use crate::transaction::Transaction;
