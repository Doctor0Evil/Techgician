use crate::energy_ball::EnergyBall;
use crate::bigint::BigIntAmount;
use chrono::{DateTime, Utc};
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
    ContractExecution,
    DomainSpend,
    BridgeSync,
    CheatActivation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyEvent {
    pub id: Uuid,
    pub event_type: EnergyEventType,
    pub ball_id: Uuid,
    pub from: Option<String>,
    pub to: Option<String>,
    pub delta_units: BigIntAmount,
    pub timestamp: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyBlock {
    pub id: Uuid,
    pub prev_hash: String,
    pub hash: String,
    pub event: EnergyEvent,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct EnergyLedger {
    db: Db,
    head_hash: String,
}

impl EnergyLedger {
    pub fn open(path: &str) -> Self {
        let db = sled::open(Path::new(path)).expect("open energy ledger");
        Self {
            db,
            head_hash: "GENESIS".into(),
        }
    }

    fn append(&mut self, event: EnergyEvent) -> EnergyBlock {
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
        let val = serde_json::to_vec(&block).unwrap();
        let _ = self.db.insert(key, val);
        self.head_hash = hash;
        block
    }

    pub fn bootstrap(&mut self, ball: EnergyBall, label: &str) {
        let evt = EnergyEvent {
            id: Uuid::new_v4(),
            event_type: EnergyEventType::ContractExecution,
            ball_id: ball.id,
            from: None,
            to: Some(ball.owner.clone()),
            delta_units: ball.units.clone(),
            timestamp: Utc::now(),
            metadata: serde_json::json!({
                "label": label,
                "snapshot": ball.snapshot()
            }),
        };
        let _ = self.append(evt);
    }

    pub fn record_pass(&mut self, ball: &EnergyBall, from: &str, to: &str) {
        let evt = EnergyEvent {
            id: Uuid::new_v4(),
            event_type: EnergyEventType::Pass,
            ball_id: ball.id,
            from: Some(from.into()),
            to: Some(to.into()),
            delta_units: BigIntAmount::zero(),
            timestamp: Utc::now(),
            metadata: serde_json::json!({
                "snapshot": ball.snapshot()
            }),
        };
        let _ = self.append(evt);
    }

    pub fn record_cheat(&mut self, ball: &EnergyBall, code: &str) {
        let evt = EnergyEvent {
            id: Uuid::new_v4(),
            event_type: EnergyEventType::CheatActivation,
            ball_id: ball.id,
            from: Some(ball.owner.clone()),
            to: Some(ball.owner.clone()),
            delta_units: BigIntAmount::zero(),
            timestamp: Utc::now(),
            metadata: serde_json::json!({
                "cheat_code": code,
                "snapshot": ball.snapshot()
            }),
        };
        let _ = self.append(evt);
    }

    pub fn len(&self) -> usize {
        self.db.len()
    }
}
