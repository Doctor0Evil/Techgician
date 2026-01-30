//! Sovereignty-safe AugmentationRight profile enforcement.
//!
//! This module parses a minimal ALN view and mechanically verifies that
//! the host's augmentation profile satisfies the six AugmentationRight
//! invariants: host-bound tokens, consent-anchored ownership, augmented-
//! citizen-only mutation surface, strict inner/outer separation, role-
//! based governance (no capital), and neurorights / anti-seizure rules.

#![forbid(unsafe_code)]

use std::collections::HashSet;

/// Minimal view of an ALN shard after parsing.
#[derive(Clone, Debug)]
pub struct AugmentationRightProfile {
    pub host_id: String,
    pub profile_id: String,

    // 1. Tokens
    pub tokens_host_bound: bool,
    pub defi_bridge: bool,
    pub stake_weighted: bool,
    pub marketplace: bool,

    // 2. Consent
    pub consent_required_for_blueprint: bool,
    pub consent_required_for_state_mutation: bool,
    pub consent_proof_type: String,
    pub consent_revocation_model: String,
    pub consent_no_commercial_reuse: bool,
    pub consent_no_military_reuse: bool,

    // 3. Access control
    pub allowed_roles: HashSet<String>,
    pub denied_roles: HashSet<String>,
    pub ai_platforms_may_propose: bool,
    pub ai_platforms_may_execute: bool,

    // 4. Inner vs outer
    pub inner_ledger_isolation: bool,
    pub outer_leak_balances: bool,
    pub outer_leak_raw_telemetry: bool,

    // 5. Governance
    pub governance_control_model: String,
    pub governance_capital_influence: String,
    pub governance_requires_host_consent: bool,
    pub governance_requires_ethical_review: bool,
    pub governance_may_touch_souls: bool,
    pub governance_may_touch_consciousness: bool,

    // 6. Neurorights
    pub neurorights_soul_immutable: bool,
    pub neurorights_consciousness_immutable: bool,
    pub neurorights_no_expropriation: bool,
    pub neurorights_no_downgrade_by_third: bool,
}

/// Result of sovereignty-safety verification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SovereigntyStatus {
    SovereigntySafe,
    ViolatesInvariant(Vec<String>),
}

impl AugmentationRightProfile {
    /// Mechanical check that this profile satisfies all AugmentationRight rules.
    pub fn verify_sovereignty_safe(&self) -> SovereigntyStatus {
        let mut errors = Vec::new();

        // 1. Host-bound, non-financial tokens
        if !self.tokens_host_bound {
            errors.push("Tokens must be host-bound (no cross-host circulation).".into());
        }
        if self.defi_bridge {
            errors.push("DeFi / bridge interfaces must be disabled for biophysical tokens.".into());
        }
        if self.stake_weighted {
            errors.push("Stake-weighted control over biophysical state is forbidden.".into());
        }
        if self.marketplace {
            errors.push("Marketplace / sale / lease of augmentation capacity is forbidden.".into());
        }

        // 2. Consent + provenance as ownership
        if !self.consent_required_for_blueprint {
            errors.push("Consent must be required for every augmentation blueprint.".into());
        }
        if !self.consent_required_for_state_mutation {
            errors.push("Consent must be required for any biophysical state mutation.".into());
        }
        if self.consent_proof_type != "ALN-DID-ZK" {
            errors.push("Consent proof type must be ALN-DID-ZK (or stricter).".into());
        }
        if self.consent_revocation_model != "host-revocable" {
            errors.push("Consent must be host-revocable; vendor-only revocation is invalid.".into());
        }

        // 3. Augmented-citizen–only mutation surface
        if !self.allowed_roles.contains("augmented-citizen") {
            errors.push("Role 'augmented-citizen' must be allowed to control own augmentations.".into());
        }
        for bad in &["vendor-generic", "sandbox", "pure-machine"] {
            if !self.denied_roles.contains(*bad) {
                errors.push(format!(
                    "Role '{}' must be explicitly denied from mutating inner-ledger state.",
                    bad
                ));
            }
        }
        if self.ai_platforms_may_execute {
            errors.push("AI platforms may not execute state changes; they may only propose.".into());
        }

        // 4. Inner vs outer separation
        if !self.inner_ledger_isolation {
            errors.push("Inner-ledger isolation must be true (no direct outer-chain control).".into());
        }
        if self.outer_leak_balances {
            errors.push("Outer attestations may not leak token balances.".into());
        }
        if self.outer_leak_raw_telemetry {
            errors.push("Outer attestations may not leak raw EEG/biophysical telemetry.".into());
        }

        // 5. Rights-based governance, no capital control
        if self.governance_control_model != "role-and-rights" {
            errors.push("Governance control model must be 'role-and-rights', not capital-weighted.".into());
        }
        if self.governance_capital_influence != "none" {
            errors.push("Capital influence over governance must be 'none'.".into());
        }
        if !self.governance_requires_host_consent {
            errors.push("Governance changes must require host consent.".into());
        }
        if !self.governance_requires_ethical_review {
            errors.push("Governance changes must require ethical review.".into());
        }
        if self.governance_may_touch_souls {
            errors.push("Governance may not touch souls; only safety proxies are allowed.".into());
        }
        if self.governance_may_touch_consciousness {
            errors.push("Governance may not directly modify consciousness states.".into());
        }

        // 6. Neurorights + anti-seizure invariants
        if !self.neurorights_soul_immutable {
            errors.push("Soul immutability must be true.".into());
        }
        if !self.neurorights_consciousness_immutable {
            errors.push("Consciousness immutability must be true.".into());
        }
        if !self.neurorights_no_expropriation {
            errors.push("No-expropriation must be enforced (no burning/freezing to benefit others).".into());
        }
        if !self.neurorights_no_downgrade_by_third {
            errors.push("No-downgrade-by-third must be enforced (vendors/regulators cannot disable if safe).".into());
        }

        if errors.is_empty() {
            SovereigntyStatus::SovereigntySafe
        } else {
            SovereigntyStatus::ViolatesInvariant(errors)
        }
    }
}
