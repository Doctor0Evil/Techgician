use crate::energy_ball::EnergyBall;
use chrono::Utc;
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
    pub created_at: chrono::DateTime<Utc>,
}

impl EnergyContract {
    pub fn new(scope: ContractScope, description: &str, condition: serde_json::Value, action: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            scope,
            description: description.into(),
            condition,
            action,
            created_at: Utc::now(),
        }
    }

    pub fn should_trigger(&self, ctx: &serde_json::Value) -> bool {
        // high-level placeholder: match on fields in ctx/condition
        if let Some(threshold) = self.condition.get("energy_gt") {
            if let Some(units) = ctx.get("units") {
                if let (Some(t), Some(u)) = (threshold.as_str(), units.as_str()) {
                    if let (Ok(t_val), Ok(u_val)) = (t.parse::<u128>(), u.parse::<u128>()) {
                        return u_val > t_val;
                    }
                }
            }
        }
        false
    }

    pub fn apply(&self, ball: &mut EnergyBall) {
        if let Some(delta) = self.action.get("delta_units") {
            if let Some(d) = delta.as_str() {
                if let Ok(v) = d.parse::<i128>() {
                    if v > 0 {
                        ball.charge_up(v as u128);
                    } else if v < 0 {
                        let _ = ball.drain((-v) as u128);
                    }
                }
            }
        }
    }
}
