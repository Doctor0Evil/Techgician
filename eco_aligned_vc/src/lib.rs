use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use hex::encode_to_slice;

/// Proposed extension of W3C Verifiable Credential v2.0 with biophysical binding
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EcoAlignedCredential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub typ: Vec<String>,
    pub issuer: String,
    pub issuanceDate: String,
    pub credentialSubject: serde_json::Value,
    pub proof: Option<Proof>,
    // Biophysical extensions
    pub ker: KER,
    pub corridor: String, // e.g., "EU-ALPS-WATER-2030" or custom budget ID
    pub wbgt_celsius: f64,
    pub eco_impact_gco2e: f64,
    pub hex_stamp: String, // base16-lowercase SHA-512 of canonical claim
}

/// KER components – all values must be provable or API-sourced
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KER {
    pub knowledge: f64,      // [0.0, 1.0]
    pub eco_impact: f64,     // gCO₂e for issuance/retrieval operation
    pub risk: f64,           // [0.0, 1.0]
}

/// Placeholder for hybrid PQ/T proof – replace with real liboqs binding in production
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proof {
    #[serde(rename = "type")]
    pub typ: String, // e.g., "ML-DSA-65" or "HybridPQT2025"
    pub created: String,
    pub proofPurpose: String,
    pub verificationMethod: String,
    pub proofValue: String, // base64url or hex signature
}

/// Stull (2011) natural wet-bulb approximation + ISO 7243 indoor WBGT
pub fn approximate_indoor_wbgt(t_celsius: f64, rh_percent: f64) -> f64 {
    use std::f64::consts::PI;
    let t = t_celsius;
    let rh = rh_percent;
    let term1 = t * (0.151977 * (rh + 8.313659).sqrt()).atan();
    let term2 = (t + rh).atan();
    let term3 = (rh - 1.676331).atan();
    let term4 = 0.00391838 * rh.powf(1.5) * (0.023101 * rh).atan();
    let tw = term1 + term2 - term3 + term4 - 4.686035;
    0.7 * tw + 0.3 * t
}

/// Simple canonicalization: sort keys, compact JSON (sufficient for proof binding)
fn canonical_json(value: &serde_json::Value) -> String {
    fn sort_object(obj: serde_json::Map<String, serde_json::Value>) -> serde_json::Map<String, serde_json::Value> {
        let mut sorted = serde_json::Map::new();
        let mut keys: Vec<String> = obj.keys().cloned().collect();
        keys.sort();
        for k in keys {
            if let Some(v) = obj.get(&k) {
                let v_sorted = match v {
                    serde_json::Value::Object(map) => serde_json::Value::Object(sort_object(map.clone())),
                    _ => v.clone(),
                };
                sorted.insert(k, v_sorted);
            }
        }
        sorted
    }
    let sorted = match value {
        serde_json::Value::Object(map) => serde_json::Value::Object(sort_object(map.clone())),
        _ => value.clone(),
    };
    serde_json::to_string(&sorted).unwrap()
}

/// Compute SHA-512 hex stamp of canonical credential (excluding mutable proof field)
pub fn compute_hex_stamp(cred: &EcoAlignedCredential) -> String {
    let mut claim = cred.clone();
    claim.proof = None; // standard binding excludes proof
    let canon = canonical_json(&serde_json::to_value(&claim).unwrap());
    let mut hasher = Sha512::new();
    hasher.update(canon.as_bytes());
    let digest = hasher.finalize();
    let mut hex_out = vec![0u8; 128];
    encode_to_slice(digest, &mut hex_out).unwrap();
    String::from_utf8(hex_out).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wbgt_example() {
        let wbgt = approximate_indoor_wbgt(35.0, 60.0);
        assert!((wbgt - 31.0).abs() < 2.0); // typical high-stress value
    }
}
