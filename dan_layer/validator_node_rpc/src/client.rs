//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use std::convert::{TryFrom, TryInto};

use anyhow::anyhow;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tari_bor::decode;
use tari_common_types::types::{FixedHash, PublicKey};
use tari_comms::{
    connectivity::ConnectivityRequester,
    multiaddr::Multiaddr,
    peer_manager::{NodeId, PeerIdentityClaim},
    protocol::rpc::RpcPoolClient,
    types::CommsPublicKey,
    PeerConnection,
};
use tari_crypto::tari_utilities::ByteArray;
use tari_dan_common_types::{NodeAddressable, PayloadId};
use tari_dan_core::services::DanPeer;
use tari_engine_types::{
    commit_result::ExecuteResult,
    substate::{Substate, SubstateAddress},
};
use tari_transaction::Transaction;
use tokio_stream::StreamExt;

use crate::{
    proto::rpc::{
        GetPeersRequest,
        GetTransactionResultRequest,
        PayloadResultStatus,
        SubmitTransactionRequest,
        SubstateStatus,
    },
    rpc_service,
    ValidatorNodeRpcClientError,
};

pub trait ValidatorNodeClientFactory: Send + Sync {
    type Addr: NodeAddressable;
    type Client: ValidatorNodeRpcClient<Addr = Self::Addr>;

    fn create_client(&self, address: &Self::Addr) -> Self::Client;
}

#[async_trait]
pub trait ValidatorNodeRpcClient: Send + Sync {
    type Addr: NodeAddressable;
    type Error: std::error::Error + Send + Sync + 'static;

    async fn submit_transaction(&mut self, transaction: Transaction) -> Result<PayloadId, Self::Error>;
    async fn get_finalized_transaction_result(
        &mut self,
        payload_id: PayloadId,
    ) -> Result<TransactionResultStatus, Self::Error>;

    async fn get_peers(&mut self) -> Result<Vec<DanPeer<Self::Addr>>, Self::Error>;

    async fn get_substate(&mut self, address: &SubstateAddress, version: u32) -> Result<SubstateResult, Self::Error>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TransactionResultStatus {
    Pending,
    Finalized(ExecuteResult),
}

impl TransactionResultStatus {
    pub fn into_finalized(&self) -> Option<ExecuteResult> {
        match self {
            Self::Pending => None,
            Self::Finalized(result) => Some(result.clone()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SubstateResult {
    DoesNotExist,
    Up {
        substate: Substate,
        created_by_tx: FixedHash,
    },
    Down {
        version: u32,
        created_by_tx: FixedHash,
        deleted_by_tx: FixedHash,
    },
}

pub struct TariCommsValidatorNodeRpcClient {
    connectivity: ConnectivityRequester,
    address: PublicKey,
    connection: Option<(PeerConnection, rpc_service::ValidatorNodeRpcClient)>,
}

impl TariCommsValidatorNodeRpcClient {
    pub async fn client_connection(
        &mut self,
    ) -> Result<rpc_service::ValidatorNodeRpcClient, ValidatorNodeRpcClientError> {
        if let Some((_, ref client)) = self.connection {
            if client.is_connected() {
                return Ok(client.clone());
            }
        }
        let mut conn = self
            .connectivity
            .dial_peer(NodeId::from_public_key(&self.address))
            .await?;
        let client = conn.connect_rpc().await?;
        Ok(client)
    }
}

#[async_trait]
impl ValidatorNodeRpcClient for TariCommsValidatorNodeRpcClient {
    type Addr = CommsPublicKey;
    type Error = ValidatorNodeRpcClientError;

    async fn submit_transaction(&mut self, transaction: Transaction) -> Result<PayloadId, ValidatorNodeRpcClientError> {
        let mut client = self.client_connection().await?;
        let request = SubmitTransactionRequest {
            transaction: Some(transaction.into()),
        };
        let response = client.submit_transaction(request).await?;

        let payload_id = response.transaction_hash.try_into().map_err(|_| {
            ValidatorNodeRpcClientError::InvalidResponse(anyhow!("Node returned an invalid or empty payload id"))
        })?;

        Ok(payload_id)
    }

    async fn get_peers(&mut self) -> Result<Vec<DanPeer<Self::Addr>>, ValidatorNodeRpcClientError> {
        let mut client = self.client_connection().await?;
        // TODO(perf): This doesnt scale, find a nice way to wrap up the stream
        let peers = client
            .get_peers(GetPeersRequest { since: 0 })
            .await?
            .map(|result| {
                let p = result?;
                let addresses: Vec<Multiaddr> = p
                    .addresses
                    .into_iter()
                    .map(|a| {
                        Multiaddr::try_from(a)
                            .map_err(|_| ValidatorNodeRpcClientError::InvalidResponse(anyhow!("Invalid address")))
                    })
                    .collect::<Result<_, _>>()?;
                let claims: Vec<PeerIdentityClaim> = p
                    .claims
                    .into_iter()
                    .map(|c| {
                        PeerIdentityClaim::try_from(c)
                            .map_err(|_| ValidatorNodeRpcClientError::InvalidResponse(anyhow!("Invalid claim")))
                    })
                    .collect::<Result<_, _>>()?;
                Result::<_, ValidatorNodeRpcClientError>::Ok(DanPeer {
                    identity: CommsPublicKey::from_bytes(&p.identity)
                        .map_err(|_| ValidatorNodeRpcClientError::InvalidResponse(anyhow!("Invalid identity")))?,
                    addresses: addresses.into_iter().zip(claims).collect(),
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .await?;
        Ok(peers)
    }

    async fn get_substate(&mut self, address: &SubstateAddress, version: u32) -> Result<SubstateResult, Self::Error> {
        let mut client = self.client_connection().await?;
        // request the shard substate to the VN
        let request = crate::proto::rpc::GetSubstateRequest {
            address: address.to_bytes(),
            version,
        };

        let resp = client.get_substate(request).await?;
        let status = SubstateStatus::from_i32(resp.status).ok_or_else(|| {
            ValidatorNodeRpcClientError::InvalidResponse(anyhow!(
                "Node returned invalid substate status {}",
                resp.status
            ))
        })?;

        // TODO: verify the quorum certificates
        // for qc in resp.quorum_certificates {
        //     let qc = QuorumCertificate::try_from(&qc)?;
        // }

        match status {
            SubstateStatus::Up => {
                let tx_hash = resp.created_transaction_hash.try_into().map_err(|_| {
                    ValidatorNodeRpcClientError::InvalidResponse(anyhow!(
                        "Node returned an invalid or empty transaction hash"
                    ))
                })?;
                let substate = Substate::from_bytes(&resp.substate)
                    .map_err(|e| ValidatorNodeRpcClientError::InvalidResponse(anyhow!(e)))?;
                Ok(SubstateResult::Up {
                    substate,
                    created_by_tx: tx_hash,
                })
            },
            SubstateStatus::Down => {
                let created_by_tx = resp.created_transaction_hash.try_into().map_err(|_| {
                    ValidatorNodeRpcClientError::InvalidResponse(anyhow!(
                        "Node returned an invalid or empty created transaction hash"
                    ))
                })?;
                let deleted_by_tx = resp.destroyed_transaction_hash.try_into().map_err(|_| {
                    ValidatorNodeRpcClientError::InvalidResponse(anyhow!(
                        "Node returned an invalid or empty destroyed transaction hash"
                    ))
                })?;
                Ok(SubstateResult::Down {
                    version: resp.version,
                    deleted_by_tx,
                    created_by_tx,
                })
            },
            SubstateStatus::DoesNotExist => Ok(SubstateResult::DoesNotExist),
        }
    }

    async fn get_finalized_transaction_result(
        &mut self,
        payload_id: PayloadId,
    ) -> Result<TransactionResultStatus, ValidatorNodeRpcClientError> {
        let mut client = self.client_connection().await?;
        let request = GetTransactionResultRequest {
            payload_id: payload_id.as_bytes().to_vec(),
        };
        let response = client.get_transaction_result(request).await?;

        match PayloadResultStatus::from_i32(response.status) {
            Some(PayloadResultStatus::Pending) => Ok(TransactionResultStatus::Pending),
            Some(PayloadResultStatus::Finalized) => {
                let execution_result = decode(&response.execution_result).map_err(|_| {
                    ValidatorNodeRpcClientError::InvalidResponse(anyhow!(
                        "Node returned an invalid or empty payload id"
                    ))
                })?;
                Ok(TransactionResultStatus::Finalized(execution_result))
            },
            None => Err(ValidatorNodeRpcClientError::InvalidResponse(anyhow!(
                "Node returned invalid payload status {}",
                response.status
            ))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TariCommsValidatorNodeClientFactory {
    connectivity: ConnectivityRequester,
}

impl TariCommsValidatorNodeClientFactory {
    pub fn new(connectivity: ConnectivityRequester) -> Self {
        Self { connectivity }
    }
}

impl ValidatorNodeClientFactory for TariCommsValidatorNodeClientFactory {
    type Addr = PublicKey;
    type Client = TariCommsValidatorNodeRpcClient;

    fn create_client(&self, address: &Self::Addr) -> Self::Client {
        TariCommsValidatorNodeRpcClient {
            connectivity: self.connectivity.clone(),
            address: address.clone(),
            connection: None,
        }
    }
}
