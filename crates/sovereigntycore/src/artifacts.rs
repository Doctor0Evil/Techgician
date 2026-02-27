use crate::types::*;
use serde_json::de::IoRead;
use serde_json::Deserializer;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(thiserror::Error, Debug)]
pub enum ArtifactError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("schema error: {0}")]
    Schema(String),
}

pub struct ArtifactLoader;

impl ArtifactLoader {
    pub fn load_rohmodel(path: &str) -> Result<RoHModel, ArtifactError> {
        let f = File::open(path)?;
        let model: RoHModel = serde_json::from_reader(f)?;
        if model.roh_ceiling <= 0.0 || model.roh_ceiling > 0.3 {
            return Err(ArtifactError::Schema(format!(
                "roh_ceiling out of Tsafe range: {}",
                model.roh_ceiling
            )));
        }
        Ok(model)
    }

    pub fn load_stake(path: &str) -> Result<StakePolicy, ArtifactError> {
        let f = File::open(path)?;
        let s: StakePolicy = serde_json::from_reader(f)?;
        if s.host_did.is_empty() {
            return Err(ArtifactError::Schema("host_did empty".into()));
        }
        Ok(s)
    }

    pub fn load_neurorights(path: &str) -> Result<NeurorightsPolicy, ArtifactError> {
        let f = File::open(path)?;
        let n: NeurorightsPolicy = serde_json::from_reader(f)?;
        Ok(n)
    }

    pub fn iter_donutloop(
        path: &str,
    ) -> Result<impl Iterator<Item = Result<DonutloopEntry, ArtifactError>>, ArtifactError> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        Ok(reader
            .lines()
            .map(|line| -> Result<DonutloopEntry, ArtifactError> {
                let line = line?;
                if line.trim().is_empty() {
                    return Err(ArtifactError::Schema("empty line".into()));
                }
                let entry: DonutloopEntry = serde_json::from_str(&line)?;
                Ok(entry)
            }))
    }
}

#[derive(Debug)]
pub struct EvaluationContext {
    pub biostate: BioStateSnapshot,
    pub envelopes: PersonalEnvelopes,
}
