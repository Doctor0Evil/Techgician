#ifndef SOVEREIGNTY_CORE_C_H
#define SOVEREIGNTY_CORE_C_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>

// Opaque handle to a SovereigntyCore instance.
typedef struct sovereignty_core_handle_t sovereignty_core_handle_t;

// Decision values mirrored from Rust enum Decision.
typedef enum {
    SOV_DECISION_ALLOWED = 0,
    SOV_DECISION_REJECTED = 1,
    SOV_DECISION_DEFERRED = 2
} sov_decision_t;

// Reason values mirrored from Rust enum DecisionReason.
typedef enum {
    SOV_REASON_OK = 0,
    SOV_REASON_ROH_CEILING = 1,
    SOV_REASON_NEURORIGHTS = 2,
    SOV_REASON_STAKE_MULTISIG = 3,
    SOV_REASON_TOKEN_SCOPE = 4,
    SOV_REASON_LIFEFORCE = 5,
    SOV_REASON_CLINICAL = 6
} sov_reason_t;

// Initialize a new sovereignty core instance.
//
// All paths are UTF-8, null-terminated.
// Returns NULL on error.
sovereignty_core_handle_t* sovereignty_core_init(
    const char* rohmodel_path,
    const char* stake_path,
    const char* neurorights_path,
    const char* evolve_path,
    const char* donutloop_path
);

// Destroy a sovereignty core instance.
void sovereignty_core_free(sovereignty_core_handle_t* handle);

// Evaluate a proposal from a JSON payload.
//
// `input_json` is a UTF-8 JSON string containing EvolutionProposal + EvaluationContext.
// On success, returns 0 and fills out_decision and out_reason.
// On failure, returns non-zero.
int sovereignty_core_evaluate_once(
    sovereignty_core_handle_t* handle,
    const char* input_json,
    sov_decision_t* out_decision,
    sov_reason_t* out_reason
);

#ifdef __cplusplus
}
#endif

#endif // SOVEREIGNTY_CORE_C_H
