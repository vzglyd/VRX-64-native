//! Manifest parsing.

use serde::{Deserialize, Serialize};

/// Slide manifest structure.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Manifest {
    pub name: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub abi_version: Option<u32>,
    pub scene_space: Option<String>,
}
