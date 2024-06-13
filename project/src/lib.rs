mod internal_hash_context;

use internal_hash_context::InternalHashContex;
use static_assertions as sa;
use std::ptr;

/// Crypto return type with Success or Failure error cases.
///
/// Probably the best way to treat this enum type could be a similar approach that we use for
/// the internal hash context, with global constant error definitions that are mapped internally
/// to an enum type. This might allow for safer validation of unique error values assigned to
/// unique enum labels, as well as adding message error handling with logs etc.
///
/// Using enum type is not bad, but we need to make sure that the Success/failure here always
/// match the correct enum (integer) representation in the C api. Moving values around could
/// change the behavior of this and cause problems.
///
/// See the case for the SHA256_ALG_ID, for additional information.
///
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub enum CryptoResult {
    Success,
    Failure,
    PointerCannotBeNull,
    BadOrUnsupportedAlgorithm,
    UninitializedOrCorruptedContext,
    BadBufferOutputSize,
}

/// Crypto algorithm types
///
/// Due to internal match cases, if the algorithm types are of the Enum type instead of constants
/// we have a (possibly undefined) behavior that match assume any invalid enum type coming from the
/// C api to be the last enum in the definition. That is, we cannot treat an unsupported case for
/// bad algorithm input, as it will default to the last one declared in the enum. To avoid this,
/// we changed the enum to a global ID constant that can be evaluated internally and can match
/// invalid inputs in a well-defined way.
///
pub const SHA256_ALG_ID: u32 = 0;
pub const SHA3_256_ALG_ID: u32 = 1;

/// Byte length of Context internal state.
pub const CRYPTO_CONTEXT_STATE_SIZE: usize = 45;

/// Crypto Context containing memory for internal usage.
/// Memory is declared as array of 64bit integer to preserve memory alignment
/// and match the internal context type.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CryptoContext {
    pub state: [u64; CRYPTO_CONTEXT_STATE_SIZE],
}

sa::assert_eq_size!(CryptoContext, InternalHashContex);
sa::assert_eq_align!(CryptoContext, InternalHashContex);

/// Crypto init
/// Initialize a context with a given algorithm.
///
/// # Safety
/// Pointer must not be null and memory should be correctly allocated by caller.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn crypto_init(ctx: *mut CryptoContext, algorithm_id: u32) -> CryptoResult {
    if ctx.is_null() {
        return CryptoResult::PointerCannotBeNull;
    }
    InternalHashContex::new(algorithm_id)
        .map(|new_ctx| {
            ptr::write(ctx.cast(), new_ctx);
            CryptoResult::Success
        })
        .unwrap_or(CryptoResult::BadOrUnsupportedAlgorithm)
}

/// Crypto update
/// Updates internal hash context with input buffer
///
/// # Safety
/// Pointers must not be null and input length must be correct.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn crypto_update(
    ctx: *mut CryptoContext,
    input: *const u8,
    input_length: usize,
) -> CryptoResult {
    if ctx.is_null() || input.is_null() {
        return CryptoResult::PointerCannotBeNull;
    }
    let internal_context: &mut InternalHashContex = &mut *(ctx.cast());
    // This must be checked as `from_raw_parts_mut` has many undefined behavior conditions that
    // must be guaranteed by the caller. In this case, some of them must be guaranteed by the user.
    let input_slice = std::slice::from_raw_parts(input, input_length);
    internal_context.udpate(input_slice);
    CryptoResult::Success
}

/// Crypto Finalize
/// Finalizes the digest computation and outputs result into bytes of the CryptoDigest.
///
/// Perhaps a better approach to this API would be to allow the user to allocate necessary
/// memory on his side. This will make the function a bit more unsafe in the user perspective,
/// as he can use bad sizes and cause undefined behavior of the code. However, if we assume
/// the user reads the doc and uses well-formed arrays of proper size, then the API could support
/// additional algorithms (smaller and larger outputs).
///
/// # Safety
/// Pointers must not be null.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn crypto_finalize(
    ctx: *mut CryptoContext,
    output: *mut u8,
    output_length: usize,
) -> CryptoResult {
    if ctx.is_null() || output.is_null() {
        return CryptoResult::PointerCannotBeNull;
    }
    let internal_context: &mut InternalHashContex = &mut *(ctx.cast());
    if output_length != internal_context.output_size() {
        return CryptoResult::BadBufferOutputSize;
    }
    // This must be checked as `from_raw_parts_mut` has many undefined behavior conditions that
    // must be guaranteed by the caller. In this case, some of them must be guaranteed by the user.
    let output_slice = std::slice::from_raw_parts_mut(output, output_length);
    internal_context.finalize(output_slice);

    CryptoResult::Success
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha3_256() {
        let mut state = CryptoContext {
            state: [0u64; CRYPTO_CONTEXT_STATE_SIZE],
        };
        unsafe {
            assert_eq!(
                crypto_init(&mut state as *mut CryptoContext, SHA3_256_ALG_ID),
                CryptoResult::Success
            );
        };

        // Update with the string "hello"
        let input = b"hello";
        unsafe {
            assert_eq!(
                crypto_update(
                    &mut state as *mut CryptoContext,
                    input.as_ptr(),
                    input.len(),
                ),
                CryptoResult::Success
            )
        }

        // Finalize and get the digest
        let mut digest = [0u8; 32];
        unsafe {
            assert_eq!(
                crypto_finalize(
                    &mut state as *mut CryptoContext,
                    digest.as_mut_ptr(),
                    digest.len()
                ),
                CryptoResult::Success
            )
        };

        // Expected output of SHA3_256 of hello string (utf8)
        let expected: [u8; 32] = [
            0x33, 0x38, 0xbe, 0x69, 0x4f, 0x50, 0xc5, 0xf3, 0x38, 0x81, 0x49, 0x86, 0xcd, 0xf0,
            0x68, 0x64, 0x53, 0xa8, 0x88, 0xb8, 0x4f, 0x42, 0x4d, 0x79, 0x2a, 0xf4, 0xb9, 0x20,
            0x23, 0x98, 0xf3, 0x92,
        ];

        assert_eq!(digest, expected);
    }

    #[test]
    fn test_sha256() {
        // Initialize the state
        let mut state = CryptoContext {
            state: [0u64; CRYPTO_CONTEXT_STATE_SIZE],
        };
        unsafe {
            assert_eq!(
                crypto_init(&mut state as *mut CryptoContext, SHA256_ALG_ID),
                CryptoResult::Success
            );
        };

        // Update with the string "hello"
        let input = b"hello";
        unsafe {
            assert_eq!(
                crypto_update(
                    &mut state as *mut CryptoContext,
                    input.as_ptr(),
                    input.len(),
                ),
                CryptoResult::Success
            )
        }

        // Finalize and get the digest
        let mut digest = [0u8; 32];
        unsafe {
            assert_eq!(
                crypto_finalize(
                    &mut state as *mut CryptoContext,
                    digest.as_mut_ptr(),
                    digest.len()
                ),
                CryptoResult::Success
            )
        };

        // Expected output of SHA256 of hello string (utf8)
        let expected: [u8; 32] = [
            0x2c, 0xf2, 0x4d, 0xba, 0x5f, 0xb0, 0xa3, 0x0e, 0x26, 0xe8, 0x3b, 0x2a, 0xc5, 0xb9,
            0xe2, 0x9e, 0x1b, 0x16, 0x1e, 0x5c, 0x1f, 0xa7, 0x42, 0x5e, 0x73, 0x04, 0x33, 0x62,
            0x93, 0x8b, 0x98, 0x24,
        ];

        assert_eq!(digest, expected);
    }
}
