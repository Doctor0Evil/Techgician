## 1. ALN‑style invariants (signatures only)

These are the logic contracts you formally prove over the Rust types we already sketched. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/994be4b5-c833-4d4d-ba32-b0c9d9a4ec7e/cycoquatic-instantiators-how-c-c.7kGeoiRMeXnnBTkkK_7A.md)

### 1.1 Corridor presence (“no corridor, no build”)

Applies at CI / compile time for infra and person shards.

```rust
// ALN-style pseudo-signatures (not Rust syntax)

contract corridor_present_infra(m: InfraNodeShard) -> bool
requires
    m.corridors.len() >= MIN_INFRA_CORRIDORS
ensures
    // Every mandatory var_id is present exactly once
    ∀ var ∈ MANDATORY_INFRA_VARS :
        ∃! row ∈ m.corridors : row.var_id == var
```

```rust
contract corridor_present_person(p: PersonShard) -> bool
requires
    true
ensures
    // Personal hard corridors (health, combustion, base eco) must exist;
    // soulsafety/equity are allowed to be soft-logged (optional).
    ∀ var ∈ MANDATORY_PERSON_VARS :
        ∃! r ∈ p.personal_rx : r.var_id == var
```

CI rule:

- If `corridor_present_*` is false for any shard in test fixtures → build fails; deployment images cannot be produced. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/9e1fa3cb-9dcf-4886-af1f-5c08df7709be/if-we-were-to-analyze-how-the-QczV8LYWRSOWAWdpKV30DQ.md)

### 1.2 Safestep for infrastructure (“no unsafe step”)

This guards MAR, cyboquatic, furnace, trayline, siting controllers.

```rust
contract safestep_infra(
    prev: Residual,
    next: Residual
) -> CorridorDecision
requires
    prev.rx.len() == next.rx.len()
ensures
    // Hard per-axis bound: no variable may hit or exceed hard band.
    (∃ j : next.rx[j].value >= 1.0) == result.stop

    // If no hard breach, Lyapunov residual must not increase outside interior.
    (¬result.stop) ∧ (next.vt > prev.vt) ⇒ result.derate

    // If neither hard breach nor residual increase, we accept.
    (¬result.stop) ∧ (next.vt <= prev.vt) ⇒ (¬result.derate)
```

Implementation is the `check_infra_action` we already wrote; ALN tools prove that all controller transitions call this and respect its result. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/389389b2-b1fe-4634-a7d3-178300a91c2a/wbtc-under-an-eibon-should-har-fbKbj59gSaW.OofgLPQ4tg.md)

### 1.3 Upgrade gating for a single person

No collections, no infra shards; only a single DID‑anchored `PersonShard`.

```rust
contract upgrade_allowed(
    p: PersonShard,
    pol: UpgradePolicy
) -> bool
requires
    p.risk_of_harm >= 0.0 ∧ p.risk_of_harm <= 1.0
    ∧ p.knowledge_factor >= 0.0 ∧ p.knowledge_factor <= 1.0
    ∧ p.eco_impact_value >= 0.0 ∧ p.eco_impact_value <= 1.0
    ∧ p.cy_points >= 0.0
ensures
    result ⇒ (
        p.risk_of_harm <= pol.r_max
        ∧ p.knowledge_factor >= pol.k_min
        ∧ p.eco_impact_value >= pol.e_min
        ∧ p.cy_points >= pol.cy_min
    )
```

Additional governance invariants (type‑level):

```rust
// Pseudocode: forbidden signatures (enforced by code review + CI lints)

forbid fn check_upgrade_many(ps: Vec<PersonShard>, ...) -> ...
forbid fn check_upgrade_with_infra(p: PersonShard, infra: InfraNodeShard, ...) -> ...
```

This keeps personal evolution and infra operation cleanly separated in the type system. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/0532b9cc-99a8-4612-a4f6-fe7fdab383c2/with-the-content-below-help-us-Z8rCwZflR669cMMP6GfYSw.md)

***

## 2. QPU.Datashard layouts (CSV‑style)

These layouts follow your qpudatashard patterns: flat, typed fields, with corridor tables as embedded rows, so they work with existing EcoNet / VitalNet tools. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/994be4b5-c833-4d4d-ba32-b0c9d9a4ec7e/cycoquatic-instantiators-how-c-c.7kGeoiRMeXnnBTkkK_7A.md)

### 2.1 `infra.node.metrics.v1`

Logical sections:

1) Header/meta  
2) KER summary  
3) Residual snapshot  
4) Corridor table rows  
5) Optional time‑series (if you batch export ticks)

You can represent it as either:

- One “wide” CSV per node (corridors flattened with prefixes), or  
- A pair of CSVs: `infra_node_header.csv` and `infra_node_corridors.csv`.  

Here’s the split form, which matches your current shard style.

#### 2.1.1 `infra_node_header.csv`

Each row = one node shard (live or sim).

Columns:

- `shard_id` – string (UUID or hash)  
- `module_type` – string (“cyboquatic.mar.v1”, “biopack.trayline.v1”, …)  
- `region` – string (“phoenix-az-us”)  
- `sim_or_live` – enum {`sim`,`live`}  
- `timestamp_utc` – int64 (epoch seconds)  
- `did_signature` – base58 or bech32  
- `knowledge_factor` – float64 (K)  
- `eco_impact_value` – float64 (E)  
- `risk_of_harm` – float64 (R)  
- `vt` – float64 (current residual \(V_t\))  
- `ker_version` – string (e.g. “KER2026.v1”)  
- `corridor_schema` – string (e.g. “CorridorSpec.WBTC2026.v3”)

Example header row:

```csv
shard_id,module_type,region,sim_or_live,timestamp_utc,did_signature,knowledge_factor,eco_impact_value,risk_of_harm,vt,ker_version,corridor_schema
"node-123","cyboquatic.mar.v1","phoenix-az-us","live",1769839200,"bostrom1ldg...","0.94","0.91","0.11","0.23","KER2026.v1","CorridorSpec.WBTC2026.v3"
```

#### 2.1.2 `infra_node_corridors.csv`

Each row = one corridor variable for one node.

Columns:

- `shard_id` – FK to header  
- `var_id` – string (`"r_heat"`, `"r_tox"`, `"r_soulsafety"` once mature, …)  
- `units` – string (e.g. “WBGT_norm”, “mg_L_norm”)  
- `safe` – float64  
- `gold` – float64  
- `hard` – float64  
- `weight` – float64 (w_j)  
- `lyap_channel` – int (u16)  
- `mandatory` – bool  
- `r_value` – float64 (current r_j(t))

Example:

```csv
shard_id,var_id,units,safe,gold,hard,weight,lyap_channel,mandatory,r_value
"node-123","r_heat","WBGT_norm",0.3,0.5,1.0,0.25,1,true,0.42
"node-123","r_tox","mg_L_norm",0.2,0.4,1.0,0.35,2,true,0.18
"node-123","r_soulsafety","index_norm",0.2,0.3,1.0,0.15,3,false,0.27
```

This layout lets you recompute \(V_t = \sum_j w_j r_j\) purely from CSV, and enforce `no corridor, no build` by checking that all `mandatory=true` var_ids from the spec appear for a given shard_id. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/389389b2-b1fe-4634-a7d3-178300a91c2a/wbtc-under-an-eibon-should-har-fbKbj59gSaW.OofgLPQ4tg.md)

***

### 2.2 `person.metrics.v1`

Similar pattern, but with personal metrics and no infra IDs.

#### 2.2.1 `person_header.csv`

Each row = one person shard per evolution cycle.

Columns:

- `person_did` – string (e.g. `bostrom18sd2u...`)  
- `evolution_cycle` – int64 (monotone)  
- `region` – string  
- `timestamp_utc` – int64  
- `did_signature` – string  
- `knowledge_factor` – float64 (K_person)  
- `eco_impact_value` – float64 (E_person)  
- `risk_of_harm` – float64 (R_person)  
- `vt` – float64 (personal residual)  
- `cy_points` – float64  
- `ker_version` – string  
- `corridor_schema` – string (e.g. “PersonCorridorSpec.2026.v1”)

Example:

```csv
person_did,evolution_cycle,region,timestamp_utc,did_signature,knowledge_factor,eco_impact_value,risk_of_harm,vt,cy_points,ker_version,corridor_schema
"bostrom18sd2u...",5,"phoenix-az-us",1769839200,"sig1...","0.95","0.92","0.07","0.18","124.0","KER2026.v1","PersonCorridorSpec.2026.v1"
```

#### 2.2.2 `person_corridors.csv`

Each row = one hard personal corridor coordinate.

Columns:

- `person_did` – FK  
- `evolution_cycle` – FK  
- `var_id` – string (`"r_health"`, `"r_combustion"`, `"r_neurorights"`, …)  
- `units` – string  
- `safe` – float64  
- `gold` – float64  
- `hard` – float64  
- `weight` – float64  
- `lyap_channel` – int  
- `mandatory` – bool (true for hard biophysical / eco corridors)  
- `r_value` – float64 (current r_j)

Example:

```csv
person_did,evolution_cycle,var_id,units,safe,gold,hard,weight,lyap_channel,mandatory,r_value
"bostrom18sd2u...",5,"r_health","index_norm",0.2,0.4,1.0,0.30,1,true,0.25
"bostrom18sd2u...",5,"r_combustion","index_norm",0.1,0.2,1.0,0.20,2,true,0.05
"bostrom18sd2u...",5,"r_neurorights","index_norm",0.2,0.3,1.0,0.25,3,true,0.19
```

#### 2.2.3 `person_soft_metrics.csv` (rsoulsafety, requity, etc.)

Optional telemetry only (soft‑logged), not enforced as hard corridors until you promote them.

Columns:

- `person_did`  
- `evolution_cycle`  
- `var_id` – `"r_soulsafety"`, `"r_equity"`, `"r_supplyresilience"`, …  
- `units` – string  
- `r_value` – float64  
- `sigma` – float64 (measurement uncertainty, optional)  
- `source` – string (sensor type / survey)

Example:

```csv
person_did,evolution_cycle,var_id,units,r_value,sigma,source
"bostrom18sd2u...",5,"r_soulsafety","index_norm",0.21,0.05,"vitalnet.app.survey.v2"
"bostrom18sd2u...",5,"r_equity","index_norm",0.18,0.07,"city.telemetry.iso26000.v1"
```

Governance rule:

- While in `person_soft_metrics`, these never participate in hard Lyapunov checks; they may influence K,E,R via analytics, but cannot block upgrades or tools. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/3f49a586-5fcd-4be7-aa21-c30596102bee/so-technically-this-is-the-bio-nDgdeIewRhS7aI2oBqoPBg.md)
- Promotion to hard corridors = moving selected `var_id`s into `person_corridors` with bands and weights, after kernels and sensing are validated.

***

## 3. How this plugs into your pipelines

- Eco‑Net / WBTC / Phoenix tools can ingest `infra_node_header.csv` and `infra_node_corridors.csv` to:
  - recompute Vt;  
  - compute K,E,R;  
  - enforce “no corridor, no build” and “violated corridor → derate/stop” for infra projects. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/994be4b5-c833-4d4d-ba32-b0c9d9a4ec7e/cycoquatic-instantiators-how-c-c.7kGeoiRMeXnnBTkkK_7A.md)

- VitalNet / personal upgrade logic ingests `person_header.csv`, `person_corridors.csv`, and `person_soft_metrics.csv` to:
  - compute personal K,E,R and CY;  
  - run `upgrade_allowed` for a given `(person_did, evolution_cycle)`;  
  - ensure upgrades stay within R_max ≤ 0.08–0.10 and E > 0, with no linkage to infra node shards. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/0532b9cc-99a8-4612-a4f6-fe7fdab383c2/with-the-content-below-help-us-Z8rCwZflR669cMMP6GfYSw.md)
