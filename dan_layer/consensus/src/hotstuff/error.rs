//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use tari_common_types::types::PublicKey;
use tari_dan_common_types::Epoch;
use tari_dan_storage::{consensus_models::BlockId, StorageError};
use tari_mmr::BalancedBinaryMerkleProofError;

use crate::traits::EpochManagerError;

#[derive(Debug, thiserror::Error)]
pub enum HotStuffError {
    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),
    #[error("Internal channel send error when {context}")]
    InternalChannelClosed { context: &'static str },
    #[error("Epoch {epoch} is not active. {details}")]
    EpochNotActive { epoch: Epoch, details: String },
    #[error("Received message from non-committee member. Epoch: {epoch}, Sender: {sender}, {context}")]
    ReceivedMessageFromNonCommitteeMember {
        epoch: Epoch,
        sender: String,
        context: String,
    },
    #[error("Proposal validation error: {0}")]
    ProposalValidationError(#[from] ProposalValidationError),
    #[error("Decision mismatch for block {block_id} in pool {pool}")]
    DecisionMismatch { block_id: BlockId, pool: &'static str },
    #[error("Not the leader. {details}")]
    NotTheLeader { details: String },
    #[error("Merkle proof error: {0}")]
    BalancedBinaryMerkleProofError(#[from] BalancedBinaryMerkleProofError),
    #[error("Epoch manager error: {0}")]
    EpochManagerError(anyhow::Error),
    #[error("Invalid vote signature from {signer_public_key} (unauthenticated)")]
    InvalidVoteSignature { signer_public_key: PublicKey },
}

// This removes the need for `map_err`s for every epoch manager call
impl<E: EpochManagerError> From<E> for HotStuffError {
    fn from(err: E) -> Self {
        Self::EpochManagerError(err.to_anyhow())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProposalValidationError {
    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),
    #[error("Node proposed by {proposed_by} with hash {hash} does not match calculated hash {calculated_hash}")]
    NodeHashMismatch {
        proposed_by: String,
        hash: BlockId,
        calculated_hash: BlockId,
    },
    #[error("Node proposed by {proposed_by} with hash {hash} did not satisfy the safeNode predicate")]
    NotSafeBlock { proposed_by: String, hash: BlockId },
    #[error("Node proposed by {proposed_by} with hash {hash} did not satisfy the validNode predicate")]
    ProposingGenesisBlock { proposed_by: String, hash: BlockId },
    #[error("Justification block {justify_block} for proposed block {hash} by {proposed_by} not found")]
    JustifyBlockNotFound {
        proposed_by: String,
        hash: BlockId,
        justify_block: BlockId,
    },
    #[error("QC in block {block_id} that was proposed by {proposed_by} is invalid: {details}")]
    JustifyBlockInvalid {
        proposed_by: String,
        block_id: BlockId,
        details: String,
    },
}
