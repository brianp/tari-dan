//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use serde::Serialize;
use tari_bor::encode;
use tari_template_abi::rust::{collections::HashMap, fmt, ops::RangeInclusive};

use crate::{
    args::MintArg,
    models::{Bucket, Metadata, NonFungibleId, ResourceAddress},
    resource::{ResourceManager, ResourceType},
};

pub struct NonFungibleResourceBuilder {
    token_symbol: String,
    metadata: Metadata,
    tokens_ids: HashMap<NonFungibleId, (Vec<u8>, Vec<u8>)>,
}

impl NonFungibleResourceBuilder {
    pub(super) fn new<S: Into<String>>(symbol: S) -> Self {
        Self {
            token_symbol: symbol.into(),
            metadata: Metadata::new(),
            tokens_ids: HashMap::new(),
        }
    }

    pub fn add_metadata<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn with_non_fungibles<'a, I, T, U>(mut self, tokens: I) -> Self
    where
        I: IntoIterator<Item = (NonFungibleId, (&'a T, &'a U))>,
        T: Serialize + 'a,
        U: Serialize + 'a,
    {
        self.tokens_ids.extend(
            tokens
                .into_iter()
                .map(|(id, (data, mutable))| (id, (encode(data).unwrap(), encode(mutable).unwrap()))),
        );
        self
    }

    pub fn mint_many_with<F, T>(mut self, bounds: RangeInclusive<usize>, mut f: F) -> Self
    where
        F: FnMut(T) -> (NonFungibleId, (Vec<u8>, Vec<u8>)),
        T: TryFrom<usize>,
        T::Error: fmt::Debug,
    {
        self.tokens_ids.extend(bounds.map(|n| f(n.try_into().unwrap())));
        self
    }

    pub fn build(self) -> ResourceAddress {
        // TODO: Improve API
        assert!(self.tokens_ids.is_empty(), "call build_bucket with initial tokens set");
        let (address, _) = Self::build_internal(self.token_symbol, self.metadata, None);
        address
    }

    pub fn build_bucket(self) -> Bucket {
        let mint_args = MintArg::NonFungible {
            tokens: self.tokens_ids,
        };

        let (_, bucket) = Self::build_internal(self.token_symbol, self.metadata, Some(mint_args));
        bucket.expect("[build_bucket] Bucket not returned from system")
    }

    fn build_internal(
        token_symbol: String,
        metadata: Metadata,
        mint_args: Option<MintArg>,
    ) -> (ResourceAddress, Option<Bucket>) {
        ResourceManager::new().create(ResourceType::NonFungible, token_symbol, metadata, mint_args)
    }
}
