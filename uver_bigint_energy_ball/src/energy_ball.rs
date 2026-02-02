use crate::bigint::BigIntAmount;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnergyClass {
    Standard,
    Overcharged,
    Clone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub class: EnergyClass,
    pub state: EnergyState,
    pub units: BigIntAmount,
    pub max_duration_unlimited: bool,
    pub expires: bool,
    pub revocable: bool,
    pub will_persist: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

impl EnergyBall {
    pub fn new(
        owner: &str,
        parent_id: Option<Uuid>,
        class: EnergyClass,
        state: EnergyState,
        units: BigIntAmount,
        max_duration_unlimited: bool,
        expires: bool,
        revocable: bool,
        will_persist: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            parent_id,
            owner: owner.into(),
            class,
            state,
            units,
            max_duration_unlimited,
            expires,
            revocable,
            will_persist,
            created_at: now,
            updated_at: now,
            metadata: serde_json::json!({ "domains": [], "tags": [] }),
        }
    }

    pub fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "parent_id": self.parent_id,
            "owner": self.owner,
            "class": format!("{:?}", self.class),
            "state": format!("{:?}", self.state),
            "units": self.units.as_str(),
            "max_duration_unlimited": self.max_duration_unlimited,
            "expires": self.expires,
            "revocable": self.revocable,
            "will_persist": self.will_persist,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
            "metadata": self.metadata,
        })
    }

    pub fn split(&mut self, amount: &BigIntAmount, new_owner: &str) -> Option<EnergyBall> {
        if &self.units.inner() < amount.inner() {
            return None;
        }
        self.units -= amount.clone();
        self.updated_at = Utc::now();
        Some(EnergyBall::new(
            new_owner,
            Some(self.id),
            EnergyClass::Standard,
            EnergyState::Active,
            amount.clone(),
            self.max_duration_unlimited,
            self.expires,
            self.revocable,
            self.will_persist,
        ))
    }

    pub fn merge(&mut self, other: &EnergyBall) {
        self.units += other.units.clone();
        self.state = EnergyState::Active;
        self.updated_at = Utc::now();
    }

    pub fn charge_up(&mut self, extra: &BigIntAmount) {
        self.units += extra.clone();
        self.state = EnergyState::Overcharged;
        self.updated_at = Utc::now();
    }

    pub fn drain(&mut self, amount: &BigIntAmount) -> bool {
        if &self.units.inner() < amount.inner() {
            return false;
        }
        self.units -= amount.clone();
        if self.units.inner().is_zero() {
            self.state = EnergyState::Depleted;
        } else {
            self.state = EnergyState::Active;
        }
        self.updated_at = Utc::now();
        true
    }

    pub fn clone_temporary(&self, ttl_seconds: i64) -> EnergyBall {
        let mut meta = self.metadata.clone();
        meta["clone_ttl_seconds"] = serde_json::json!(ttl_seconds);
        meta["clone_origin"] = serde_json::json!(self.id);
        EnergyBall::new(
            &self.owner,
            Some(self.id),
            EnergyClass::Clone,
            EnergyState::Active,
            self.units.clone(),
            false,
            true,
            true,
            false,
        )
    }
}
