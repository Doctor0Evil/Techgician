pub mod types;
pub mod artifacts;
pub mod guards;
pub mod donutloop;

// Re-exports for consumers
pub use crate::types::{
    EvolutionProposal, Decision, DecisionReason, EnvironmentPlane, BioStateSnapshot,
};
pub use crate::artifacts::{
    RohModel, StakePolicy, NeurorightsPolicy, ArtifactLoader,
};
pub use crate::guards::SovereigntyCore;
pub use crate::donutloop::{DonutloopEntry, DonutloopSink};
