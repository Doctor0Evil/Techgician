use crate::energy_ball::EnergyBall;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sled::Db;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnergyEventType {
    Pass,
    Split,
    Merge,
    CheatActivate,
    CloneCreate,
    CloneMerge,
    StateChange,
    BridgeSync,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyEvent {
    pub id: Uuid,
    pub event_type: EnergyEventType,
    pub ball_id: Uuid,
    pub from_owner: Option<String>,
    pub to_owner: Option<String>,
    pub units_delta: i128,
    pub timestamp: chrono::DateTime<Utc>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyBlock {
    pub id: Uuid,
    pub prev_hash: String,
    pub hash: String,
    pub event: EnergyEvent,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Clone)]
pub struct EnergyLedger {
    db: Db,
    head_hash: String,
}

impl EnergyLedger {
    pub fn open(path: &str) -> Self {
        let db = sled::open(Path::new(path)).expect("EnergyLedger open failed");
        Self {
            db,
            head_hash: "GENESIS".into(),
        }
    }

    pub fn bootstrap(&mut self, ball: EnergyBall, label: &str) {
        let event = EnergyEvent {
            id: Uuid::new_v4(),
            event_type: EnergyEventType::StateChange,
            ball_id: ball.id,
            from_owner: None,
            to_owner: Some(ball.owner.clone()),
            units_delta: ball.units as i128,
            timestamp: Utc::now(),
            metadata: serde_json::json!({
                "label": label,
                "snapshot": ball.snapshot(),
            }),
        };
        let _ = self.append(event);
    }

    pub fn append(&mut self, event: EnergyEvent) -> EnergyBlock {
        let created_at = Utc::now();
        let mut hasher = Sha256::new();
        hasher.update(self.head_hash.as_bytes());
        hasher.update(serde_json::to_vec(&event).unwrap());
        hasher.update(created_at.timestamp_nanos().to_be_bytes());
        let hash = hex::encode(hasher.finalize());
        let block = EnergyBlock {
            id: Uuid::new_v4(),
            prev_hash: self.head_hash.clone(),
            hash: hash.clone(),
            event,
            created_at,
        };
        let key = hash.as_bytes().to_vec();
        let value = serde_json::to_vec(&block).unwrap();
        let _ = self.db.insert(key, value);
        self.head_hash = hash;
        block
    }

    pub fn record_pass(&mut self, ball: &EnergyBall, from: &str, to: &str) {
        let event = EnergyEvent {
            id: Uuid::new_v4(),
            event_type: EnergyEventType::Pass,
            ball_id: ball.id,
            from_owner: Some(from.into()),
            to_owner: Some(to.into()),
            units_delta: 0,
            timestamp: Utc::now(),
            metadata: serde_json::json!({
                "snapshot": ball.snapshot(),
            }),
        };
        let _ = self.append(event);
    }

    pub fn record_cheat(&mut self, ball: &EnergyBall, code: &str) {
        let event = EnergyEvent {
            id: Uuid::new_v4(),
            event_type: EnergyEventType::CheatActivate,
            ball_id: ball.id,
            from_owner: Some(ball.owner.clone()),
            to_owner: Some(ball.owner.clone()),
            units_delta: 0,
            timestamp: Utc::now(),
            metadata: serde_json::json!({
                "cheat_code": code,
                "snapshot": ball.snapshot(),
            }),
        };
        let _ = self.append(event);
    }
}
