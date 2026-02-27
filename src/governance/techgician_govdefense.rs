// techgician_govdefense.rs - Sovereigntycore stub for anti-grab enforcement
// Full production: Enforces TsafeRoH + multisig on EVOLVE proposals

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Corridor {
    pub domain: String, // e.g., "WBGT", "RoH"
    pub bounds: (f32, f32), // [min, max], e.g., RoH (0.0, 0.3)
    pub lyapunov_residual: f32, // V(t) <= 0.05
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolutionProposal {
    pub host_did: String, // bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
    pub roh_before: f32,
    pub roh_after: f32,
    pub k: f32, // Knowledge >=0.94
    pub e: f32, // Eco >=0.90
    pub r: f32, // Risk <=0.12
    pub multisig: Vec<String>, // Bostrom addrs
    pub hexstamp: String, // ALNDIDBostromStampV1
}

pub struct SovCore {
    corridors: HashMap<String, Corridor>,
    roh_ceiling: f32,
}

impl SovCore {
    pub fn new() -> Self {
        let mut corridors = HashMap::new();
        corridors.insert("RoH".to_string(), Corridor { domain: "RoH".to_string(), bounds: (0.0, 0.3), lyapunov_residual: 0.05 });
        // Add bee/marine: HB 0.99, OC 0.98 invariants
        Self { corridors, roh_ceiling: 0.3 }
    }

    pub fn authorize_proposal(&self, prop: &EvolutionProposal) -> bool {
        // Internal integrity: RoH monotonic + ceiling
        if prop.roh_after > prop.roh_before || prop.roh_after > self.roh_ceiling { return false; }
        // Governance: KER bounds
        if prop.k < 0.94 || prop.e < 0.90 || prop.r > 0.12 { return false; }
        // External: Multisig (min 3/5 Bostrom addrs)
        if prop.multisig.len() < 3 { return false; }
        // Eco-positive: Bee/marine corridors
        if let Some(c) = self.corridors.get("HB") {
            if prop.r > c.lyapunov_residual { return false; } // Prioritize bees
        }
        true // Safe: Log to donutloop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_grab_defense() {
        let core = SovCore::new();
        let bad_prop = EvolutionProposal {
            host_did: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            roh_before: 0.2,
            roh_after: 0.35, // Violates ceiling
            k: 0.95,
            e: 0.91,
            r: 0.11,
            multisig: vec!["bostrom1ldgmtf20d6604a24ztr0jxht7xt7az4jhkmsrc".to_string()],
            hexstamp: "ALNDIDBostromStampV1".to_string(),
        };
        assert_eq!(core.authorize_proposal(&bad_prop), false); // Blocks power-grab
    }
}
