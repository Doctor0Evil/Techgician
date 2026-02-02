use crate::energy_ball::{EnergyBall, EnergyState};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Domain {
    Home,
    Finance,
    Travel,
    Shopping,
    Academic,
    Library,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainStats {
    pub domain: Domain,
    pub total_energy_consumed: u128,
    pub total_events: u64,
    pub last_updated: chrono::DateTime<Utc>,
}

#[derive(Clone)]
pub struct DomainRouter {
    stats: HashMap<Domain, DomainStats>,
}

impl DomainRouter {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn bootstrap_domains(&mut self) {
        for d in [
            Domain::Home,
            Domain::Finance,
            Domain::Travel,
            Domain::Shopping,
            Domain::Academic,
            Domain::Library,
        ] {
            self.stats.insert(
                d.clone(),
                DomainStats {
                    domain: d.clone(),
                    total_energy_consumed: 0,
                    total_events: 0,
                    last_updated: Utc::now(),
                },
            );
        }
    }

    pub fn route_energy(
        &mut self,
        domain: Domain,
        ball: &mut EnergyBall,
        requested_units: u128,
    ) -> bool {
        if !ball.drain(requested_units) {
            return false;
        }
        if let Some(stat) = self.stats.get_mut(&domain) {
            stat.total_energy_consumed += requested_units;
            stat.total_events += 1;
            stat.last_updated = Utc::now();
        }
        true
    }

    pub fn domain_snapshot(&self) -> serde_json::Value {
        let mut rows = Vec::new();
        for stat in self.stats.values() {
            rows.push(serde_json::json!({
                "domain": format!("{:?}", stat.domain),
                "total_energy_consumed": stat.total_energy_consumed.to_string(),
                "total_events": stat.total_events,
                "last_updated": stat.last_updated,
            }));
        }
        serde_json::json!({
            "timestamp": Utc::now(),
            "domains": rows,
        })
    }

    pub fn apply_growth(
        &mut self,
        ball: &mut EnergyBall,
        unused_compute: u128,
        under_attack: bool,
    ) {
        ball.growth_protocol(unused_compute, under_attack);
    }

    pub fn clone_for_redundancy(&self, ball: &EnergyBall, ttl_seconds: i64) -> EnergyBall {
        ball.clone_temporary(ttl_seconds)
    }

    pub fn visualize_matrix(&self, balls: &[EnergyBall]) -> serde_json::Value {
        let mut matrix = Vec::new();
        for b in balls {
            matrix.push(serde_json::json!({
                "id": b.id,
                "owner": b.owner,
                "class": format!("{:?}", b.class),
                "state": format!("{:?}", b.state),
                "units": b.units.to_string(),
            }));
        }
        serde_json::json!({
            "timestamp": Utc::now(),
            "matrix": matrix,
            "domain_stats": self.domain_snapshot(),
        })
    }

    pub fn gamified_achievement(&self) -> serde_json::Value {
        let mut achievements = Vec::new();
        for stat in self.stats.values() {
            let title = if stat.total_energy_consumed > 1_000_000 {
                "Energy Tycoon"
            } else if stat.total_events > 10_000 {
                "Infinite Loop Master"
            } else {
                "Rising Operator"
            };
            achievements.push(serde_json::json!({
                "domain": format!("{:?}", stat.domain),
                "title": title,
                "total_energy_consumed": stat.total_energy_consumed.to_string(),
                "total_events": stat.total_events,
            }));
        }
        serde_json::json!({
            "timestamp": Utc::now(),
            "achievements": achievements,
        })
    }
}
