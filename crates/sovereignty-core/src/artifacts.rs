use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArtifactError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Missing field: {0}")]
    MissingField(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohModel {
    pub roh_ceiling: f32, // e.g. 0.3
    pub weights_bio: f32,
    pub weights_legal: f32,
    pub weights_eco: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakePolicy {
    pub host_did: String,
    pub evolve_signers: Vec<String>,       // required DIDs
    pub lifeforce_scopes: Vec<String>,     // e.g., ["lifeforce_alteration"]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsPolicy {
    pub forbid_dream_state: bool,
    pub forbid_decision_use_domains: Vec<String>, // e.g. ["employment","credit"]
}

pub struct ArtifactLoader;

impl ArtifactLoader {
    pub fn load_rohmodel<P: AsRef<Path>>(p: P) -> Result<RohModel, ArtifactError> {
        let s = fs::read_to_string(p).map_err(|e| ArtifactError::Io(e.to_string()))?;
        // ALN/JSON; here assume JSON for core, with ALN -> JSON pre-step.
        serde_json::from_str(&s).map_err(|e| ArtifactError::Parse(e.to_string()))
    }

    pub fn load_stake<P: AsRef<Path>>(p: P) -> Result<StakePolicy, ArtifactError> {
        let s = fs::read_to_string(p).map_err(|e| ArtifactError::Io(e.to_string()))?;
        // stake.aln is typically ALN/JSON; treat as JSON struct.
        serde_json::from_str(&s).map_err(|e| ArtifactError::Parse(e.to_string()))
    }

    pub fn load_neurorights<P: AsRef<Path>>(p: P) -> Result<NeurorightsPolicy, ArtifactError> {
        let s = fs::read_to_string(p).map_err(|e| ArtifactError::Io(e.to_string()))?;
        serde_json::from_str(&s).map_err(|e| ArtifactError::Parse(e.to_string()))
    }
}
