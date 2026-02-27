use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

use sovereigntycore::{EvaluationContext, EvolutionProposal, SovereigntyCore, Decision, DecisionReason};

#[repr(C)]
pub struct sovereignty_core_handle_t {
    inner: SovereigntyCore,
}

#[repr(C)]
pub enum sov_decision_t {
    SOV_DECISION_ALLOWED = 0,
    SOV_DECISION_REJECTED = 1,
    SOV_DECISION_DEFERRED = 2,
}

#[repr(C)]
pub enum sov_reason_t {
    SOV_REASON_OK = 0,
    SOV_REASON_ROH_CEILING = 1,
    SOV_REASON_NEURORIGHTS = 2,
    SOV_REASON_STAKE_MULTISIG = 3,
    SOV_REASON_TOKEN_SCOPE = 4,
    SOV_REASON_LIFEFORCE = 5,
    SOV_REASON_CLINICAL = 6,
}

unsafe fn cstr_to_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    CStr::from_ptr(ptr).to_str().ok()
}

#[no_mangle]
pub unsafe extern "C" fn sovereignty_core_init(
    rohmodel_path: *const c_char,
    stake_path: *const c_char,
    neurorights_path: *const c_char,
    evolve_path: *const c_char,
    donutloop_path: *const c_char,
) -> *mut sovereignty_core_handle_t {
    let roh = match cstr_to_str(rohmodel_path) { Some(s) => s, None => return std::ptr::null_mut() };
    let st = match cstr_to_str(stake_path) { Some(s) => s, None => return std::ptr::null_mut() };
    let nr = match cstr_to_str(neurorights_path) { Some(s) => s, None => return std::ptr::null_mut() };
    let ev = match cstr_to_str(evolve_path) { Some(s) => s, None => return std::ptr::null_mut() };
    let dl = match cstr_to_str(donutloop_path) { Some(s) => s, None => return std::ptr::null_mut() };

    let core = match SovereigntyCore::new(roh, st, nr, ev, dl) {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };
    let boxed = Box::new(sovereignty_core_handle_t { inner: core });
    Box::into_raw(boxed)
}

#[no_mangle]
pub unsafe extern "C" fn sovereignty_core_free(handle: *mut sovereignty_core_handle_t) {
    if handle.is_null() {
        return;
    }
    drop(Box::from_raw(handle));
}

#[derive(serde::Deserialize)]
struct JsLikeInput {
    pub proposal: EvolutionProposal,
    pub ctx: EvaluationContext,
}

// Map Rust Decision to C enum.
fn map_decision(d: &Decision) -> sov_decision_t {
    match d {
        Decision::ALLOWED => sov_decision_t::SOV_DECISION_ALLOWED,
        Decision::REJECTED => sov_decision_t::SOV_DECISION_REJECTED,
        Decision::DEFERRED => sov_decision_t::SOV_DECISION_DEFERRED,
    }
}

// Map Rust DecisionReason to C enum.
fn map_reason(r: &DecisionReason) -> sov_reason_t {
    match r {
        DecisionReason::Ok => sov_reason_t::SOV_REASON_OK,
        DecisionReason::RoHCeiling => sov_reason_t::SOV_REASON_ROH_CEILING,
        DecisionReason::NeurorightsViolation => sov_reason_t::SOV_REASON_NEURORIGHTS,
        DecisionReason::StakeMultisigMissing => sov_reason_t::SOV_REASON_STAKE_MULTISIG,
        DecisionReason::TokenScopeViolation => sov_reason_t::SOV_REASON_TOKEN_SCOPE,
        DecisionReason::LifeforceEnvelope => sov_reason_t::SOV_REASON_LIFEFORCE,
        DecisionReason::ClinicalEnvelope => sov_reason_t::SOV_REASON_CLINICAL,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sovereignty_core_evaluate_once(
    handle: *mut sovereignty_core_handle_t,
    input_json: *const c_char,
    out_decision: *mut sov_decision_t,
    out_reason: *mut sov_reason_t,
) -> c_int {
    if handle.is_null() || input_json.is_null() || out_decision.is_null() || out_reason.is_null() {
        return -1;
    }

    let core = &mut (*handle).inner;
    let s = match cstr_to_str(input_json) {
        Some(s) => s,
        None => return -2,
    };

    let parsed: JsLikeInput = match serde_json::from_str(s) {
        Ok(v) => v,
        Err(_) => return -3,
    };

    let (decision, reason) = match core.evaluate(&parsed.proposal, &parsed.ctx) {
        Ok((d, r)) => (d, r),
        Err(_) => (Decision::REJECTED, DecisionReason::RoHCeiling),
    };

    *out_decision = map_decision(&decision);
    *out_reason = map_reason(&reason);
    0
}
