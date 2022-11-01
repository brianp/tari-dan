//   Copyright 2022. The Tari Project
//
//   Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//   following conditions are met:
//
//   1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//   disclaimer.
//
//   2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//   following disclaimer in the documentation and/or other materials provided with the distribution.
//
//   3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//   products derived from this software without specific prior written permission.
//
//   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//   INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//   DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//   SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//   WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//   USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::sync::{atomic::AtomicU32, Arc};

use tari_dan_common_types::ShardId;
use tari_engine_types::hashing::hasher;
use tari_template_lib::{
    models::{BucketId, ComponentAddress, ResourceAddress, VaultId},
    Hash,
};

#[derive(Debug, Clone)]
pub struct IdProvider {
    current_id: Arc<AtomicU32>,
    transaction_hash: Hash,
}

impl IdProvider {
    pub fn new(transaction_hash: Hash) -> Self {
        Self {
            current_id: Arc::new(AtomicU32::new(0)),
            transaction_hash,
        }
    }

    fn next(&self) -> u32 {
        self.current_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    pub fn transaction_hash(&self) -> Hash {
        self.transaction_hash
    }

    /// Generates a new unique id H(tx_hash || n).
    /// NOTE: we rely on IDs being predictable for all outputs (components, resources, vaults).
    fn new_id(&self) -> Hash {
        hasher("output")
            .chain(&self.transaction_hash)
            .chain(&self.next())
            .result()
    }

    pub fn new_resource_address(&self) -> ResourceAddress {
        self.new_id()
    }

    pub fn new_component_address(&self) -> ComponentAddress {
        self.new_id()
    }

    pub fn new_output_shard(&self) -> ShardId {
        self.new_id().into_array().into()
    }

    pub fn new_vault_id(&self) -> VaultId {
        self.new_id()
    }

    pub fn new_bucket_id(&self) -> BucketId {
        self.next()
    }
}
