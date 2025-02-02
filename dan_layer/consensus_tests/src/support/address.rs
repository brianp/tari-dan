//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tari_dan_common_types::{hasher::tari_hasher, hashing::TariDanConsensusHashDomain, NodeAddressable, ShardId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TestAddress(pub &'static str);

impl TestAddress {
    pub fn to_shard_id(self) -> ShardId {
        ShardId::from_bytes(
            tari_hasher::<TariDanConsensusHashDomain>("test")
                .chain(self.0)
                .result()
                .as_slice(),
        )
        .unwrap()
    }
}

impl NodeAddressable for TestAddress {
    fn zero() -> Self {
        TestAddress("")
    }

    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Display for TestAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestAddress({})", self.0)
    }
}
