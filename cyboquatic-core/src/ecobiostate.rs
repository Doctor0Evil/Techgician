#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Nanoseconds since Unix epoch, UTC.
pub type EpochNanos = i128;

/// Scalar indices are dimensionless but must be tied to a documented computation.
/// Example: CognitiveLoadIndex \in [0,1] estimated from validated NeuroPC pipeline.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BioState {
    pub fatigue: f32,            // 0–1, validated against actigraphy / HRV
    pub duty: f32,               // 0–1, fraction of time under active load
    pub cognitive_load_index: f32, // 0–1, task- and protocol-defined
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EcoMetrics {
    pub eco_impact_score: f32,   // dimensionless index, explicitly derived from exergy, water, etc.
    pub device_hours: f32,       // hours, accumulated per device_id
    pub thermal_comfort_index: f32, // e.g. function of WBGT, air speed, clothing, activity
}

/// Waste load is a short vector of intensive loads normalized per unit time or volume.
/// Each entry MUST be tied to a lab-calibrated sensor or assay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteLoad {
    /// Per-class load, e.g. [organics, plastics, fibers] in kg/h or g/s normalized.
    pub per_class_load: Vec<f32>,
    /// Optional labels for diagnostics; not used in tight control loops.
    pub class_labels: Vec<String>,
}

/// Control intent is what the controller asked the plant to do at this timestep.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ControlIntent {
    pub pump_speed_rps: f32,     // revolutions per second
    pub valve_position: f32,     // 0–1 open
    pub cooler_setpoint_c: f32,  // degrees Celsius
}

/// Provenance: sovereign ownership and consent scope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    /// Sovereign subject DID / Bostrom address.
    pub subject_id: String,      // e.g. "bostrom18sd2u..."
    /// Host identifier, e.g. "NeuroPC/Organic_CPU".
    pub origin_host: String,
    /// Governance tag: "EVOLVE", "MUTATION", "TECH", etc.
    pub consent_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoBioState {
    pub timestamp: EpochNanos,
    pub bio_state: BioState,
    pub eco_metrics: EcoMetrics,
    pub waste_load: WasteLoad,
    pub control_intent: ControlIntent,
    pub provenance: Provenance,
}
