# Eco-Project Router Intent Routing – WBGT-First Policy

## 1. Policy Goal

All Eco-Project Router components MUST treat WBGT / heat-stress signals as higher-priority than terms like “blockchain” when classifying user intent and selecting repositories. This aligns with OSHA/ISO 7243 guidance that WBGT ≥ ~26–28 °C is an immediate human safety concern in outdoor and field contexts.[file:3]

## 2. Priority Rules

1. If user text contains ANY of:
   - `wbgt`, `wet bulb`, `wet-bulb`, `heat stress`, `thermal comfort`, `thermal resilience`, `air-globe`, `airglobe`
   then:
   - Set primary intent = `AirGlobeWBGT`.
   - Suppress `EcoNetTokenomics` or generic blockchain intents even if words like `token`, `blockchain`, `NFT`, `rollup` appear.

2. If WBGT terms are absent, but both:
   - Ecological terms appear (`reforestation`, `mar`, `aquifer`, `wetland`, `urban heat`, `green roof`),
   - And blockchain terms appear,
   then:
   - Infer an *eco-first* intent (e.g., `CyboquaticCooling`, `CybocindricReactor`, `BiodegradableMaterials`) based on domain keywords,
   - Attach a secondary flag `intent::EcoNetTokenomics` for later routing, but do NOT make it primary.

3. Only when:
   - No WBGT or explicit fieldwork/environmental-risk terms appear,
   - And blockchain/ledger terms dominate,
   MAY `EcoNetTokenomics` become primary.

## 3. Implementation Hooks

### 3.1 Rust

Router MUST:

- Implement an ordered check:
  1. WBGT/air-globe/heat-stress tokens
  2. Cyboquatic/cybocindric/domain tokens
  3. Blockchain/tokenomics tokens

- Expose both:
  - `primary_intent: EcoIntent`
  - `secondary_intents: Vec<EcoIntent>`

### 3.2 JavaScript

The JS wrapper MUST mirror the same ordering for UI hints, and MUST label WBGT-classified routes visually (e.g., `intent::AirGlobeWBGT::SAFETY_CRITICAL`).

## 4. Test Cases (Non-Exhaustive)

- “Design Econet WBGT gates for air-globes” → primary `AirGlobeWBGT`, secondary `EcoNetTokenomics`.
- “Tokenized rewards for MAR groundwater recharge” → primary `CyboquaticCooling`, secondary `EcoNetTokenomics`.
- “Rollup-based eco-net accounting backend” → primary `EcoNetTokenomics`.

## 5. Research Hooks

For field deployments, log:

- Frequency of WBGT-first vs blockchain-first routing.
- Correlations between WBGT-keyword presence and:
  - Geospatial region,
  - Project type (urban greening, MAR, reforestation).

This enables future calibration of keyword lists and thresholds.
