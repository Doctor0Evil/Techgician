//! Biophysical runtime module, wired with AugmentationRight bootstrap.

#![forbid(unsafe_code)]

pub mod augmentation_bootstrap;

// Re-export for main or host node.
pub use augmentation_bootstrap::assert_augmentation_right_sov_safe;
