use crate::artifacts::{ArtifactLoader, NeurorightsPolicy, RohModel, StakePolicy};
use crate::donutloop::DonutloopSink;
use crate::types::{
    Decision, DecisionReason, EvaluationContext, EvolutionProposal, TokenType,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GuardError {
    #[error("Artifact error: {0}")]
    Artifact(String),
    #[error("Donutloop error: {0}")]
    Donutloop(String),
}

pub struct SovereigntyCore {
    rohmodel: RohModel,
    stake: StakePolicy,
    neurorights: NeurorightsPolicy,
    roh_ceiling: f32,
    donutloop: DonutloopSink,
}

impl SovereigntyCore {
    pub fn new(
        rohmodel_path: &str,
        stake_path: &str,
        neurorights_path: &str,
        evolve_path: &str,
        donutloop_path: &str,
    ) -> Result<Self, GuardError> {
        let rohmodel = ArtifactLoader::load_rohmodel(rohmodel_path)
            .map_err(|e| GuardError::Artifact(e.to_string()))?;
        let stake = ArtifactLoader::load_stake(stake_path)
            .map_err(|e| GuardError::Artifact(e.to_string()))?;
        let neurorights = ArtifactLoader::load_neurorights(neurorights_path)
            .map_err(|e| GuardError::Artifact(e.to_string()))?;

        Ok(Self {
            roh_ceiling: rohmodel.roh_ceiling,
            rohmodel,
            stake,
            neurorights,
            donutloop: DonutloopSink::new(evolve_path.to_string(), donutloop_path.to_string()),
        })
    }

    pub fn evaluate(
        &self,
        proposal: &EvolutionProposal,
        ctx: &EvaluationContext,
    ) -> Result<(Decision, DecisionReason), GuardError> {
        // 1. RoH ceiling + monotone rule
        if proposal.roh_after_est > self.roh_ceiling {
            self.log_and_return(proposal, &ctx, Decision::Rejected, DecisionReason::RoHCeiling, proposal.roh_before)?;
            return Ok((Decision::Rejected, DecisionReason::RoHCeiling));
        }
        if proposal.roh_after_est > proposal.roh_before {
            self.log_and_return(proposal, &ctx, Decision::Rejected, DecisionReason::RoHMonotone, proposal.roh_before)?;
            return Ok((Decision::Rejected, DecisionReason::RoHMonotone));
        }

        // 2. Neurorights constraints
        if self.neurorights.forbid_dream_state && !ctx.envelopes.dream_state_allowed {
            self.log_and_return(proposal, &ctx, Decision::Rejected, DecisionReason::DreamStateProhibited, proposal.roh_before)?;
            return Ok((Decision::Rejected, DecisionReason::DreamStateProhibited));
        }

        // Forbid decision use in named domains for EVOLVE/SMART affecting BioState:
        if self.affects_decision_use(proposal) {
            self.log_and_return(proposal, &ctx, Decision::Rejected, DecisionReason::DecisionUseForbidden, proposal.roh_before)?;
            return Ok((Decision::Rejected, DecisionReason::DecisionUseForbidden));
        }

        // 3. Stakeholder multisig for EVOLVE high-impact scopes
        if matches!(proposal.token_type, TokenType::EVOLVE)
            && self.requires_multisig(proposal)
            && !self.verify_multisig(proposal)
        {
            self.log_and_return(proposal, &ctx, Decision::Deferred, DecisionReason::MissingMultisig, proposal.roh_before)?;
            return Ok((Decision::Deferred, DecisionReason::MissingMultisig));
        }

        // 4. Token kind and domain separation (no TECH changes BioState, etc.)
        if self.violates_domain_separation(proposal) {
            self.log_and_return(proposal, &ctx, Decision::Rejected, DecisionReason::DomainSeparationViolation, proposal.roh_before)?;
            return Ok((Decision::Rejected, DecisionReason::DomainSeparationViolation));
        }

        // 5. All guards passed -> Allowed
        self.log_and_return(
            proposal,
            &ctx,
            Decision::Allowed,
            DecisionReason::PolicyOk,
            proposal.roh_after_est,
        )?;
        Ok((Decision::Allowed, DecisionReason::PolicyOk))
    }

    fn affects_decision_use(&self, _proposal: &EvolutionProposal) -> bool {
        // Hook point: inspect payload_ref, scope, etc., and compare against neurorights.forbid_decision_use_domains.
        false
    }

    fn requires_multisig(&self, proposal: &EvolutionProposal) -> bool {
        matches!(proposal.scope,
            crate::types::ProposalScope::ArchChange |
            crate::types::ProposalScope::LifeforceAlteration |
            crate::types::ProposalScope::PolicyUpdate)
    }

    fn verify_multisig(&self, proposal: &EvolutionProposal) -> bool {
        let required = &self.stake.evolve_signers;
        if required.is_empty() {
            return true;
        }
        let mut seen: Vec<String> = Vec::new();
        for sig in &proposal.multisig_signers {
            if required.contains(&sig.signer_did) {
                // cryptographic verification goes here (PQC multisig, etc.)
                seen.push(sig.signer_did.clone());
            }
        }
        seen.sort();
        seen.dedup();
        seen.len() == required.len()
    }

    fn violates_domain_separation(&self, proposal: &EvolutionProposal) -> bool {
        match proposal.token_type {
            TokenType::TECH => {
                matches!(proposal.scope, crate::types::ProposalScope::LifeforceAlteration)
            }
            TokenType::SMART | TokenType::EVOLVE => false,
        }
    }

    fn log_and_return(
        &self,
        proposal: &EvolutionProposal,
        ctx: &EvaluationContext,
        decision: Decision,
        reason: DecisionReason,
        roh_after: f32,
    ) -> Result<(), GuardError> {
        self.donutloop
            .log_decision(proposal, decision, reason, &ctx.envelopes, roh_after)
            .map_err(GuardError::Donutloop)
    }
}
