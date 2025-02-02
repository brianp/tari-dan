//  Copyright 2022 The Tari Project
//  SPDX-License-Identifier: BSD-3-Clause

use std::{collections::HashMap, sync::Arc};

use tari_dan_common_types::services::template_provider::TemplateProvider;

use crate::{
    function_definitions::FunctionArgDefinition,
    packager::LoadedTemplate,
    runtime::{AuthorizationScope, Runtime},
};

pub struct FlowContext<TTemplateProvider: TemplateProvider<Template = LoadedTemplate>> {
    pub template_provider: Arc<TTemplateProvider>,
    pub runtime: Runtime,
    pub auth_scope: AuthorizationScope,
    pub args: HashMap<String, (Vec<u8>, FunctionArgDefinition)>,
    pub recursion_depth: usize,
    pub max_recursion_depth: usize,
}
