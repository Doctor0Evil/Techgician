use crate::bigint::BigIntAmount;
use crate::energy_ball::EnergyBall;
use chrono::{DateTime, Utc};
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
    pub total_energy: BigIntAmount,
    pub total_events: u64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone)]
pub struct DomainRouter {
    stats: HashMap<Domain, DomainStats>,
}

impl DomainRouter {
    pub fn new() -> Self {
        Self { stats: HashMap::new() }
    }

    pub fn bootstrap(&mut self) {
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
                    total_energy: BigIntAmount::zero(),
                    total_events: 0,
                    last_updated: Utc::now(),
                },
            );
        }
    }

    pub fn spend(
        &mut self,
        domain: &Domain,
        ball: &mut EnergyBall,
        units: &BigIntAmount,
    ) -> bool {
        if !ball.drain(units) {
            return false;
        }
        if let Some(stat) = self.stats.get_mut(domain) {
            stat.total_energy += units.clone();
            stat.total_events += 1;
            stat.last_updated = Utc::now();
        }
        true
    }

    pub fn snapshot(&self) -> serde_json::Value {
        let mut rows = Vec::new();
        for s in self.stats.values() {
            rows.push(serde_json::json!({
                "domain": format!("{:?}", s.domain),
                "total_energy": s.total_energy.as_str(),
                "total_events": s.total_events,
                "last_updated": s.last_updated,
            }));
        }
        serde_json::json!({
            "timestamp": Utc::now(),
            "domains": rows,
        })
    }
}
