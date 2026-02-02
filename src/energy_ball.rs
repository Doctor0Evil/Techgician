use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnergyBallClass {
    Standard,
    Overcharged,
    Clone,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnergyState {
    Idle,
    Active,
    Overcharged,
    Depleted,
    Frozen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyBall {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub owner: String,
    pub class: EnergyBallClass,
    pub state: EnergyState,
    pub units: u128,
    pub max_duration_unlimited: bool,
    pub expires: bool,
    pub revocable: bool,
    pub will_persist: bool,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

impl EnergyBall {
    pub fn new_root(owner: &str, class: EnergyBallClass, units: u128) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            parent_id: None,
            owner: owner.into(),
            class,
            state: EnergyState::Active,
            units,
            max_duration_unlimited: true,
            expires: false,
            revocable: false,
            will_persist: true,
            created_at: now,
            last_updated_at: now,
            metadata: serde_json::json!({
                "domains": [],
                "tags": ["ROOT", "UVER"],
            }),
        }
    }

    pub fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "parent_id": self.parent_id,
            "owner": self.owner,
            "class": format!("{:?}", self.class),
            "state": format!("{:?}", self.state),
            "units": self.units.to_string(),
            "max_duration_unlimited": self.max_duration_unlimited,
            "expires": self.expires,
            "revocable": self.revocable,
            "will_persist": self.will_persist,
            "created_at": self.created_at,
            "last_updated_at": self.last_updated_at,
            "metadata": self.metadata,
        })
    }

    pub fn split(&mut self, amount: u128, new_owner: &str) -> Option<EnergyBall> {
        if self.units < amount {
            return None;
        }
        self.units -= amount;
        self.last_updated_at = Utc::now();
        let now = Utc::now();
        Some(Self {
            id: Uuid::new_v4(),
            parent_id: Some(self.id),
            owner: new_owner.into(),
            class: EnergyBallClass::Standard,
            state: EnergyState::Active,
            units: amount,
            max_duration_unlimited: self.max_duration_unlimited,
            expires: self.expires,
            revocable: self.revocable,
            will_persist: self.will_persist,
            created_at: now,
            last_updated_at: now,
            metadata: serde_json::json!({
                "domains": [],
                "tags": ["SPLIT"],
            }),
        })
    }

    pub fn merge(&mut self, other: &EnergyBall) {
        self.units += other.units;
        self.last_updated_at = Utc::now();
        self.state = EnergyState::Active;
    }

    pub fn charge_up(&mut self, extra: u128) {
        self.units += extra;
        self.state = EnergyState::Overcharged;
        self.last_updated_at = Utc::now();
    }

    pub fn drain(&mut self, amount: u128) -> bool {
        if self.units < amount {
            return false;
        }
        self.units -= amount;
        if self.units == 0 {
            self.state = EnergyState::Depleted;
        } else {
            self.state = EnergyState::Active;
        }
        self.last_updated_at = Utc::now();
        true
    }

    pub fn clone_temporary(&self, ttl_seconds: i64) -> EnergyBall {
        let now = Utc::now();
        let mut meta = self.metadata.clone();
        meta["clone_ttl_seconds"] = serde_json::json!(ttl_seconds);
        meta["clone_origin_id"] = serde_json::json!(self.id);
        EnergyBall {
            id: Uuid::new_v4(),
            parent_id: Some(self.id),
            owner: self.owner.clone(),
            class: EnergyBallClass::Clone,
            state: EnergyState::Active,
            units: self.units,
            max_duration_unlimited: false,
            expires: true,
            revocable: true,
            will_persist: false,
            created_at: now,
            last_updated_at: now,
            metadata: meta,
        }
    }

    pub fn growth_protocol(&mut self, unused_compute: u128, under_attack: bool) {
        if under_attack {
            let decay = (self.units / 10).max(1);
            self.units = self.units.saturating_sub(decay);
            self.state = EnergyState::Frozen;
        } else {
            self.units = self.units.saturating_add(unused_compute);
            self.state = EnergyState::Overcharged;
        }
        self.last_updated_at = Utc::now();
    }

    pub fn random_state_transition(&mut self) {
        // optional, for gamified behavior
        let roll: u8 = rand::thread_rng().gen_range(0..100);
        if roll < 2 {
            self.state = EnergyState::Overcharged;
        } else if roll > 98 {
            self.state = EnergyState::Idle;
        }
        self.last_updated_at = Utc::now();
    }
}
