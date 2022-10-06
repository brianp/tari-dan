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

use tari_common_types::types::FixedHash;

use crate::global::GlobalDbAdapter;

pub struct TemplateDb<'a, TGlobalDbAdapter: GlobalDbAdapter> {
    backend: &'a TGlobalDbAdapter,
    tx: &'a TGlobalDbAdapter::DbTransaction,
}

impl<'a, TGlobalDbAdapter: GlobalDbAdapter> TemplateDb<'a, TGlobalDbAdapter> {
    pub fn new(backend: &'a TGlobalDbAdapter, tx: &'a TGlobalDbAdapter::DbTransaction) -> Self {
        Self { backend, tx }
    }

    pub fn get_template(&self, key: &[u8]) -> Result<Option<DbTemplate>, TGlobalDbAdapter::Error> {
        self.backend.get_template(self.tx, key)
    }

    pub fn insert_template(&self, template: DbTemplate) -> Result<(), TGlobalDbAdapter::Error> {
        self.backend.insert_template(self.tx, template)
    }
}

#[derive(Debug, Clone)]
pub struct DbTemplate {
    pub template_address: FixedHash,
    pub url: String,
    pub height: u64,
    pub compiled_code: Vec<u8>,
}