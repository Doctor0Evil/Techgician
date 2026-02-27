const { evaluate_once } = require('./index.node'); // built napi module
const fs = require('fs');

const payload = {
  rohmodel_path: "./config/rohmodel.json",
  stake_path: "./config/stake.json",
  neurorights_path: "./config/neurorights.json",
  evolve_path: "./state/evolve.jsonl",
  donutloop_path: "./state/donutloop.aln",
  proposal: {
    proposal_id: "demo-1",
    host_did: "bostrom18sd2u...",
    submitted_at: new Date().toISOString(),
    scope: "DayToDayTuning",
    token_type: "SMART",
    roh_before: 0.12,
    roh_after_est: 0.10,
    knowledge_delta_est: 0.01,
    cybostate_delta_est: 0.0,
    env_plane: "SelfHostedNeuroPC",
    shard_refs: {
      rohmodel_ref: "rohmodel.aln#v1",
      neurorights_ref: "neurorights.json#v1",
      stake_ref: "stake.aln#v1"
    },
    payload_ref: "payloads/demo-1.json",
    multisig_signers: []
  },
  ctx: {
    biostate: {
      env_plane: "SelfHostedNeuroPC",
      roh_axes: {
        roh_total: 0.12,
        roh_bio: 0.08,
        roh_legal: 0.02,
        roh_eco: 0.02
      },
      lifeforce_bands_ok: true,
      clinical_envelope_ok: true
    },
    envelopes: {
      pain_band: "Low",
      fear_band: "Low",
      cognitive_band: "Medium",
      dream_state_allowed: false
    }
  }
};

const resultJson = evaluate_once(JSON.stringify(payload));
const result = JSON.parse(resultJson);

console.log("decision:", result.decision, "reason:", result.reason);
