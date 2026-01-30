use crate::aln_did_stamp::AlnDidBostromStampV1;
use sha2::{Digest, Sha256};
use thiserror::Error;
use time::OffsetDateTime;

#[derive(Debug, Error)]
pub enum StampValidationError {
    #[error("primary_bostrom_addr invalid format")]
    InvalidPrimaryAddr,
    #[error("safe_addrs must not be empty")]
    EmptySafeAddrs,
    #[error("score out of range [0,1]")]
    ScoreOutOfRange,
    #[error("timestamp invalid: {0}")]
    InvalidTimestamp(String),
    #[error("response_hash_hex invalid hex")]
    InvalidResponseHashHex,
    #[error("response hash mismatch")]
    ResponseHashMismatch,
}

fn is_valid_bostrom_addr(addr: &str) -> bool {
    let len = addr.len();
    addr.starts_with("bostrom") && len >= 8 && len <= 80 && addr.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
}

fn is_score_valid(v: f64) -> bool {
    (0.0..=1.0).contains(&v) && v.is_finite()
}

fn parse_iso8601(ts: &str) -> Result<OffsetDateTime, StampValidationError> {
    OffsetDateTime::parse(ts, &time::format_description::well_known::Rfc3339)
        .map_err(|e| StampValidationError::InvalidTimestamp(e.to_string()))
}

fn is_hex_64(s: &str) -> bool {
    if s.len() != 64 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_hexdigit() && c.is_ascii_lowercase() || c.is_ascii_digit())
}

/// Deterministic SHA-256 over UTF-8 bytes of `payload`.
pub fn sha256_hex(payload: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    let digest = hasher.finalize();
    hex::encode(digest)
}

pub fn validate_stamp_basic(stamp: &AlnDidBostromStampV1) -> Result<(), StampValidationError> {
    if !is_valid_bostrom_addr(&stamp.primary_bostrom_addr) {
        return Err(StampValidationError::InvalidPrimaryAddr);
    }
    if stamp.safe_addrs.is_empty() {
        return Err(StampValidationError::EmptySafeAddrs);
    }
    if !is_score_valid(stamp.T_score_0_to_1)
        || !is_score_valid(stamp.P_score_0_to_1)
        || !is_score_valid(stamp.R_score_0_to_1)
        || !is_score_valid(stamp.C_score_0_to_1)
    {
        return Err(StampValidationError::ScoreOutOfRange);
    }
    parse_iso8601(&stamp.timestamp_utc_iso8601)?;
    if !is_hex_64(&stamp.response_hash_hex) {
        return Err(StampValidationError::InvalidResponseHashHex);
    }
    Ok(())
}

/// Validate stamp and recompute response hash over `response_payload`.
pub fn validate_stamp_with_payload(
    stamp: &AlnDidBostromStampV1,
    response_payload: &str,
) -> Result<(), StampValidationError> {
    validate_stamp_basic(stamp)?;
    let recomputed = sha256_hex(response_payload);
    if recomputed != stamp.response_hash_hex {
        return Err(StampValidationError::ResponseHashMismatch);
    }
    Ok(())
}
