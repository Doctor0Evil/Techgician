#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeuromorphicKeyOpKind {
    Rotate,
    Derive,
    Revoke,
    EscrowUpdate,
}

#[derive(Clone, Debug)]
pub struct NeuromorphicKeyOp {
    pub op_id: ChainId,
    pub kind: NeuromorphicKeyOpKind,
    // Logical domains: control-plane, audit, nanoswarm, etc.
    pub domain: String,
    pub reason: String,              // human-readable, chat-linked
    pub consent_token: Option<String>, // user-signed token when required
}

pub struct EvolutionProposal {
    // existing fields ...
    pub key_ops: Vec<NeuromorphicKeyOp>,
}
