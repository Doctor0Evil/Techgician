mod bigint;
mod energy_ball;
mod ledger;
mod contracts;
mod domains;
mod security;
mod api;

use crate::bigint::BigIntAmount;
use crate::energy_ball::{EnergyBall, EnergyClass, EnergyState};
use crate::ledger::EnergyLedger;
use crate::security::SecurityKernel;
use crate::domains::DomainRouter;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use chrono::Utc;
use uuid::Uuid;

static GLOBAL_LEDGER: Lazy<RwLock<EnergyLedger>> =
    Lazy::new(|| RwLock::new(EnergyLedger::open("data/energy_ledger")));
static GLOBAL_SECURITY: Lazy<RwLock<SecurityKernel>> =
    Lazy::new(|| RwLock::new(SecurityKernel::new()));
static GLOBAL_DOMAINS: Lazy<RwLock<DomainRouter>> =
    Lazy::new(|| RwLock::new(DomainRouter::new()));

#[tokio::main]
async fn main() {
    {
        let mut sec = GLOBAL_SECURITY.write();
        sec.set_class_clearance("Class-3", true);
        sec.register_actor("MASTER_OVERRIDE", true);
        sec.register_actor("HOME_ORCH", true);
        sec.register_actor("FIN_ORCH", true);
        sec.register_actor("TRAVEL_ORCH", true);
        sec.register_actor("SHOP_ORCH", true);
        sec.register_actor("ACADEMIC_ORCH", true);
        sec.register_actor("LIBRARY_ORCH", true);
        sec.lock_language_kernel();
    }

    {
        let mut domains = GLOBAL_DOMAINS.write();
        domains.bootstrap();
    }

    {
        let mut ledger = GLOBAL_LEDGER.write();
        let root_energy = BigIntAmount::from_str("1000000000000000000000000000");
        let root_ball = EnergyBall::new(
            "ROOT_SYSTEM",
            None,
            EnergyClass::Standard,
            EnergyState::Active,
            root_energy,
            true,
            false,
            false,
            true,
        );
        ledger.bootstrap(root_ball, "UVER-ROOT-BOOT");
    }

    api::serve_api().await;
}
