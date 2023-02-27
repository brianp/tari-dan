//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use std::borrow::Borrow;

use lazy_static::lazy_static;
use tari_common_types::types::{BulletRangeProof, CommitmentFactory, PrivateKey, RangeProofService};
use tari_crypto::{
    commitment::{ExtensionDegree, HomomorphicCommitmentFactory},
    extended_range_proof::ExtendedRangeProofService,
    ristretto::bulletproofs_plus::{RistrettoExtendedMask, RistrettoExtendedWitness},
};
use tari_template_lib::models::ConfidentialProof;
use tari_utilities::ByteArray;

lazy_static! {
    /// Static reference to the default commitment factory. Each instance of CommitmentFactory requires a number of heap allocations.
    static ref COMMITMENT_FACTORY: CommitmentFactory = CommitmentFactory::default();
    /// Static reference to the default range proof service. Each instance of RangeProofService requires a number of heap allocations.
    static ref RANGE_PROOF_SERVICE: RangeProofService =
        RangeProofService::init(64, 1, CommitmentFactory::default()).unwrap();
}

pub fn range_proof_service() -> &'static RangeProofService {
    RANGE_PROOF_SERVICE.borrow()
}

pub fn generate_confidential_proof(mask: &PrivateKey, value: u64, minimum_value_promise: u64) -> ConfidentialProof {
    let commitment = COMMITMENT_FACTORY.commit_value(mask, value);
    let range_proof = generate_extended_bullet_proof(mask, value, minimum_value_promise);

    ConfidentialProof {
        commitment: {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(commitment.as_bytes());
            bytes
        },
        range_proof: range_proof.0,
        minimum_value_promise,
    }
}

// fn generate_commitment_knowledge_proof(mask: &PrivateKey, value: u64) -> (RistrettoComSig, Commitment) {
//     let public_mask = PublicKey::from_secret_key(mask);
//     let value_mask = PrivateKey::from(value);
//     let nonce_a = PrivateKey::random(&mut OsRng);
//     let nonce_x = PrivateKey::random(&mut OsRng);
//     let public_nonce_agg = COMMITMENT_FACTORY.commit(&nonce_x, &nonce_a);
//     let commitment = COMMITMENT_FACTORY.commit_value(mask, value);
//     let challenge =
//         challenges::confidential_commitment_proof(&public_mask, public_nonce_agg.as_public_key(), &commitment);
//
//     let signature =
//         RistrettoComSig::sign(&value_mask, mask, &nonce_a, &nonce_x, &challenge, &COMMITMENT_FACTORY).unwrap();
//
//     (signature, commitment)
// }

fn generate_extended_bullet_proof(mask: &PrivateKey, value: u64, minimum_value_promise: u64) -> BulletRangeProof {
    let extended_mask = RistrettoExtendedMask::assign(ExtensionDegree::DefaultPedersen, vec![mask.clone()]).unwrap();
    let witnesses = vec![RistrettoExtendedWitness {
        mask: extended_mask,
        value,
        minimum_value_promise,
    }];
    let range_proof = range_proof_service().construct_extended_proof(witnesses, None).unwrap();
    BulletRangeProof(range_proof)
}

pub mod challenges {
    use tari_common_types::types::{Commitment, PublicKey};
    use tari_engine_types::hashing::hasher;
    use tari_template_lib::Hash;

    pub fn confidential_commitment_proof(
        public_key: &PublicKey,
        public_nonce: &PublicKey,
        commitment: &Commitment,
    ) -> Hash {
        hasher("ConfidentialProof")
            .chain(&public_key)
            .chain(&public_nonce)
            .chain(commitment.as_public_key())
            .result()
    }
}