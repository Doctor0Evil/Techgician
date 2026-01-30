use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlnDidBostromStampV1 {
    pub author_system: String,
    pub primary_bostrom_addr: String,
    pub alt_bostrom_addr: Option<String>,
    pub safe_addrs: Vec<String>,
    pub response_hash_hex: String,
    pub T_score_0_to_1: f64,
    pub P_score_0_to_1: f64,
    pub R_score_0_to_1: f64,
    pub C_score_0_to_1: f64,
    pub timestamp_utc_iso8601: String,
    pub notes: Option<String>,
}
