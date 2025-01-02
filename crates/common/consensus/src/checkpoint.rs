use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Encode, Decode, TreeHash)]
pub struct Checkpoint {
    pub epoch: u64,
    pub root: B256,
}
