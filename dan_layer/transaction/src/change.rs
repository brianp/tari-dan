//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use std::{
    fmt,
    fmt::{Display, Formatter},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SubstateChange {
    /// An "Up" state
    Create,
    /// Substate exists but will not be created/destroyed
    Exists,
    /// A "Down" state
    Destroy,
}

impl SubstateChange {
    pub fn is_create(&self) -> bool {
        matches!(self, SubstateChange::Create)
    }

    pub fn is_exists(&self) -> bool {
        matches!(self, SubstateChange::Exists)
    }

    pub fn is_destroy(&self) -> bool {
        matches!(self, SubstateChange::Destroy)
    }
}

impl Display for SubstateChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SubstateChange::Create => write!(f, "Create"),
            SubstateChange::Exists => write!(f, "Exists"),
            SubstateChange::Destroy => write!(f, "Destroy"),
        }
    }
}
