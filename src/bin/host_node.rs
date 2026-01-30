//! Minimal host node entrypoint that enforces AugmentationRight before
//! starting the Organic_CPU / NeuroPC biophysical runtime.

#![forbid(unsafe_code)]

use std::process;

use biophysical_chain::runtime::assert_augmentation_right_sov_safe;

fn main() {
    // Path to the host's AugmentationRight shard.
    let shard_path = "qpu/data/shards/host-augmentation-right.aln";

    if let Err(err) = assert_augmentation_right_sov_safe(shard_path) {
        eprintln!("[FATAL] AugmentationRight verification failed: {err}");
        eprintln!("Host node will not start under a sovereignty-unsafe profile.");
        process::exit(1);
    }

    // If we reach here, the profile is sovereignty-safe. Proceed to
    // initialize the rest of your biophysical runtime and NeuroPC stack.
    // e.g.:
    //
    // let runtime = BiophysicalRuntime::new(...);
    // runtime.run_event_loop();
}
