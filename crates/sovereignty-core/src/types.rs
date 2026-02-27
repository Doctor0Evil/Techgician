use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// Token scope per your doctrine.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenType {
    SMART,
    EVOLVE,
    TECH,
}

/// Domain plane for environment separation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnvironmentPlane {
    SoftwareOnly,
    SelfHostedNeuroPC,
    WearableBCI,
    Implanted,
}

/// Snapshot of current biostate as seen by sovereigntycore.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BioStateSnapshot {
    pub env_plane: EnvironmentPlane,
    pub roh_axes: RohAxes,
    pub lifeforce_bands_ok: bool,
    pub clinical_envelope_ok: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RohAxes {
    pub roh_total: f32,
    pub roh_bio: f32,
    pub roh_legal: f32,
    pub roh_eco: f32,
}

/// Personal envelopes and neurorights flags at decision time.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersonalEnvelopes {
    pub pain_band: EnvelopeBand,
    pub fear_band: EnvelopeBand,
    pub cognitive_band: EnvelopeBand,
    pub dream_state_allowed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnvelopeBand {
    Low,
    Medium,
    High,
}

/// Proposal scope used for guard routing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProposalScope {
    DayToDayTuning,
    ArchChange,
    LifeforceAlteration,
    InfraChange,
    PolicyUpdate,
}

/// Evolution proposal as presented to sovereigntycore.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolutionProposal {
    pub proposal_id: String,
    pub host_did: String,
    pub submitted_at: DateTime<Utc>,
    pub scope: ProposalScope,
    pub token_type: TokenType,
    pub roh_before: f32,
    pub roh_after_est: f32,
    pub knowledge_delta_est: f32,
    pub cybostate_delta_est: f32,
    pub env_plane: EnvironmentPlane,
    pub shard_refs: ShardRefs,
    pub payload_ref: String, // path or hash of payload
    pub multisig_signers: Vec<SignerRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShardRefs {
    pub rohmodel_ref: String,     // e.g. "rohmodel.aln#v5"
    pub neurorights_ref: String,  // e.g. "neurorights.bostrom.json#v7"
    pub stake_ref: String,        // e.g. "stake.aln#host-bostrom"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignerRef {
    pub signer_did: String,
    pub signature_hex: String,
}

/// Sovereignty decision outcomes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Decision {
    Allowed,
    Rejected,
    Deferred,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DecisionReason {
    RoHCeiling,
    RoHMonotone,
    UniversalAdultFloor,
    EcoBudget,
    NeurorightsViolation,
    DreamStateProhibited,
    DecisionUseForbidden,
    MissingMultisig,
    TokenScopeViolation,
    DomainSeparationViolation,
    DonutloopWriteError,
    PolicyOk,
}

/// Context handed into core evaluate() call.
#[derive(Clone, Debug)]
pub struct EvaluationContext {
    pub biostate: BioStateSnapshot,
    pub envelopes: PersonalEnvelopes,
}
