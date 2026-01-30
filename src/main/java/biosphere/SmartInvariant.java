package biosphere;

public interface SmartInvariant {
    String id(); // e.g., "SMART::BLOOD::OXYGEN::NEUROPC"
    boolean verifyDeterminism(String blockHash, String previousStateHash);
    boolean verifyTraceability(String proposalId, String cycleId);
}
