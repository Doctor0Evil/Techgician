use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorRule {
    pub id: String,
    pub allowed: bool,
}

#[derive(Clone)]
pub struct SecurityKernel {
    clearance: HashMap<String, bool>,
    actors: HashMap<String, ActorRule>,
    language_locked: bool,
}

impl SecurityKernel {
    pub fn new() -> Self {
        Self {
            clearance: HashMap::new(),
            actors: HashMap::new(),
            language_locked: false,
        }
    }

    pub fn set_class_clearance(&mut self, class: &str, allowed: bool) {
        self.clearance.insert(class.into(), allowed);
    }

    pub fn has_clearance(&self, class: &str) -> bool {
        *self.clearance.get(class).unwrap_or(&false)
    }

    pub fn register_actor(&mut self, id: &str, allowed: bool) {
        self.actors.insert(
            id.into(),
            ActorRule {
                id: id.into(),
                allowed,
            },
        );
    }

    pub fn can_act(&self, id: &str) -> bool {
        self.actors.get(id).map(|r| r.allowed).unwrap_or(false)
    }

    pub fn lock_language_kernel(&mut self) {
        self.language_locked = true;
    }

    pub fn manifest(&self) -> serde_json::Value {
        json!({
            "timestamp": Utc::now(),
            "language_locked": self.language_locked,
            "classes": self.clearance.keys().collect::<Vec<_>>(),
            "actors": self.actors.keys().collect::<Vec<_>>(),
        })
    }
}
