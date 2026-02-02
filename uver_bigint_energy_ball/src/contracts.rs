use crate::energy_ball::EnergyBall;
use crate::bigint::BigIntAmount;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractScope {
    OnChain,
    OffChain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyContract {
    pub id: Uuid,
    pub scope: ContractScope,
    pub description: String,
    pub condition: serde_json::Value,
    pub action: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl EnergyContract {
    pub fn new(
        scope: ContractScope,
        description: &str,
        condition: serde_json::Value,
        action: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            scope,
            description: description.into(),
            condition,
            action,
            created_at: Utc::now(),
        }
    }

    pub fn should_trigger(&self, ball: &EnergyBall) -> bool {
        if let Some(threshold) = self.condition.get("units_gt") {
            if let Some(t_str) = threshold.as_str() {
                let threshold_amt = BigIntAmount::from_str(t_str);
                return ball.units.inner() > threshold_amt.inner();
            }
        }
        false
    }

    pub fn apply(&self, ball: &mut EnergyBall) {
        if let Some(delta) = self.action.get("delta_units") {
            if let Some(delta_str) = delta.as_str() {
                let amt = BigIntAmount::from_str(delta_str);
                if amt.is_negative() {
                    let zero = BigIntAmount::zero();
                    let target = zero.saturating_sub(&amt);
                    let _ = ball.drain(&target);
                } else {
                    ball.charge_up(&amt);
                }
            }
        }
    }
}
