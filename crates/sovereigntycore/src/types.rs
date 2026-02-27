use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoHAxis {
    pub name: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoHModel {
    pub version: String,
    pub roh_ceiling: f32,          // must be <= 0.3 for TsafeRoH
    pub axes: Vec<RoHAxis>,        // e.g., bio, psycho, legal, eco
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeRole {
    pub name: String,              // "Host", "OrganicCPU", "ResearchAgent"
    pub did: String,               // DID / Bostrom address
    pub can_sign_scopes: Vec<String>, // e.g., ["lifeforce", "archchange"]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakePolicy {
    pub version: String,
    pub host_did: String,
    pub roles: Vec<StakeRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurorightsPolicy {
    pub version: String,
    pub forbid_dream_state: bool,
    pub forbid_decision_use_domains: Vec<String>, // e.g., ["employment", "credit"]
    pub allowed_export_domains: Vec<String>,      // whitelisted export channels
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    SMART,
    EVOLVE,
    INSTINCT,
    TECH,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalScope {
    #[serde(rename = "daytodaytuning")]
    DayToDayTuning,
    #[serde(rename = "archchange")]
    ArchChange,
    #[serde(rename = "lifeforce")]
    Lifeforce,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionProposal {
    pub proposal_id: String,
    pub host_did: String,
    pub submitted_at: DateTime<Utc>,
    pub scope: ProposalScope,
    pub tokentype: TokenType,

    pub roh_before_est: f32,
    pub roh_after_est: f32,

    pub k_gain_est: f32,
    pub c_delta_est: f32,

    pub requires_multisig: bool,
    pub signer_dids: Vec<String>,

    pub payload_ref: String, // path or hash for detailed change
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentPlane {
    #[serde(rename = "SELFHOSTED")]
    SelfHosted,
    #[serde(rename = "IMPLANTED")]
    Implanted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RohAxesSnapshot {
    pub roh_total: f32,
    pub roh_bio: f32,
    pub roh_psych: f32,
    pub roh_legal: f32,
    pub roh_eco: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioStateSnapshot {
    pub env_plane: EnvironmentPlane,
    pub roh_axes: RohAxesSnapshot,
    pub lifeforce_bands_ok: bool,
    pub clinical_envelope_ok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvelopeBand {
    LOW,
    MEDIUM,
    HIGH,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalEnvelopes {
    pub pain_band: EnvelopeBand,
    pub fear_band: EnvelopeBand,
    pub cognitive_band: EnvelopeBand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalEnvelopeEvent {
    pub pain_band: EnvelopeBand,
    pub fear_band: EnvelopeBand,
    pub cognitive_band: EnvelopeBand,
    pub approaching_threshold: bool,
    pub instinct_vetoed: bool,
    pub explicit_spend: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Decision {
    ALLOWED,
    REJECTED,
    DEFERRED,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionReason {
    RoHCeiling,
    NeurorightsViolation,
    StakeMultisigMissing,
    TokenScopeViolation,
    LifeforceEnvelope,
    ClinicalEnvelope,
    Ok,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonutloopEntry {
    pub proposal_id: String,
    pub seq: u64,
    pub host_did: String,
    pub roh_before: f32,
    pub roh_after: f32,
    pub knowledge_factor_before: f32,
    pub knowledge_factor_after: f32,
    pub decision: Decision,
    pub decision_reason: DecisionReason,
    pub scope: ProposalScope,
    pub tokentype: TokenType,
    pub personal_envelope_event: PersonalEnvelopeEvent,
    pub hexstamp_current: String,
    pub hexstamp_prev: String,
    pub rohmodel_ref: String,
    pub neurorights_ref: String,
    pub stake_ref: String,
}
