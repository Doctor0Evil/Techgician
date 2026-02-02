use crate::bigint::BigIntAmount;
use crate::energy_ball::{EnergyBall, EnergyClass, EnergyState};
use crate::ledger::{EnergyLedger};
use crate::domains::{DomainRouter, Domain};
use crate::security::SecurityKernel;
use crate::{GLOBAL_LEDGER, GLOBAL_DOMAINS, GLOBAL_SECURITY};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use warp::Filter;

#[derive(Deserialize)]
struct PassReq {
    actor: String,
    from: String,
    to: String,
    ball_units: String,
}

#[derive(Deserialize)]
struct DomainSpendReq {
    actor: String,
    domain: String,
    ball_units: String,
    spend_units: String,
}

#[derive(Deserialize)]
struct CheatReq {
    actor: String,
    cheat_code: String,
    ball_units: String,
}

#[derive(Serialize)]
struct StatusResp {
    timestamp: String,
    blocks: usize,
    security: serde_json::Value,
    domains: serde_json::Value,
}

pub async fn serve_api() {
    let pass_route = warp::path!("energy" / "pass")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_pass);

    let spend_route = warp::path!("energy" / "spend")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_spend);

    let cheat_route = warp::path!("energy" / "cheat")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_cheat);

    let status_route = warp::path!("energy" / "status")
        .and(warp::get())
        .and_then(handle_status);

    let routes = pass_route.or(spend_route).or(cheat_route).or(status_route);
    warp::serve(routes).run(([127, 0, 0, 1], 8091)).await;
}

async fn handle_pass(req: PassReq) -> Result<impl warp::Reply, warp::Rejection> {
    let security: SecurityKernel = GLOBAL_SECURITY.read().clone();
    if !security.has_clearance("Class-3") || !security.can_act(&req.actor) {
        return Ok(warp::reply::json(&json!({"ok": false, "error": "ACCESS_DENIED"})));
    }

    let units = BigIntAmount::from_str(&req.ball_units);
    let ball = EnergyBall::new(
        &req.to,
        None,
        EnergyClass::Standard,
        EnergyState::Active,
        units,
        true,
        false,
        false,
        true,
    );

    let mut ledger = GLOBAL_LEDGER.write();
    ledger.record_pass(&ball, &req.from, &req.to);

    Ok(warp::reply::json(&json!({
        "ok": true,
        "event": "PASS_RECORDED",
        "ball": ball.snapshot()
    })))
}

async fn handle_spend(req: DomainSpendReq) -> Result<impl warp::Reply, warp::Rejection> {
    let security: SecurityKernel = GLOBAL_SECURITY.read().clone();
    if !security.has_clearance("Class-3") || !security.can_act(&req.actor) {
        return Ok(warp::reply::json(&json!({"ok": false, "error": "ACCESS_DENIED"})));
    }

    let domain = match req.domain.as_str() {
        "Home" => Domain::Home,
        "Finance" => Domain::Finance,
        "Travel" => Domain::Travel,
        "Shopping" => Domain::Shopping,
        "Academic" => Domain::Academic,
        "Library" => Domain::Library,
        _ => return Ok(warp::reply::json(&json!({"ok": false, "error": "UNKNOWN_DOMAIN"}))),
    };

    let ball_units = BigIntAmount::from_str(&req.ball_units);
    let spend_units = BigIntAmount::from_str(&req.spend_units);

    let mut ball = EnergyBall::new(
        &req.actor,
        None,
        EnergyClass::Standard,
        EnergyState::Active,
        ball_units,
        true,
        false,
        false,
        true,
    );

    let mut domains: DomainRouter = GLOBAL_DOMAINS.write().clone();
    let ok = domains.spend(&domain, &mut ball, &spend_units);
    if !ok {
        return Ok(warp::reply::json(&json!({"ok": false, "error": "INSUFFICIENT_UNITS"})));
    }

    Ok(warp::reply::json(&json!({
        "ok": true,
        "event": "DOMAIN_SPEND",
        "domain": format!("{:?}", domain),
        "ball_state": format!("{:?}", ball.state),
        "domain_stats": domains.snapshot()
    })))
}

async fn handle_cheat(req: CheatReq) -> Result<impl warp::Reply, warp::Rejection> {
    let security: SecurityKernel = GLOBAL_SECURITY.read().clone();
    if !security.has_clearance("Class-3") || !security.can_act(&req.actor) {
        return Ok(warp::reply::json(&json!({"ok": false, "error": "ACCESS_DENIED"})));
    }

    let mut ball = EnergyBall::new(
        &req.actor,
        None,
        EnergyClass::Overcharged,
        EnergyState::Overcharged,
        BigIntAmount::from_str(&req.ball_units),
        true,
        false,
        false,
        true,
    );

    let mut ledger = GLOBAL_LEDGER.write();
    ledger.record_cheat(&ball, &req.cheat_code);

    Ok(warp::reply::json(&json!({
        "ok": true,
        "event": "CHEAT_ACTIVATED",
        "cheat_code": req.cheat_code,
        "ball": ball.snapshot()
    })))
}

async fn handle_status() -> Result<impl warp::Reply, warp::Rejection> {
    let ledger = GLOBAL_LEDGER.read();
    let security = GLOBAL_SECURITY.read();
    let domains = GLOBAL_DOMAINS.read();

    let resp = StatusResp {
        timestamp: Utc::now().to_rfc3339(),
        blocks: ledger.len(),
        security: security.manifest(),
        domains: domains.snapshot(),
    };

    Ok(warp::reply::json(&resp))
}
