use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyAccessRule {
    pub entity_id: String,
    pub allowed: bool,
}

#[derive(Clone)]
pub struct SecurityKernel {
    clearance: HashMap<String, bool>,
    energy_access: HashMap<String, EnergyAccessRule>,
    language_locked: bool,
}

impl SecurityKernel {
    pub fn new() -> Self {
        Self {
            clearance: HashMap::new(),
            energy_access: HashMap::new(),
            language_locked: true,
        }
    }

    pub fn set_class_clearance(&mut self, class: &str, allowed: bool) {
        self.clearance.insert(class.into(), allowed);
    }

    pub fn has_clearance(&self, class: &str) -> bool {
        *self.clearance.get(class).unwrap_or(&false)
    }

    pub fn enable_energy_access_control(&mut self) {
        self.language_locked = true;
    }

    pub fn register_energy_actor(&mut self, id: &str, allowed: bool) {
        self.energy_access.insert(
            id.into(),
            EnergyAccessRule {
                entity_id: id.into(),
                allowed,
            },
        );
    }

    pub fn can_move_energy(&self, actor: &str) -> bool {
        self.energy_access
            .get(actor)
            .map(|r| r.allowed)
            .unwrap_or(false)
    }

    pub fn energy_forensics_manifest(&self) -> serde_json::Value {
        json!({
            "timestamp": Utc::now(),
            "clearance_classes": self.clearance.keys().collect::<Vec<_>>(),
            "language_locked": self.language_locked,
            "actors": self.energy_access.keys().collect::<Vec<_>>(),
        })
    }
}
