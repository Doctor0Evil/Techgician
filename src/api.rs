use crate::energy_ball::{EnergyBall, EnergyBallClass};
use crate::ledger::{EnergyLedger, EnergyEventType, EnergyEvent};
use crate::domains::{DomainRouter, Domain};
use crate::security::SecurityKernel;
use crate::{GLOBAL_LEDGER, GLOBAL_ROUTER, GLOBAL_SECURITY};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use warp::Filter;

#[derive(Deserialize)]
struct PassRequest {
    actor: String,
    from_owner: String,
    to_owner: String,
    ball_id: String,
}

#[derive(Deserialize)]
struct CheatRequest {
    actor: String,
    ball_id: String,
    cheat_code: String,
}

#[derive(Deserialize)]
struct DomainSpendRequest {
    actor: String,
    ball_id: String,
    domain: String,
    units: String,
}

#[derive(Serialize)]
struct EnergyStatus {
    timestamp: String,
    total_blocks: usize,
    security_manifest: serde_json::Value,
    domain_stats: serde_json::Value,
}

pub async fn serve_api() {
    let pass_route = warp::path!("energy" / "pass")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_pass);

    let cheat_route = warp::path!("energy" / "cheat")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_cheat);

    let spend_route = warp::path!("energy" / "spend")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_spend);

    let status_route = warp::path!("energy" / "status")
        .and(warp::get())
        .and_then(handle_status);

    let routes = pass_route.or(cheat_route).or(spend_route).or(status_route);
    warp::serve(routes).run(([127, 0, 0, 1], 8090)).await;
}

async fn handle_pass(req: PassRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let security = GLOBAL_SECURITY.read().clone();
    if !security.has_clearance("Class-3") || !security.can_move_energy(&req.actor) {
        return Ok(warp::reply::json(&json!({ "ok": false, "error": "ACCESS_DENIED" })));
    }

    let ball_id = Uuid::parse_str(&req.ball_id).map_err(|_| warp::reject())?;
    let mut ledger = GLOBAL_LEDGER.write();
    let mut ball = EnergyBall {
        id: ball_id,
        parent_id: None,
        owner: req.to_owner.clone(),
        class: EnergyBallClass::Standard,
        state: crate::energy_ball::EnergyState::Active,
        units: 0,
        max_duration_unlimited: true,
        expires: false,
        revocable: false,
        will_persist: true,
        created_at: Utc::now(),
        last_updated_at: Utc::now(),
        metadata: json!({ "tags": ["PASS_EVENT"] }),
    };

    ledger.record_pass(&ball, &req.from_owner, &req.to_owner);

    Ok(warp::reply::json(&json!({
        "ok": true,
        "event": "PASS_RECORDED",
        "ball_snapshot": ball.snapshot()
    })))
}

async fn handle_cheat(req: CheatRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let security = GLOBAL_SECURITY.read().clone();
    if !security.has_clearance("Class-3") || !security.can_move_energy(&req.actor) {
        return Ok(warp::reply::json(&json!({ "ok": false, "error": "ACCESS_DENIED" })));
    }

    let ball_id = Uuid::parse_str(&req.ball_id).map_err(|_| warp::reject())?;
    let mut ledger = GLOBAL_LEDGER.write();
    let mut ball = EnergyBall {
        id: ball_id,
        parent_id: None,
        owner: req.actor.clone(),
        class: EnergyBallClass::Overcharged,
        state: crate::energy_ball::EnergyState::Overcharged,
        units: 1_000_000_000_000,
        max_duration_unlimited: true,
        expires: false,
        revocable: false,
        will_persist: true,
        created_at: Utc::now(),
        last_updated_at: Utc::now(),
        metadata: json!({ "tags": ["CHEAT", req.cheat_code] }),
    };

    ledger.record_cheat(&ball, &req.cheat_code);

    Ok(warp::reply::json(&json!({
        "ok": true,
        "event": "CHEAT_ACTIVATED",
        "ball_snapshot": ball.snapshot()
    })))
}

async fn handle_spend(req: DomainSpendRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let security = GLOBAL_SECURITY.read().clone();
    if !security.has_clearance("Class-3") || !security.can_move_energy(&req.actor) {
        return Ok(warp::reply::json(&json!({ "ok": false, "error": "ACCESS_DENIED" })));
    }

    let ball_id = Uuid::parse_str(&req.ball_id).map_err(|_| warp::reject())?;
    let units: u128 = req.units.parse().map_err(|_| warp::reject())?;
    let domain = match req.domain.as_str() {
        "Home" => Domain::Home,
        "Finance" => Domain::Finance,
        "Travel" => Domain::Travel,
        "Shopping" => Domain::Shopping,
        "Academic" => Domain::Academic,
        "Library" => Domain::Library,
        _ => return Ok(warp::reply::json(&json!({ "ok": false, "error": "UNKNOWN_DOMAIN" }))),
    };

    let mut router = GLOBAL_ROUTER.write();
    let mut ball = EnergyBall {
        id: ball_id,
        parent_id: None,
        owner: req.actor.clone(),
        class: EnergyBallClass::Standard,
        state: crate::energy_ball::EnergyState::Active,
        units,
        max_duration_unlimited: true,
        expires: false,
        revocable: false,
        will_persist: true,
        created_at: Utc::now(),
        last_updated_at: Utc::now(),
        metadata: json!({ "tags": ["DOMAIN_SPEND", req.domain] }),
    };

    let ok = router.route_energy(domain, &mut ball, units);
    if !ok {
        return Ok(warp::reply::json(&json!({ "ok": false, "error": "INSUFFICIENT_UNITS" })));
    }

    Ok(warp::reply::json(&json!({
        "ok": true,
        "event": "DOMAIN_SPEND",
        "ball_state": format!("{:?}", ball.state),
        "domain_stats": router.domain_snapshot()
    })))
}

async fn handle_status() -> Result<impl warp::Reply, warp::Rejection> {
    let ledger = GLOBAL_LEDGER.read();
    let router = GLOBAL_ROUTER.read();
    let security = GLOBAL_SECURITY.read();

    let total_blocks = ledger.db.len();

    let resp = EnergyStatus {
        timestamp: Utc::now().to_rfc3339(),
        total_blocks,
        security_manifest: security.energy_forensics_manifest(),
        domain_stats: router.domain_snapshot(),
    };

    Ok(warp::reply::json(&resp))
}
