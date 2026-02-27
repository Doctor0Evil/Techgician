#ifndef SOVEREIGNTY_CORE_CPP_HPP
#define SOVEREIGNTY_CORE_CPP_HPP

#include "sovereignty_core_c.h"
#include <stdexcept>
#include <string>

namespace sov {

struct DecisionResult {
    sov_decision_t decision;
    sov_reason_t reason;
};

class Core {
public:
    Core(const std::string& rohmodel_path,
         const std::string& stake_path,
         const std::string& neurorights_path,
         const std::string& evolve_path,
         const std::string& donutloop_path)
    : handle_(nullptr)
    {
        handle_ = sovereignty_core_init(
            rohmodel_path.c_str(),
            stake_path.c_str(),
            neurorights_path.c_str(),
            evolve_path.c_str(),
            donutloop_path.c_str()
        );
        if (!handle_) {
            throw std::runtime_error("sovereignty_core_init failed");
        }
    }

    ~Core() {
        if (handle_) {
            sovereignty_core_free(handle_);
            handle_ = nullptr;
        }
    }

    DecisionResult evaluate_once(const std::string& input_json) {
        sov_decision_t d;
        sov_reason_t r;
        int rc = sovereignty_core_evaluate_once(handle_, input_json.c_str(), &d, &r);
        if (rc != 0) {
            throw std::runtime_error("sovereignty_core_evaluate_once failed");
        }
        return DecisionResult{d, r};
    }

    Core(const Core&) = delete;
    Core& operator=(const Core&) = delete;

    Core(Core&& other) noexcept : handle_(other.handle_) {
        other.handle_ = nullptr;
    }

    Core& operator=(Core&& other) noexcept {
        if (this != &other) {
            if (handle_) {
                sovereignty_core_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }

private:
    sovereignty_core_handle_t* handle_;
};

} // namespace sov

#endif // SOVEREIGNTY_CORE_CPP_HPP
