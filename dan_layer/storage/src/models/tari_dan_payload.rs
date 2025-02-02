//  Copyright 2021. The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::fmt::Debug;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use tari_common_types::types::FixedHash;
use tari_dan_common_types::ShardId;
use tari_engine_types::commit_result::ExecuteResult;
use tari_transaction::Transaction;

use crate::models::{ConsensusHash, Payload};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TariDanPayload {
    transaction: Transaction,
    timestamp: i64,
    result: Option<ExecuteResult>,
}

impl TariDanPayload {
    pub fn new(transaction: Transaction) -> Self {
        Self {
            transaction,
            timestamp: Utc::now().timestamp(),
            result: None,
        }
    }

    pub fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    pub fn into_payload(self) -> Transaction {
        self.transaction
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn result(&self) -> Option<&ExecuteResult> {
        self.result.as_ref()
    }

    pub fn set_result(&mut self, result: ExecuteResult) {
        self.result = Some(result);
    }
}

impl ConsensusHash for TariDanPayload {
    fn consensus_hash(&self) -> FixedHash {
        self.transaction.hash().into_array().into()
    }
}

impl Payload for TariDanPayload {
    fn involved_shards(&self) -> Vec<ShardId> {
        self.transaction.meta().involved_shards()
    }

    fn max_outputs(&self) -> u32 {
        self.transaction.meta().max_outputs()
    }
}

#[derive(Debug, Clone, Default)]
pub struct CheckpointData {
    hash: FixedHash,
}

impl ConsensusHash for CheckpointData {
    fn consensus_hash(&self) -> FixedHash {
        self.hash
    }
}
