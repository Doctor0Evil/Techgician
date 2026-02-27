use chrono::{DateTime, Utc};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use sovereignty_core::{
    EvaluationContext, SovereigntyCore,
    BioStateSnapshot, EnvironmentPlane, PersonalEnvelopes, EnvelopeBand,
    EvolutionProposal, ProposalScope, TokenType,
};

#[derive(Serialize, Deserialize)]
struct JsEvaluateInput {
    rohmodel_path: String,
    stake_path: String,
    neurorights_path: String,
    evolve_path: String,
    donutloop_path: String,
    proposal: EvolutionProposal,
    ctx: JsEvaluationContext,
}

#[derive(Serialize, Deserialize)]
struct JsEvaluationContext {
    biostate: BioStateSnapshot,
    envelopes: PersonalEnvelopes,
}

#[derive(Serialize, Deserialize)]
struct JsEvaluateOutput {
    decision: sovereignty_core::Decision,
    reason: sovereignty_core::DecisionReason,
}

#[napi]
pub fn evaluate_once(input: String) -> Result<String> {
    let parsed: JsEvaluateInput =
        serde_json::from_str(&input).map_err(|e| Error::from_reason(e.to_string()))?;

    let core = SovereigntyCore::new(
        &parsed.rohmodel_path,
        &parsed.stake_path,
        &parsed.neurorights_path,
        &parsed.evolve_path,
        &parsed.donutloop_path,
    )
    .map_err(|e| Error::from_reason(e.to_string()))?;

    let ctx = EvaluationContext {
        biostate: parsed.ctx.biostate,
        envelopes: parsed.ctx.envelopes,
    };

    let (decision, reason) = core
        .evaluate(&parsed.proposal, &ctx)
        .map_err(|e| Error::from_reason(e.to_string()))?;

    let out = JsEvaluateOutput { decision, reason };
    let s = serde_json::to_string(&out).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(s)
}

// Optional helpers for constructing types from JS if you don't want to
// build full EvolutionProposal JSON by hand; left minimal for now.

#[napi]
pub fn parse_iso8601(ts: String) -> Result<i64> {
    let dt: DateTime<Utc> = ts
        .parse()
        .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(dt.timestamp_millis())
}
