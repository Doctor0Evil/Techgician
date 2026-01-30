#[derive(Clone, Debug)]
pub struct MicroEpochProtocol {
    pub protocol_id: String,
    pub varied_parameter: String, // "intensity", "timing"
    pub steps: u32,
    pub step_delta: f32,
    pub safety_envelope: String,  // link to constraints profile
}

#[derive(Clone, Debug)]
pub struct DiscoveredComponent {
    pub base_pattern: ModulationPattern,
    pub context: Vec<String>,        // tasks, states where it worked
    pub evidence_ref: String,        // hash to dataset
    pub promoted_name: String,       // new canonical component name
    pub version: String,
}
