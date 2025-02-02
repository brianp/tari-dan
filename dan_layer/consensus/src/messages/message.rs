//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use serde::Serialize;
use tari_dan_common_types::Epoch;

use super::{NewViewMessage, ProposalMessage, VoteMessage};

#[derive(Debug, Clone, Serialize)]
pub enum HotstuffMessage {
    NewView(NewViewMessage),
    Proposal(ProposalMessage),
    Vote(VoteMessage),
}

impl HotstuffMessage {
    pub fn epoch(&self) -> Epoch {
        match self {
            Self::NewView(msg) => msg.high_qc.epoch(),
            Self::Proposal(msg) => msg.block.epoch(),
            Self::Vote(msg) => msg.epoch,
        }
    }
}
