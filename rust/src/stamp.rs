use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ALNDIDBostromStampV1 {
    pub authorsystem: String,
    pub primarybostromaddr: String,
    pub altbostromaddr: Option<String>,
    pub safeaddrs: Vec<String>,
    pub responsehashhex: String,
    pub tscore0to1: f64,
    pub pscore0to1: f64,
    pub rscore0to1: f64,
    pub cscore0to1: f64,
    pub timestamputciso8601: String,
    pub notes: String,
}

impl ALNDIDBostromStampV1 {
    /// Validates scores are in [0.0, 1.0] and computes/verifies response hash.
    pub fn validate(&self, canonical_text: &[u8]) -> Result<(), StampError> {
        if !(0.0..=1.0).contains(&self.tscore0to1)
            || !(0.0..=1.0).contains(&self.pscore0to1)
            || !(0.0..=1.0).contains(&self.rscore0to1)
            || !(0.0..=1.0).contains(&self.cscore0to1)
        {
            return Err(StampError::InvalidScore);
        }

        let mut hasher = Sha256::new();
        hasher.update(canonical_text);
        let computed_hash = format!("{:x}", hasher.finalize());

        if computed_hash != self.responsehashhex {
            return Err(StampError::HashMismatch {
                expected: self.responsehashhex.clone(),
                computed: computed_hash,
            });
        }

        // Future: Add Bech32 address validation for bostrom* prefixes
        // Add ISO8601 timestamp parse/validation

        Ok(())
    }
}

#[derive(Debug)]
pub enum StampError {
    InvalidScore,
    HashMismatch { expected: String, computed: String },
}

impl fmt::Display for StampError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StampError::InvalidScore => write!(f, "Scores must be in [0.0, 1.0]"),
            StampError::HashMismatch { expected, computed } => {
                write!(f, "Hash mismatch: expected {}, computed {}", expected, computed)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        // Add real unit tests with sample canonical bytes + known hash
    }
}
