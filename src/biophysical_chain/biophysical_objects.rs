#[derive(Clone, Debug)]
pub struct NeuroFunctionalTarget {
    pub id: String,          // e.g., "HapticGuidanceLaneLeftArm"
    pub path: String,        // e.g., "peripheral.haptics.left_arm"
    pub description: String, // human-readable, ALN-aligned
}

#[derive(Clone, Debug)]
pub enum ModulationShape {
    Constant,
    RampUp,
    RampDown,
    PulseTrain,
    CustomEnvelope(String), // reference to ALN / profile
}

#[derive(Clone, Debug)]
pub struct ModulationPattern {
    pub name: String,              // e.g., "VisualEdgeEnhanceKernelV1"
    pub target: NeuroFunctionalTarget,
    pub shape: ModulationShape,
    pub base_intensity: f32,       // normalized
    pub duration_secs: u64,
    pub version: String,           // "v1.0.0"
    pub risk_class: String,        // "low", "medium", "experimental"
}

#[derive(Clone, Debug)]
pub struct StateMarker {
    pub name: String,        // e.g., "FocusedFlowStateBeta"
    pub description: String,
    pub inferred_from: Vec<String>, // "EEG_beta", "HRV", "self_report_focus"
}
