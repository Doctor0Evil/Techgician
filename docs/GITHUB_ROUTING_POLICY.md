# Eco-Project Router – GitHub Repository Filtering Policy

## 1. Default Trust Set

The router MUST treat the following orgs as default, high-trust sources:

- `Doctor0Evil`
- `Techgician`

All GitHub search URLs constructed by the router MUST include `user:Doctor0Evil user:Techgician` by default.[file:4]

## 2. Context-Aware Expansion

### 2.1 Heat & WBGT

If primary intent is `AirGlobeWBGT` and user text includes terms like:

- `urban heat`, `heat island`, `cool roof`, `green roof`, `shade`, `street tree`

THEN the router MAY append additional org filters, e.g.:

- `user:climate-trace`
- `user:OpenClimateFix`

(These names are placeholders; in production, maintain a signed YAML mapping.)

### 2.2 Soil Carbon / Wetlands / MAR

If text includes:

- `soil carbon`, `peatland`, `wetland`, `mar`, `managed aquifer recharge`

Then include:

- Future curated orgs for hydrology / soil-carbon modeling once vetted.

## 3. Onboarding New Orgs

1. Propose org in `config/trusted_orgs.yaml`.
2. Attach evidence:
   - Domain expertise (papers, deployments).
   - Open licensing.
   - No history of data or security malpractice.
3. Run a scripted audit:
   - Check repo topics (`climate`, `wbgt`, `hydrology`, `restoration`).
   - Scan for suspicious binary blobs or known-vulnerable dependencies.
4. A DID-signed governance decision MUST approve before inclusion.

## 4. Query Composition Rules

Given:

- `base_query` from intent (e.g., `airglobe wbgt safety control econet language:rust language:js`)
- `orgs` from trust policy

Router MUST construct:

```text
https://github.com/search?q=<urlencoded(
  base_query + " " + orgs.map(o => "user:" + o).join(" ")
)>&type=code
