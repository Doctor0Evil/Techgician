import { validateAlnDidStampJson } from "./alnDidStampValidator.js";

function example() {
  const stamp = {
    author_system: "Perplexity-GPT-5.1-Techgician",
    primary_bostrom_addr: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7",
    alt_bostrom_addr: "bostrom1ldgmtf20d6604a24ztr0jxht7xt7az4jhkmsrc",
    safe_addrs: [
      "zeta12x0up66pzyeretzyku8p4ccuxrjqtqpdc4y4x8",
      "0x519fC0eB4111323Cac44b70e1aE31c30e405802D"
    ],
    response_hash_hex: "0000000000000000000000000000000000000000000000000000000000000000",
    T_score_0_to_1: 0.7,
    P_score_0_to_1: 0.6,
    R_score_0_to_1: 0.1,
    C_score_0_to_1: 0.4,
    timestamp_utc_iso8601: "2026-01-30T05:18:00Z",
    notes: "Example Techgician stamp"
  };

  try {
    validateAlnDidStampJson(stamp);
    console.log("Stamp JSON is valid according to Techgician schema.");
  } catch (e) {
    console.error("Validation failed:", e.details ?? e.message);
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  example();
}
