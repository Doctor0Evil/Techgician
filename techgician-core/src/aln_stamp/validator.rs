#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KerAtSigning {
    pub K: f32, // [0,1]
    pub E: f32, // [0,1]
    pub R: f32, // [0,1]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcSigner {
    pub role: String,   // "author" | "infra" | "auditor"
    pub did: String,
    pub pubkey: String, // base64 or hex – fixed in spec
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcMultisig {
    pub scheme: String,       // "dilithium3", "sphincs+-sha256-192s", ...
    pub threshold_m: u32,
    pub threshold_n: u32,
    pub signers: Vec<PqcSigner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnDidBostromStampV1 {
    pub authorsystem: String,
    pub primarybostromaddr: String,
    pub altbostromaddr: Option<String>,
    pub safeaddrs: Vec<String>,
    pub responsehashhex: String,
    pub Tscore0to1: f32,
    pub Pscore0to1: f32,
    pub Rscore0to1: f32,
    pub Cscore0to1: f32,
    pub timestamputciso8601: String,

    pub ker_at_signing: KerAtSigning,
    pub corridor_ids: Vec<String>,
    pub pqc_multisig: PqcMultisig,
}

#[derive(Debug, thiserror::Error)]
pub enum StampError {
    #[error("invalid Bostrom address")]
    InvalidBostrom,
    #[error("score out of [0,1] range")]
    InvalidScore,
    #[error("invalid responsehashhex")]
    InvalidHash,
    #[error("invalid timestamp")]
    InvalidTimestamp,
    #[error("invalid KER range")]
    InvalidKer,
    #[error("insufficient corridors")]
    InvalidCorridors,
    #[error("invalid PQC multisig")]
    InvalidPqc,
}

/// Validate static fields; PQC signature verification is delegated to scheme-specific crates.
pub fn validate_stamp_meta(stamp: &AlnDidBostromStampV1) -> Result<(), StampError> {
    // Bostrom addr regex check (cheap, deterministic)
    let bostrom_ok = stamp
        .primarybostromaddr
        .starts_with("bostrom")
        && stamp.primarybostromaddr.len() >= 10
        && stamp.primarybostromaddr.len() <= 80;
    if !bostrom_ok {
        return Err(StampError::InvalidBostrom);
    }

    // score ranges
    for s in [
        stamp.Tscore0to1,
        stamp.Pscore0to1,
        stamp.Rscore0to1,
        stamp.Cscore0to1,
    ] {
        if !(0.0..=1.0).contains(&s) || !s.is_finite() {
            return Err(StampError::InvalidScore);
        }
    }

    // responsehashhex length and charset
    if stamp.responsehashhex.len() != 64
        || !stamp
            .responsehashhex
            .chars()
            .all(|c| c.is_ascii_hexdigit() && c.is_ascii_lowercase())
    {
        return Err(StampError::InvalidHash);
    }

    // timestamp parse
    if DateTime::parse_from_rfc3339(&stamp.timestamputciso8601)
        .map(|dt| dt.with_timezone(&Utc))
        .is_err()
    {
        return Err(StampError::InvalidTimestamp);
    }

    // KER ranges
    for v in [stamp.ker_at_signing.K, stamp.ker_at_signing.E, stamp.ker_at_signing.R] {
        if !(0.0..=1.0).contains(&v) || !v.is_finite() {
            return Err(StampError::InvalidKer);
        }
    }

    // corridors
    if stamp.corridor_ids.is_empty() {
        return Err(StampError::InvalidCorridors);
    }

    // basic PQC multisig sanity
    let pqc = &stamp.pqc_multisig;
    if pqc.threshold_m == 0 || pqc.threshold_n == 0 || pqc.threshold_m > pqc.threshold_n {
        return Err(StampError::InvalidPqc);
    }
    if pqc.signers.len() < pqc.threshold_n as usize {
        return Err(StampError::InvalidPqc);
    }

    Ok(())
}

/// Recompute SHA-256 over the canonical artifact bytes and compare to responsehashhex.
pub fn verify_response_hash(canonical_artifact_bytes: &[u8], stamp: &AlnDidBostromStampV1) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(canonical_artifact_bytes);
    let digest = hasher.finalize();
    let hex = hex::encode(digest); // lower-case
    hex == stamp.responsehashhex
}
