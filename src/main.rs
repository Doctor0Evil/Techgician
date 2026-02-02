mod energy_ball;
mod ledger;
mod domains;
mod security;
mod api;
mod webhook;
mod contracts;

use crate::energy_ball::{EnergyBall, EnergyBallClass, EnergyState};
use crate::ledger::EnergyLedger;
use crate::domains::DomainRouter;
use crate::security::SecurityKernel;
use crate::api::serve_api;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use chrono::Utc;
use uuid::Uuid;

static GLOBAL_LEDGER: Lazy<RwLock<EnergyLedger>> =
    Lazy::new(|| RwLock::new(EnergyLedger::open("data/energy_chain")));
static GLOBAL_ROUTER: Lazy<RwLock<DomainRouter>> =
    Lazy::new(|| RwLock::new(DomainRouter::new()));
static GLOBAL_SECURITY: Lazy<RwLock<SecurityKernel>> =
    Lazy::new(|| RwLock::new(SecurityKernel::new()));

#[tokio::main]
async fn main() {
    {
        let mut sec = GLOBAL_SECURITY.write();
        sec.set_class_clearance("Class-3", true);
        sec.enable_energy_access_control();
    }

    {
        let mut router = GLOBAL_ROUTER.write();
        router.bootstrap_domains();
    }

    {
        let mut ledger = GLOBAL_LEDGER.write();
        let boot_ball = EnergyBall::new_root(
            "SYSTEM",
            EnergyBallClass::Standard,
            1_000_000_000,
        );
        ledger.bootstrap(boot_ball, "XR-GRID:UVER:BOOT");
    }

    serve_api().await;
}
