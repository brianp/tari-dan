// Copyright 2021. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::services::infrastructure_services::OutboundService;

pub fn mock_inbound<TAddr: NodeAddressable, TPayload: Payload>() -> MockInboundConnectionService<TAddr, TPayload> {
    MockInboundConnectionService::default()
}

type Messages<TAddr, TPayload> = (
    Sender<(TAddr, DanMessage<TPayload, TAddr>)>,
    Receiver<(TAddr, DanMessage<TPayload, TAddr>)>,
);

#[derive()]
pub struct MockInboundConnectionService<TAddr: NodeAddressable, TPayload: Payload> {
    messages: Messages<TAddr, TPayload>,
}

impl<TAddr: NodeAddressable, TPayload: Payload> Default for MockInboundConnectionService<TAddr, TPayload> {
    fn default() -> Self {
        Self { messages: channel(10) }
    }
}

impl<TAddr: NodeAddressable, TPayload: Payload> MockInboundConnectionService<TAddr, TPayload> {
    pub fn _push(&mut self, from: TAddr, message: DanMessage<TPayload, TAddr>) {
        self.messages.0.try_send((from, message)).unwrap()
    }

    pub fn _create_sender(&self) -> Sender<(TAddr, DanMessage<TPayload, TAddr>)> {
        self.messages.0.clone()
    }
}

pub fn mock_outbound<TAddr: NodeAddressable, TPayload: Payload>(
    committee: Vec<TAddr>,
) -> MockOutboundService<TAddr, TPayload> {
    MockOutboundService::new(committee)
}

pub struct MockOutboundService<TAddr: NodeAddressable, TPayload: Payload> {
    #[allow(clippy::type_complexity)]
    inbound_senders: HashMap<TAddr, Sender<(TAddr, DanMessage<TPayload, TAddr>)>>,
    inbounds: HashMap<TAddr, MockInboundConnectionService<TAddr, TPayload>>,
}

impl<TAddr: NodeAddressable, TPayload: Payload> Clone for MockOutboundService<TAddr, TPayload> {
    fn clone(&self) -> Self {
        MockOutboundService {
            inbound_senders: self.inbound_senders.clone(),
            inbounds: HashMap::new(),
        }
    }
}
impl<TAddr: NodeAddressable, TPayload: Payload> MockOutboundService<TAddr, TPayload> {
    pub fn new(committee: Vec<TAddr>) -> Self {
        let mut inbounds = HashMap::new();
        let mut inbound_senders = HashMap::new();
        for member in committee {
            let inbound = mock_inbound();
            inbound_senders.insert(member.clone(), inbound.messages.0.clone());
            inbounds.insert(member.clone(), inbound);
        }
        Self {
            inbounds,
            inbound_senders,
        }
    }

    pub fn take_inbound(&mut self, member: &TAddr) -> Option<MockInboundConnectionService<TAddr, TPayload>> {
        self.inbounds.remove(member)
    }
}

use std::fmt::Debug;

use tari_dan_common_types::NodeAddressable;
use tari_dan_storage::models::Payload;

use crate::message::DanMessage;

#[async_trait]
impl<TAddr: NodeAddressable + Send + Sync + Debug, TPayload: Payload> OutboundService
    for MockOutboundService<TAddr, TPayload>
{
    type Addr = TAddr;
    type Error = anyhow::Error;
    type Payload = TPayload;

    async fn send(
        &mut self,
        _from: TAddr,
        to: TAddr,
        message: DanMessage<Self::Payload, Self::Addr>,
    ) -> Result<(), Self::Error> {
        // intentionally swallow error here because the other end can die in tests
        let _result = self.inbound_senders.get_mut(&to).unwrap().send((to, message)).await;
        Ok(())
    }

    async fn broadcast(
        &mut self,
        from: TAddr,
        _committee: &[TAddr],
        message: DanMessage<Self::Payload, Self::Addr>,
    ) -> Result<(), Self::Error> {
        let receivers: Vec<TAddr> = self.inbound_senders.keys().cloned().collect();
        for receiver in receivers {
            self.send(from.clone(), receiver.clone(), message.clone()).await?
        }
        Ok(())
    }

    async fn flood(
        &mut self,
        from: Self::Addr,
        message: DanMessage<Self::Payload, Self::Addr>,
    ) -> Result<(), Self::Error> {
        let receivers: Vec<TAddr> = self.inbound_senders.keys().cloned().collect();
        for receiver in receivers {
            self.send(from.clone(), receiver.clone(), message.clone()).await?
        }
        Ok(())
    }
}
