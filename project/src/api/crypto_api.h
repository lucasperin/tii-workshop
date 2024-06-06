#ifndef CRYPTO_API_H
#define CRYPTO_API_H

#include "stdint.h"
#include "stddef.h"

#define CRYPTO_ALIGNED(n) __attribute__((aligned(n)))
#define CRYPTO_MUST_USE __attribute__((warn_unused_result))

/**
 * Crypto algorithm types
 *
 * Due to internal match cases, if the algorithm types are of the Enum type instead of constants
 * we have a (possibly undefined) behavior that match assume any invalid enum type coming from the
 * C api to be the last enum in the definition. That is, we cannot treat an unsupported case for
 * bad algorithm input, as it will default to the last one declared in the enum. To avoid this,
 * we changed the enum to a global ID constant that can be evaluated internally and can match
 * invalid inputs in a well-defined way.
 *
 */
#define SHA256_ALG_ID 0

#define SHA3_256_ALG_ID 1

/**
 * Byte length of Context internal state.
 */
#define CRYPTO_CONTEXT_STATE_SIZE 45

/**
 * Byte length of digest produced by library.
 */
#define CRYPTO_DIGEST_SIZE 32

/**
 * Crypto return type with Success or Failure error cases.
 *
 * Probably the best way to treat this enum type could be a similar approach that we use for
 * the internal hash context, with global constant error definitions that are mapped internally
 * to an enum type. This might allow for safer validation of unique error values assigned to
 * unique enum labels, as well as adding message error handling with logs etc.
 *
 * Using enum type is not bad, but we need to make sure that the Success/failure here always
 * match the correct enum (integer) representation in the C api. Moving values around could
 * change the behavior of this and cause problems.
 *
 * See the case for the SHA256_ALG_ID, for additional information.
 *
 */
typedef enum {
  Success,
  Failure,
  PointerCannotBeNull,
  BadOrUnsupportedAlgorithm,
  UninitializedOrCorruptedContext,
} CryptoResult;

/**
 * Crypto Context containing memory for internal usage.
 * Memory is declared as array of 64bit integer to preserve memory alignment
 * and match the internal context type.
 */
typedef struct {
  uint64_t state[CRYPTO_CONTEXT_STATE_SIZE];
} CryptoContext;

/**
 * Crypto digest, contains the bytes of the hash output.
 *
 * This type struct is defined as transparent in the rust code, translated to an array type in C.
 * The reason for this is that it is safer to cast the transparent type to a rust array type in
 * the rust code, since the struct (non-transparent) type could have additional overhead. This
 * allows us to cast the structure directly to an array safely, instead having to cast explicitly
 * the internal array member.
 *
 */
typedef uint8_t CryptoDigest[CRYPTO_DIGEST_SIZE];

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Crypto init
 * Initialize a context with a given algorithm.
 *
 * # Safety
 * Pointer must not be null and memory should be correctly allocated by caller.
 */
CRYPTO_MUST_USE CryptoResult crypto_init(CryptoContext *ctx, uint32_t algorithm_id);

/**
 * Crypto update
 * Updates internal hash context with input buffer
 *
 * # Safety
 * Pointers must not be null and input length must be correct.
 */
CRYPTO_MUST_USE CryptoResult crypto_update(CryptoContext *ctx, const uint8_t *input, size_t length);

/**
 * Crypto Finalize
 * Finalizes the digest computation and outputs result into bytes of the CryptoDigest.
 *
 * Perhaps a better approach to this API would be to allow the user to allocate necessary
 * memory on his side. This will make the function a bit more unsafe in the user perspective,
 * as he can use bad sizes and cause undefined behavior of the code. However, if we assume
 * the user reads the doc and uses well-formed arrays of proper size, then the API could support
 * additional algorithms (smaller and larger outputs).
 *
 * # Safety
 * Pointers must not be null.
 */
CRYPTO_MUST_USE CryptoResult crypto_finalize(CryptoContext *ctx, CryptoDigest *result);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* CRYPTO_API_H */
