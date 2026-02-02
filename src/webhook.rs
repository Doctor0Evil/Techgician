use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyWebhookPayload {
    pub event: String,
    pub user_id: String,
    pub bridge_url: String,
    pub domain: String,
    pub action: String,
    pub timestamp: String,
}

impl EnergyWebhookPayload {
    pub fn example_bridge() -> Self {
        Self {
            event: "energy_ball_bridge".into(),
            user_id: "a254c6b4-ee23-444a-9b1a-95f32cadd643".into(),
            bridge_url: "https://api2.element.market/bridge/wallet".into(),
            domain: "Finance".into(),
            action: "increase".into(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}
