#pragma once

#include "techgician/aln_did_stamp.hpp"

#include <stdexcept>
#include <string>

namespace techgician {

class StampValidationError : public std::runtime_error {
public:
    explicit StampValidationError(const std::string& msg)
        : std::runtime_error(msg) {}
};

bool is_valid_bostrom_addr(const std::string& addr);
bool is_score_valid(double v);
bool is_hex_64(const std::string& s);

void validate_stamp_basic(const AlnDidBostromStampV1& stamp);
void validate_stamp_with_payload(const AlnDidBostromStampV1& stamp, const std::string& response_payload);

} // namespace techgician
