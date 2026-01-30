#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BioDomain {
    Blood,
    Sugar,
    Smart,
    Wave,
    Brain,
    Oxygen,
}

impl BioDomain {
    pub fn hex_tag(&self) -> &'static str {
        match self {
            BioDomain::Blood  => "0xB10oD001",
            BioDomain::Sugar  => "0x5U6AR002",
            BioDomain::Smart  => "0x5MART003",
            BioDomain::Wave   => "0xWA4E004",
            BioDomain::Brain  => "0x8RAIN005",
            BioDomain::Oxygen => "0x0XY6EN06",
        }
    }
}

#[derive(Clone, Debug)]
pub struct IrreversibleToken {
    pub transcript_hash: String,
    pub user_signature: String, // e.g., Bostrom sig, not interpreted here
    pub issued_at: SystemTime,
}

// In EvolutionProposal:
pub irreversible_token: Option<IrreversibleToken>;
if constraints.require_irreversible_confirmation
    && pattern.reversibility == Reversibility::Irreversible
    && proposal.irreversible_token.is_none()
{
    accepted = false;
    reasons.push("Irreversible pattern requires explicit irreversible_token".into());
}

#[derive(Clone, Debug)]
pub struct SmartMeta {
    pub learning_epoch: u64,
    pub qpudatashard_id: String,
    pub entropy_seed_hex: String, // logged, not used for randomness here
}

// In AutomationCycle:
pub smart_meta: Option<SmartMeta>;
