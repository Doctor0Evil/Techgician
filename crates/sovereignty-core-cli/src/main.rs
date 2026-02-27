use clap::{Parser, Subcommand};
use sovereignty_core::{
    ArtifactLoader, DonutloopEntry, SovereigntyCore,
};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Sovereignty-core CLI for validating TECH artifacts and reading donutloop.
#[derive(Parser, Debug)]
#[command(name = "sovereignty-core-cli")]
#[command(about = "Validate .rohmodel.aln, .stake.aln, .neurorights.json and summarize donutloop.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Validate RohModel, StakePolicy, and NeurorightsPolicy artifacts.
    Validate {
        /// Path to rohmodel JSON/ALN-converted file
        #[arg(long)]
        rohmodel: String,
        /// Path to stake JSON/ALN-converted file
        #[arg(long)]
        stake: String,
        /// Path to neurorights JSON file
        #[arg(long)]
        neurorights: String,
    },
    /// Print a summary of donutloop entries (.donutloop.aln / .evolve.jsonl).
    SummarizeDonutloop {
        /// Path to donutloop JSONL file (.donutloop.aln)
        #[arg(long)]
        donutloop: String,
    },
    /// Dry-run initialization of sovereignty-core guard with given artifacts.
    Init {
        #[arg(long)]
        rohmodel: String,
        #[arg(long)]
        stake: String,
        #[arg(long)]
        neurorights: String,
        #[arg(long)]
        evolve_path: String,
        #[arg(long)]
        donutloop_path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { rohmodel, stake, neurorights } => {
            match run_validate(&rohmodel, &stake, &neurorights) {
                Ok(_) => {
                    println!("OK: all artifacts parsed successfully.");
                }
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::SummarizeDonutloop { donutloop } => {
            if let Err(e) = run_summarize_donutloop(&donutloop) {
                eprintln!("ERROR: {e}");
                std::process::exit(1);
            }
        }
        Commands::Init {
            rohmodel,
            stake,
            neurorights,
            evolve_path,
            donutloop_path,
        } => {
            match SovereigntyCore::new(
                &rohmodel,
                &stake,
                &neurorights,
                &evolve_path,
                &donutloop_path,
            ) {
                Ok(_) => {
                    println!("OK: sovereignty-core initialized with given artifacts.");
                }
                Err(e) => {
                    eprintln!("ERROR initializing sovereignty-core: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn run_validate(
    rohmodel: &str,
    stake: &str,
    neurorights: &str,
) -> Result<(), String> {
    let r = ArtifactLoader::load_rohmodel(rohmodel)
        .map_err(|e| format!("rohmodel: {e}"))?;
    if r.roh_ceiling <= 0.0 || r.roh_ceiling > 1.0 {
        return Err(format!("rohmodel: invalid roh_ceiling {}", r.roh_ceiling));
    }

    let s = ArtifactLoader::load_stake(stake)
        .map_err(|e| format!("stake: {e}"))?;
    if s.host_did.is_empty() {
        return Err("stake: host_did must not be empty".into());
    }

    let n = ArtifactLoader::load_neurorights(neurorights)
        .map_err(|e| format!("neurorights: {e}"))?;
    if n.forbid_decision_use_domains.is_empty() {
        eprintln!("WARN: neurorights.forbid_decision_use_domains is empty.");
    }

    println!("rohmodel.roh_ceiling = {}", r.roh_ceiling);
    println!("stake.host_did = {}", s.host_did);
    println!(
        "neurorights.forbid_dream_state = {}",
        n.forbid_dream_state
    );
    Ok(())
}

fn run_summarize_donutloop(donutloop_path: &str) -> Result<(), String> {
    let file = File::open(donutloop_path)
        .map_err(|e| format!("open: {e}"))?;
    let reader = BufReader::new(file);

    let mut count: u64 = 0;
    let mut allowed: u64 = 0;
    let mut rejected: u64 = 0;
    let mut deferred: u64 = 0;
    let mut max_roh_after: f64 = 0.0;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("read: {e}"))?;
        if line.trim().is_empty() { continue; }

        let entry: DonutloopEntry = serde_json::from_str(&line)
            .map_err(|e| format!("parse: {e}"))?;

        count += 1;
        max_roh_after = max_roh_after.max(entry.roh_after);

        match entry.decision {
            sovereignty_core::Decision::Allowed => allowed += 1,
            sovereignty_core::Decision::Rejected => rejected += 1,
            sovereignty_core::Decision::Deferred => deferred += 1,
        }
    }

    println!("donutloop entries: {}", count);
    println!("Allowed: {}, Rejected: {}, Deferred: {}", allowed, rejected, deferred);
    println!("max roh_after recorded: {:.6}", max_roh_after);
    Ok(())
}
