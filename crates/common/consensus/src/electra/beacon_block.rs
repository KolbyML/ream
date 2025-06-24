use alloy_primitives::B256;
use anyhow::ensure;
use ream_bls::BLSSignature;
use ream_merkle::{generate_proof, merkle_tree};
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash::TreeHash;
use tree_hash_derive::TreeHash;

use super::beacon_block_body::BeaconBlockBody;
use crate::{
    beacon_block_header::{BeaconBlockHeader, SignedBeaconBlockHeader},
    blob_sidecar::BlobSidecar,
    constants::BLOCK_BODY_MERKLE_DEPTH,
    execution_engine::rpc_types::get_blobs::{Blob, BlobAndProofV1},
    polynomial_commitments::kzg_proof::KZGProof,
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Encode, Decode, TreeHash)]
pub struct SignedBeaconBlock {
    pub message: BeaconBlock,
    pub signature: BLSSignature,
}

impl SignedBeaconBlock {
    pub fn signed_header(&self) -> SignedBeaconBlockHeader {
        SignedBeaconBlockHeader {
            message: BeaconBlockHeader {
                slot: self.message.slot,
                proposer_index: self.message.proposer_index,
                parent_root: self.message.parent_root,
                state_root: self.message.state_root,
                body_root: self.message.body.tree_hash_root(),
            },
            signature: self.signature.clone(),
        }
    }

    pub fn blob_sidecar(
        &self,
        blob_and_proof: BlobAndProofV1,
        index: u64,
    ) -> anyhow::Result<BlobSidecar> {
        ensure!(
            index < self.message.body.blob_kzg_commitments.len() as u64,
            "index must be less than the number of blob kzg commitments"
        );
        Ok(BlobSidecar {
            index,
            blob: blob_and_proof.blob,
            kzg_commitment: self.message.body.blob_kzg_commitments[index as usize],
            kzg_proof: blob_and_proof.proof,
            signed_block_header: self.signed_header(),
            kzg_commitment_inclusion_proof: self
                .message
                .body
                .blob_kzg_commitment_inclusion_proof(index)?
                .into(),
        })
    }

    pub fn get_blob_sidecars(
        &self,
        blobs: Vec<Blob>,
        blob_kzg_proofs: Vec<KZGProof>,
    ) -> anyhow::Result<Vec<BlobSidecar>> {
        blobs
            .into_iter()
            .zip(blob_kzg_proofs)
            .enumerate()
            .map(|(index, (blob, proof))| {
                self.blob_sidecar(BlobAndProofV1 { blob, proof }, index as u64)
            })
            .collect::<anyhow::Result<Vec<_>>>()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Encode, Decode, TreeHash)]
pub struct BeaconBlock {
    #[serde(with = "serde_utils::quoted_u64")]
    pub slot: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub proposer_index: u64,
    pub parent_root: B256,
    pub state_root: B256,
    pub body: BeaconBlockBody,
}

impl BeaconBlock {
    pub fn block_root(&self) -> B256 {
        self.tree_hash_root()
    }

    pub fn merkle_leaves(&self) -> Vec<B256> {
        vec![
            self.slot.tree_hash_root(),
            self.proposer_index.tree_hash_root(),
            self.parent_root.tree_hash_root(),
            self.state_root.tree_hash_root(),
            self.body.tree_hash_root(),
        ]
    }

    pub fn data_inclusion_proof(&self, index: u64) -> anyhow::Result<Vec<B256>> {
        const DEPTH: u64 = 3;
        let tree = merkle_tree(&self.merkle_leaves(), DEPTH)?;
        generate_proof(&tree, index, DEPTH)
    }

    pub fn state_root_proof(&self) -> anyhow::Result<Vec<B256>> {
        self.data_inclusion_proof(3)
    }
}
