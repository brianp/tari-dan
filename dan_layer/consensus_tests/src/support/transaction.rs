//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use tari_common_types::types::PrivateKey;
use tari_dan_storage::consensus_models::{Decision, ExecutedTransaction};
use tari_engine_types::{
    commit_result::{ExecuteResult, FinalizeResult, RejectReason, TransactionResult},
    fees::{FeeCostBreakdown, FeeReceipt},
    resource_container::ResourceContainer,
    substate::SubstateDiff,
};

use crate::support::helpers::random_shard;

pub fn build_transaction(decision: Decision, fee: u64, num_shards: usize) -> ExecutedTransaction {
    let k = PrivateKey::default();

    let mut builder = tari_transaction::Transaction::builder();
    for _ in 0..num_shards {
        builder = builder.add_output(random_shard());
    }
    let tx = builder.sign(&k).build();

    let tx_hash = *tx.hash();
    ExecutedTransaction::new(tx.into(), ExecuteResult {
        finalize: FinalizeResult::new(
            tx_hash,
            vec![],
            vec![],
            if decision.is_commit() {
                TransactionResult::Accept(SubstateDiff::new())
            } else {
                TransactionResult::Reject(RejectReason::ExecutionFailure("Test failure".to_string()))
            },
            FeeCostBreakdown {
                total_fees_charged: fee.try_into().unwrap(),
                breakdown: vec![],
            },
        ),
        transaction_failure: None,
        fee_receipt: Some(FeeReceipt {
            total_fee_payment: fee.try_into().unwrap(),
            fee_resource: ResourceContainer::Confidential {
                address: "resource_0000000000000000000000000000000000000000000000000000000000000000"
                    .parse()
                    .unwrap(),
                commitments: Default::default(),
                revealed_amount: fee.try_into().unwrap(),
            },
            cost_breakdown: vec![],
        }),
    })
}