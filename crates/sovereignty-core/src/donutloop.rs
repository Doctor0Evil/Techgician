use crate::types::{Decision, DecisionReason, EvolutionProposal, PersonalEnvelopes};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CyboState {
    pub c_value: f32,
    pub band: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersonalEnvelopeEvent {
    pub pain_band: String,
    pub fear_band: String,
    pub cognitive_band: String,
    pub approaching_threshold: bool,
    pub instinct_vetoed: bool,
    pub explicit_spend: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DonutloopEntry {
    pub proposal_id: String,
    pub seq: u64,
    pub host_did: String,
    pub roh_before: f64,
    pub roh_after: f64,
    pub rohmodel_ref: String,
    pub knowledge_before: f64,
    pub knowledge_after: f64,
    pub cybostate_before: CyboState,
    pub cybostate_after: CyboState,
    pub decision: Decision,
    pub decision_reason: DecisionReason,
    pub scope: String,
    pub token_type: String,
    pub neurorights_ref: String,
    pub stake_ref: String,
    pub personal_envelope_event: PersonalEnvelopeEvent,
    pub submitted_at: DateTime<Utc>,
    pub decided_at: DateTime<Utc>,
    pub hexstamp_current: String,
    pub hexstamp_prev: Option<String>,
}

pub struct DonutloopSink {
    evolve_path: String,    // .evolve.jsonl
    donutloop_path: String, // .donutloop.aln (also JSONL here)
}

impl DonutloopSink {
    pub fn new(evolve_path: String, donutloop_path: String) -> Self {
        Self { evolve_path, donutloop_path }
    }

    fn next_seq<P: AsRef<Path>>(p: P) -> u64 {
        let file = match File::open(p) {
            Ok(f) => f,
            Err(_) => return 1,
        };
        let reader = BufReader::new(file);
        let mut last_seq = 0u64;
        for line in reader.lines().flatten() {
            if line.trim().is_empty() { continue; }
            if let Ok(entry) = serde_json::from_str::<DonutloopEntry>(&line) {
                last_seq = entry.seq;
            }
        }
        last_seq + 1
    }

    fn last_hex<P: AsRef<Path>>(p: P) -> Option<String> {
        let file = File::open(p).ok()?;
        let reader = BufReader::new(file);
        let mut last: Option<DonutloopEntry> = None;
        for line in reader.lines().flatten() {
            if line.trim().is_empty() { continue; }
            if let Ok(entry) = serde_json::from_str::<DonutloopEntry>(&line) {
                last = Some(entry);
            }
        }
        last.map(|e| e.hexstamp_current)
    }

    fn write_line<P: AsRef<Path>>(p: P, s: &str) -> std::io::Result<()> {
        let mut f = OpenOptions::new().create(true).append(true).open(p)?;
        f.write_all(s.as_bytes())?;
        f.write_all(b"\n")?;
        Ok(())
    }

    pub fn log_decision(
        &self,
        proposal: &EvolutionProposal,
        decision: Decision,
        reason: DecisionReason,
        envelopes: &PersonalEnvelopes,
        roh_after: f32,
    ) -> Result<(), String> {
        let seq = Self::next_seq(&self.donutloop_path);
        let prev_hex = Self::last_hex(&self.donutloop_path);

        let personal_event = PersonalEnvelopeEvent {
            pain_band: format!("{:?}", envelopes.pain_band),
            fear_band: format!("{:?}", envelopes.fear_band),
            cognitive_band: format!("{:?}", envelopes.cognitive_band),
            approaching_threshold: matches!(envelopes.pain_band, crate::types::EnvelopeBand::High)
                || matches!(envelopes.fear_band, crate::types::EnvelopeBand::High),
            instinct_vetoed: matches!(reason, DecisionReason::NeurorightsViolation),
            explicit_spend: matches!(proposal.token_type, crate::types::TokenType::EVOLVE),
        };

        let cybo_before = CyboState { c_value: 0.0, band: "UNKNOWN".into() };
        let cybo_after = CyboState { c_value: 0.0, band: "UNKNOWN".into() };

        let decided_at = Utc::now();

        let mut hasher = Sha256::new();
        hasher.update(proposal.proposal_id.as_bytes());
        hasher.update(seq.to_be_bytes());
        hasher.update((proposal.roh_before as f64).to_be_bytes());
        hasher.update((roh_after as f64).to_be_bytes());
        if let Some(ref prev) = prev_hex {
            hasher.update(prev.as_bytes());
        }
        let hexstamp_current = hex::encode(hasher.finalize());

        let entry = DonutloopEntry {
            proposal_id: proposal.proposal_id.clone(),
            seq,
            host_did: proposal.host_did.clone(),
            roh_before: proposal.roh_before as f64,
            roh_after: roh_after as f64,
            rohmodel_ref: proposal.shard_refs.rohmodel_ref.clone(),
            knowledge_before: 0.0,
            knowledge_after: proposal.knowledge_delta_est as f64,
            cybostate_before: cybo_before,
            cybostate_after: cybo_after,
            decision: decision.clone(),
            decision_reason: reason.clone(),
            scope: format!("{:?}", proposal.scope),
            token_type: format!("{:?}", proposal.token_type),
            neurorights_ref: proposal.shard_refs.neurorights_ref.clone(),
            stake_ref: proposal.shard_refs.stake_ref.clone(),
            personal_envelope_event: personal_event,
            submitted_at: proposal.submitted_at,
            decided_at,
            hexstamp_current,
            hexstamp_prev: prev_hex,
        };

        let line = serde_json::to_string(&entry).map_err(|e| e.to_string())?;
        Self::write_line(&self.donutloop_path, &line)
            .map_err(|e| e.to_string())?;

        // Mirror to .evolve.jsonl for proposal log:
        Self::write_line(&self.evolve_path, &line)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
