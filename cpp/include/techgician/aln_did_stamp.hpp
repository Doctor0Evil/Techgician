#pragma once

#include <optional>
#include <string>
#include <vector>

#include <nlohmann/json.hpp>

namespace techgician {

struct AlnDidBostromStampV1 {
    std::string author_system;
    std::string primary_bostrom_addr;
    std::optional<std::string> alt_bostrom_addr;
    std::vector<std::string> safe_addrs;
    std::string response_hash_hex;
    double T_score_0_to_1;
    double P_score_0_to_1;
    double R_score_0_to_1;
    double C_score_0_to_1;
    std::string timestamp_utc_iso8601;
    std::optional<std::string> notes;
};

void to_json(nlohmann::json& j, const AlnDidBostromStampV1& s);
void from_json(const nlohmann::json& j, AlnDidBostromStampV1& s);

} // namespace techgician
