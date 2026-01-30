#include "techgician/aln_did_stamp.hpp"

namespace techgician {

using nlohmann::json;

void to_json(json& j, const AlnDidBostromStampV1& s) {
    j = json{
        {"author_system", s.author_system},
        {"primary_bostrom_addr", s.primary_bostrom_addr},
        {"safe_addrs", s.safe_addrs},
        {"response_hash_hex", s.response_hash_hex},
        {"T_score_0_to_1", s.T_score_0_to_1},
        {"P_score_0_to_1", s.P_score_0_to_1},
        {"R_score_0_to_1", s.R_score_0_to_1},
        {"C_score_0_to_1", s.C_score_0_to_1},
        {"timestamp_utc_iso8601", s.timestamp_utc_iso8601}
    };
    if (s.alt_bostrom_addr.has_value()) {
        j["alt_bostrom_addr"] = *s.alt_bostrom_addr;
    }
    if (s.notes.has_value()) {
        j["notes"] = *s.notes;
    }
}

void from_json(const json& j, AlnDidBostromStampV1& s) {
    j.at("author_system").get_to(s.author_system);
    j.at("primary_bostrom_addr").get_to(s.primary_bostrom_addr);
    j.at("safe_addrs").get_to(s.safe_addrs);
    j.at("response_hash_hex").get_to(s.response_hash_hex);
    j.at("T_score_0_to_1").get_to(s.T_score_0_to_1);
    j.at("P_score_0_to_1").get_to(s.P_score_0_to_1);
    j.at("R_score_0_to_1").get_to(s.R_score_0_to_1);
    j.at("C_score_0_to_1").get_to(s.C_score_0_to_1);
    j.at("timestamp_utc_iso8601").get_to(s.timestamp_utc_iso8601);

    if (j.contains("alt_bostrom_addr") && !j.at("alt_bostrom_addr").is_null()) {
        s.alt_bostrom_addr = j.at("alt_bostrom_addr").get<std::string>();
    } else {
        s.alt_bostrom_addr.reset();
    }

    if (j.contains("notes") && !j.at("notes").is_null()) {
        s.notes = j.at("notes").get<std::string>();
    } else {
        s.notes.reset();
    }
}

} // namespace techgician
