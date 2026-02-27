mod types;
mod artifacts;

pub use artifacts::{ArtifactLoader, ArtifactError, EvaluationContext};
pub use types::*;

use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(thiserror::Error, Debug)]
pub enum SovereigntyError {
    #[error("artifact error: {0}")]
    Artifact(#[from] ArtifactError),
    #[error("roh violation")]
    RoHViolation,
    #[error("neurorights violation")]
    NeurorightsViolation,
    #[error("stake multisig failure")]
    StakeMultisig,
    #[error("token scope violation")]
    TokenScope,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct SovereigntyCore {
    rohmodel: RoHModel,
    stake: StakePolicy,
    neurorights: NeurorightsPolicy,
    evolve_path: String,
    donutloop_path: String,
    seq_counter: u64,
}

impl SovereigntyCore {
    pub fn new(
        rohmodel_path: &str,
        stake_path: &str,
        neurorights_path: &str,
        evolve_path: &str,
        donutloop_path: &str,
    ) -> Result<Self, SovereigntyError> {
        let rohmodel = ArtifactLoader::load_rohmodel(rohmodel_path)?;
        let stake = ArtifactLoader::load_stake(stake_path)?;
        let neurorights = ArtifactLoader::load_neurorights(neurorights_path)?;
        Ok(Self {
            rohmodel,
            stake,
            neurorights,
            evolve_path: evolve_path.to_string(),
            donutloop_path: donutloop_path.to_string(),
            seq_counter: 0,
        })
    }

    pub fn evaluate(
        &mut self,
        proposal: &EvolutionProposal,
        ctx: &EvaluationContext,
    ) -> Result<(Decision, DecisionReason), SovereigntyError> {
        // 1. RoH ceiling + monotone safety
        if proposal.roh_after_est > self.rohmodel.roh_ceiling
            || proposal.roh_after_est > proposal.roh_before_est
        {
            self.log_decision(proposal, ctx, Decision::REJECTED, DecisionReason::RoHCeiling)?;
            return Err(SovereigntyError::RoHViolation);
        }

        // 2. Neurorights: dream state + decision-use bans (simplified placeholder)
        if self.neurorights.forbid_dream_state
            && matches!(ctx.biostate.env_plane, EnvironmentPlane::Implanted)
            && !ctx.biostate.clinical_envelope_ok
        {
            self.log_decision(
                proposal,
                ctx,
                Decision::REJECTED,
                DecisionReason::NeurorightsViolation,
            )?;
            return Err(SovereigntyError::NeurorightsViolation);
        }

        // 3. Stakeholder multisig
        if proposal.requires_multisig && !self.check_multisig(proposal) {
            self.log_decision(
                proposal,
                ctx,
                Decision::REJECTED,
                DecisionReason::StakeMultisigMissing,
            )?;
            return Err(SovereigntyError::StakeMultisig);
        }

        // 4. Token kind / scope bounds (simplified)
        if !self.check_token_scope(proposal) {
            self.log_decision(
                proposal,
                ctx,
                Decision::REJECTED,
                DecisionReason::TokenScopeViolation,
            )?;
            return Err(SovereigntyError::TokenScope);
        }

        // 5. Lifeforce / clinical envelopes
        if !ctx.biostate.lifeforce_bands_ok {
            self.log_decision(
                proposal,
                ctx,
                Decision::REJECTED,
                DecisionReason::LifeforceEnvelope,
            )?;
            return Ok((Decision::REJECTED, DecisionReason::LifeforceEnvelope));
        }
        if matches!(ctx.biostate.env_plane, EnvironmentPlane::Implanted)
            && !ctx.biostate.clinical_envelope_ok
        {
            self.log_decision(
                proposal,
                ctx,
                Decision::REJECTED,
                DecisionReason::ClinicalEnvelope,
            )?;
            return Ok((Decision::REJECTED, DecisionReason::ClinicalEnvelope));
        }

        // All guards passed: mark ALLOWED and append to donutloop
        self.log_decision(proposal, ctx, Decision::ALLOWED, DecisionReason::Ok)?;
        Ok((Decision::ALLOWED, DecisionReason::Ok))
    }

    fn check_multisig(&self, proposal: &EvolutionProposal) -> bool {
        if !proposal.requires_multisig {
            return true;
        }
        let mut needed: Vec<&StakeRole> = self
            .stake
            .roles
            .iter()
            .filter(|r| r.can_sign_scopes.contains(&"archchange".to_string()))
            .collect();
        if needed.is_empty() {
            return false;
        }
        needed.iter().all(|role| proposal.signer_dids.contains(&role.did))
    }

    fn check_token_scope(&self, proposal: &EvolutionProposal) -> bool {
        match proposal.tokentype {
            TokenType::SMART => !matches!(proposal.scope, ProposalScope::ArchChange),
            TokenType::EVOLVE => true,
            TokenType::INSTINCT => true,
            TokenType::TECH => !matches!(proposal.scope, ProposalScope::Lifeforce),
        }
    }

    fn next_seq(&mut self) -> u64 {
        self.seq_counter += 1;
        self.seq_counter
    }

    fn eval_envelope_event(&self, ctx: &EvaluationContext) -> PersonalEnvelopeEvent {
        let approaching = matches!(ctx.envelopes.pain_band, EnvelopeBand::HIGH)
            || matches!(ctx.envelopes.fear_band, EnvelopeBand::HIGH);
        let instinct_vetoed = false; // future: INSTINCT kernel
        let explicit_spend = approaching;
        PersonalEnvelopeEvent {
            pain_band: ctx.envelopes.pain_band.clone(),
            fear_band: ctx.envelopes.fear_band.clone(),
            cognitive_band: ctx.envelopes.cognitive_band.clone(),
            approaching_threshold: approaching,
            instinct_vetoed,
            explicit_spend,
        }
    }

    fn log_decision(
        &mut self,
        proposal: &EvolutionProposal,
        ctx: &EvaluationContext,
        decision: Decision,
        reason: DecisionReason,
    ) -> Result<(), SovereigntyError> {
        let seq = self.next_seq();
        let event = self.eval_envelope_event(ctx);

        // hexstamp placeholders (to be replaced by your hash kernel)
        let prev_hex = "0xD0NUTPREVPLACEHOLDER";
        let current_hex = format!("0xD0NUT{:016X}", seq);

        let entry = DonutloopEntry {
            proposal_id: proposal.proposal_id.clone(),
            seq,
            host_did: proposal.host_did.clone(),
            roh_before: proposal.roh_before_est,
            roh_after: proposal.roh_after_est,
            knowledge_factor_before: 0.0,
            knowledge_factor_after: proposal.k_gain_est,
            decision,
            decision_reason: reason,
            scope: proposal.scope.clone(),
            tokentype: proposal.tokentype.clone(),
            personal_envelope_event: event,
            hexstamp_current: current_hex,
            hexstamp_prev: prev_hex.to_string(),
            rohmodel_ref: format!("rohmodel.aln#{}", self.rohmodel.version),
            neurorights_ref: format!("neurorights.json#{}", self.neurorights.version),
            stake_ref: format!("stake.aln#{}", self.stake.version),
        };

        // append-only write
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.donutloop_path)?;
        let line = serde_json::to_string(&entry).unwrap();
        writeln!(f, "{}", line)?;
        Ok(())
    }
}
