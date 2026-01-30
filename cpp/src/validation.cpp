#include "techgician/validation.hpp"

#include <cctype>
#include <iomanip>
#include <sstream>

#include <openssl/evp.h>

namespace techgician {

bool is_valid_bostrom_addr(const std::string& addr) {
    if (addr.size() < 8 || addr.size() > 80) {
        return false;
    }
    if (addr.rfind("bostrom", 0) != 0) {
        return false;
    }
    for (char c : addr) {
        if (!(std::islower(static_cast<unsigned char>(c)) || std::isdigit(static_cast<unsigned char>(c)))) {
            return false;
        }
    }
    return true;
}

bool is_score_valid(double v) {
    return v >= 0.0 && v <= 1.0;
}

bool is_hex_64(const std::string& s) {
    if (s.size() != 64) {
        return false;
    }
    for (char c : s) {
        if (!std::isdigit(static_cast<unsigned char>(c)) &&
            !(c >= 'a' && c <= 'f')) {
            return false;
        }
    }
    return true;
}

static std::string sha256_hex(const std::string& payload) {
    unsigned char hash[EVP_MAX_MD_SIZE];
    unsigned int len = 0;

    EVP_MD_CTX* ctx = EVP_MD_CTX_new();
    if (!ctx) {
        throw StampValidationError("EVP_MD_CTX_new failed");
    }
    if (EVP_DigestInit_ex(ctx, EVP_sha256(), nullptr) != 1) {
        EVP_MD_CTX_free(ctx);
        throw StampValidationError("EVP_DigestInit_ex failed");
    }
    if (EVP_DigestUpdate(ctx, payload.data(), payload.size()) != 1) {
        EVP_MD_CTX_free(ctx);
        throw StampValidationError("EVP_DigestUpdate failed");
    }
    if (EVP_DigestFinal_ex(ctx, hash, &len) != 1) {
        EVP_MD_CTX_free(ctx);
        throw StampValidationError("EVP_DigestFinal_ex failed");
    }
    EVP_MD_CTX_free(ctx);

    std::ostringstream oss;
    for (unsigned int i = 0; i < len; ++i) {
        oss << std::hex << std::setfill('0') << std::setw(2)
            << static_cast<int>(hash[i]);
    }
    return oss.str();
}

void validate_stamp_basic(const AlnDidBostromStampV1& stamp) {
    if (!is_valid_bostrom_addr(stamp.primary_bostrom_addr)) {
        throw StampValidationError("primary_bostrom_addr invalid format");
    }
    if (stamp.safe_addrs.empty()) {
        throw StampValidationError("safe_addrs must not be empty");
    }
    if (!is_score_valid(stamp.T_score_0_to_1) ||
        !is_score_valid(stamp.P_score_0_to_1) ||
        !is_score_valid(stamp.R_score_0_to_1) ||
        !is_score_valid(stamp.C_score_0_to_1)) {
        throw StampValidationError("score out of range [0,1]");
    }
    if (!is_hex_64(stamp.response_hash_hex)) {
        throw StampValidationError("response_hash_hex invalid hex");
    }
    if (stamp.timestamp_utc_iso8601.empty()) {
        throw StampValidationError("timestamp must not be empty");
    }
}

void validate_stamp_with_payload(const AlnDidBostromStampV1& stamp, const std::string& response_payload) {
    validate_stamp_basic(stamp);
    std::string recomputed = sha256_hex(response_payload);
    if (recomputed != stamp.response_hash_hex) {
        throw StampValidationError("response hash mismatch");
    }
}

} // namespace techgician
