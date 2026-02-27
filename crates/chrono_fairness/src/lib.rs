//! chrono_fairness
//!
//! Governance-only policies that prevent oppression via:
//! - time-zone games,
//! - chronosphere throttling (clock skew, region delays),
//! - greed-based temporal priority.
//!
//! This crate deliberately contains ZERO Tsafe elements.
//! It only inspects governance metadata and rejects unfair time policies.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Who is asking for temporal control.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeControlRequester {
    pub org_id: String,          // e.g. utility, cloud, state actor
    pub region: String,          // e.g. "US-AZ-PHX"
    pub role: String,            // e.g. "GRID_OP", "CLOUD_OP"
    pub equity_score: f32,       // historically how fair they behaved 0..1
}

/// What they want to do to clocks / scheduling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronoPolicyProposal {
    pub policy_id: String,

    /// Claimed effective time window in UTC.
    pub start_utc: DateTime<Utc>,
    pub end_utc: DateTime<Utc>,

    /// Max allowed skew relative to UTC in seconds (for their systems).
    pub max_clock_skew_secs: i64,

    /// Per-region latency multipliers they want to impose.
    /// Example: { "US-AZ-PHX": 1.0, "GLOBAL-SOUTH": 1.5 } is oppressive.
    pub region_latency_multiplier: Vec<(String, f32)>,

    /// Whether they try to shift billing / quotas by time-zone tricks.
    pub timezone_price_discrimination: bool,

    /// Whether they request “priority slots” for their own workloads.
    pub self_priority_factor: f32,

    /// Narrative: justification text (auditable).
    pub narrative: String,
}

/// Decision on chrono policy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChronoDecision {
    Allowed,
    Rejected,
    NeedsHumanReview,
}

/// Why the decision was taken.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChronoReason {
    Ok,
    TimezoneDiscrimination,
    RegionLatencyAbuse,
    SelfPriorityAbuse,
    SkewTooHigh,
    WindowTooLong,
    LowEquityScore,
}

/// Simple configuration of fairness limits.
/// All limits are global, not Tsafe plant envelopes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronoFairnessConfig {
    /// Max clock skew allowed for any system (seconds).
    pub max_skew_secs_global: i64,

    /// Max allowed latency multiplier between any two regions.
    pub max_latency_multiplier: f32,

    /// Max self-priority factor (1.0 = equal; >1.0 is privileged).
    pub max_self_priority_factor: f32,

    /// Max policy window duration in hours.
    pub max_window_hours: f32,

    /// Minimum equity score required for any special temporal power.
    pub min_equity_score: f32,
}

impl Default for ChronoFairnessConfig {
    fn default() -> Self {
        Self {
            max_skew_secs_global: 2,      // clocks must be near-UTC
            max_latency_multiplier: 1.05, // <5% differential allowed
            max_self_priority_factor: 1.0,
            max_window_hours: 4.0,
            min_equity_score: 0.90,
        }
    }
}

/// Core function: check if a chronosphere policy is oppressive.
///
/// - NO Tsafe state, NO control over plants.
/// - Only governance judgment on temporal fairness.
pub fn evaluate_chrono_policy(
    cfg: &ChronoFairnessConfig,
    requester: &TimeControlRequester,
    proposal: &ChronoPolicyProposal,
) -> (ChronoDecision, ChronoReason) {
    // 1. Reject any timezone-based price discrimination outright.
    if proposal.timezone_price_discrimination {
        return (ChronoDecision::Rejected, ChronoReason::TimezoneDiscrimination);
    }

    // 2. Clock skew must be extremely small; no hidden time dilation.
    if proposal.max_clock_skew_secs.abs() > cfg.max_skew_secs_global {
        return (ChronoDecision::Rejected, ChronoReason::SkewTooHigh);
    }

    // 3. Region latency multipliers must stay near 1.0
    for (_region, mult) in &proposal.region_latency_multiplier {
        if *mult > cfg.max_latency_multiplier || *mult < (2.0 - cfg.max_latency_multiplier) {
            // For simplicity: also forbid making some regions artificially *faster*
            return (ChronoDecision::Rejected, ChronoReason::RegionLatencyAbuse);
        }
    }

    // 4. Self-priority must not exceed configured factor.
    if proposal.self_priority_factor > cfg.max_self_priority_factor {
        return (ChronoDecision::Rejected, ChronoReason::SelfPriorityAbuse);
    }

    // 5. No long-lived temporal grabs.
    let dt = proposal.end_utc - proposal.start_utc;
    let hours = dt.num_seconds() as f32 / 3600.0;
    if hours > cfg.max_window_hours {
        return (ChronoDecision::Rejected, ChronoReason::WindowTooLong);
    }

    // 6. Only high-equity actors can even request minor deviations.
    if requester.equity_score < cfg.min_equity_score {
        return (ChronoDecision::Rejected, ChronoReason::LowEquityScore);
    }

    // If all checks pass, the policy is temporally fair enough.
    (ChronoDecision::Allowed, ChronoReason::Ok)
}

/// Convenience helper for governance UIs.
pub fn needs_human_review(
    cfg: &ChronoFairnessConfig,
    requester: &TimeControlRequester,
    proposal: &ChronoPolicyProposal,
) -> bool {
    let (decision, reason) = evaluate_chrono_policy(cfg, requester, proposal);
    match decision {
        ChronoDecision::Allowed => {
            // If allowed but the narrative is empty, require a human to sign off.
            proposal.narrative.trim().is_empty()
        }
        ChronoDecision::Rejected => {
            // Hard reject, but governance can still inspect manually.
            // We mark "NeedsHumanReview" for some soft reasons.
            matches!(
                reason,
                ChronoReason::LowEquityScore | ChronoReason::WindowTooLong
            )
        }
        ChronoDecision::NeedsHumanReview => true,
    }
}
