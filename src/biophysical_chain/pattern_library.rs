#[derive(Clone, Debug)]
pub struct PatternLibraryEntry {
    pub pattern: ModulationPattern,
    pub expected_effect: String,
    pub compatible_tasks: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct SovereignChannelProfile {
    pub owner_id: String,          // your Bostrom / DID
    pub profile_name: String,      // "SovereignChannelProfiles2026v1"
    pub allowed_patterns: Vec<String>, // pattern.name ids
    pub preferred_risk_class: String,
}
