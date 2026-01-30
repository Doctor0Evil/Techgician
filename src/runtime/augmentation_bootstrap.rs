//! Startup and governance wiring for AugmentationRight.
//!
//! This module loads the host-augmentation-right.aln shard, maps it into
//! AugmentationRightProfile, and refuses to run the biophysical runtime
//! if sovereignty invariants are violated.

#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

use crate::governance::augmentation_right::{
    AugmentationRightProfile,
    SovereigntyStatus,
};

/// Minimal ALN parser stub for the augmentation-right shard.
/// In production, replace with your real ALN parser.
///
/// This expects exactly the structure from:
///   qpu/data/shards/host-augmentation-right.aln
pub fn load_augmentation_right_profile<P: AsRef<Path>>(
    path: P,
) -> Result<AugmentationRightProfile, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("Failed to read shard: {e}"))?;

    // Very simple line-based parsing; assumes well-formed shard.
    let mut host_id = String::new();
    let mut profile_id = String::new();

    let mut tokens_host_bound = false;
    let mut defi_bridge = false;
    let mut stake_weighted = false;
    let mut marketplace = false;

    let mut consent_required_for_blueprint = false;
    let mut consent_required_for_state_mutation = false;
    let mut consent_proof_type = String::new();
    let mut consent_revocation_model = String::new();
    let mut consent_no_commercial_reuse = false;
    let mut consent_no_military_reuse = false;

    let mut allowed_roles = std::collections::HashSet::new();
    let mut denied_roles = std::collections::HashSet::new();
    let mut ai_platforms_may_propose = false;
    let mut ai_platforms_may_execute = false;

    let mut inner_ledger_isolation = false;
    let mut outer_leak_balances = false;
    let mut outer_leak_raw_telemetry = false;

    let mut governance_control_model = String::new();
    let mut governance_capital_influence = String::new();
    let mut governance_requires_host_consent = false;
    let mut governance_requires_ethical_review = false;
    let mut governance_may_touch_souls = false;
    let mut governance_may_touch_consciousness = false;

    let mut neurorights_soul_immutable = false;
    let mut neurorights_consciousness_immutable = false;
    let mut neurorights_no_expropriation = false;
    let mut neurorights_no_downgrade_by_third = false;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Very small ad-hoc parser based on prefixes.
        if line.starts_with("host-id") {
            host_id = line.split_whitespace().nth(1).unwrap_or_default().to_string();
        } else if line.starts_with("profile-id") {
            profile_id = line.split_whitespace().nth(1).unwrap_or_default().to_string();
        } else if line.starts_with("host-bound") {
            tokens_host_bound = line.contains("true");
        } else if line.starts_with("defi-bridge") {
            defi_bridge = line.contains("true");
        } else if line.starts_with("stake-weighted") {
            stake_weighted = line.contains("true");
        } else if line.starts_with("marketplace") {
            marketplace = line.contains("true");
        } else if line.starts_with("augmentation-blueprint") {
            consent_required_for_blueprint = line.contains("true");
        } else if line.starts_with("state-mutation") {
            consent_required_for_state_mutation = line.contains("true");
        } else if line.starts_with("proof-type") {
            if let Some(idx) = line.find('"') {
                let tail = &line[idx+1..];
                if let Some(end) = tail.find('"') {
                    consent_proof_type = tail[..end].to_string();
                }
            }
        } else if line.starts_with("revocation-model") {
            if let Some(idx) = line.find('"') {
                let tail = &line[idx+1..];
                if let Some(end) = tail.find('"') {
                    consent_revocation_model = tail[..end].to_string();
                }
            }
        } else if line.starts_with("no-commercial-reuse") {
            consent_no_commercial_reuse = line.contains("true");
        } else if line.starts_with("no-military-reuse") {
            consent_no_military_reuse = line.contains("true");
        } else if line.starts_with("- augmented-citizen") {
            allowed_roles.insert("augmented-citizen".into());
        } else if line.starts_with("- ethical-operator") {
            allowed_roles.insert("ethical-operator".into());
        } else if line.starts_with("- system-daemon") {
            allowed_roles.insert("system-daemon".into());
        } else if line.starts_with("- vendor-generic") {
            denied_roles.insert("vendor-generic".into());
        } else if line.starts_with("- sandbox") {
            denied_roles.insert("sandbox".into());
        } else if line.starts_with("- pure-machine") {
            denied_roles.insert("pure-machine".into());
        } else if line.starts_with("may-propose") {
            ai_platforms_may_propose = line.contains("true");
        } else if line.starts_with("may-execute") {
            ai_platforms_may_execute = line.contains("true");
        } else if line.starts_with("inner-ledger-isolation") {
            inner_ledger_isolation = line.contains("true");
        } else if line.starts_with("leak-balances") {
            outer_leak_balances = line.contains("true");
        } else if line.starts_with("leak-raw-telemetry") {
            outer_leak_raw_telemetry = line.contains("true");
        } else if line.starts_with("control-model") {
            if let Some(idx) = line.find('"') {
                let tail = &line[idx+1..];
                if let Some(end) = tail.find('"') {
                    governance_control_model = tail[..end].to_string(
