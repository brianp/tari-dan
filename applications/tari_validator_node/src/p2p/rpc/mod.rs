//  Copyright 2021, The Tari Project
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

mod service_impl;

pub use service_impl::ValidatorNodeRpcServiceImpl;
use tari_dan_core::services::PeerProvider;
use tari_dan_storage_sqlite::sqlite_shard_store_factory::SqliteShardStore;
use tari_validator_node_rpc::rpc_service::ValidatorNodeRpcServer;

use crate::p2p::services::mempool::MempoolHandle;

pub fn create_tari_validator_node_rpc_service<TPeerProvider>(
    peer_provider: TPeerProvider,
    shard_store_store: SqliteShardStore,
    mempool: MempoolHandle,
) -> ValidatorNodeRpcServer<ValidatorNodeRpcServiceImpl<TPeerProvider>>
where
    TPeerProvider: PeerProvider + Clone + Send + Sync + 'static,
{
    ValidatorNodeRpcServer::new(ValidatorNodeRpcServiceImpl::new(
        peer_provider,
        shard_store_store,
        mempool,
    ))
}
