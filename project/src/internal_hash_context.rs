use crate::{SHA256_ALG_ID, SHA3_256_ALG_ID};
use sha2::{Digest, Sha256};
use sha3::Sha3_256;
use InternalHashContex::*;

/// This is an internal enum type that maps algorithm IDs into their correct type.
/// The advantage of using this is that it will guarantee that algorithm IDs are unique
///
#[allow(clippy::large_enum_variant)]
#[repr(u32)]
pub enum InternalHashContex {
    Sha256Context(sha2::Sha256) = SHA256_ALG_ID,
    Sha3_256Context(sha3::Sha3_256) = SHA3_256_ALG_ID,
}

/// Implementation of hash API thought the enumerated types.
///
/// Check that the entire implementation here is safe and the handling of unsafe code
/// is reserved for the C function implementations in the lib file.
///
impl InternalHashContex {
    pub fn new(algorithm_id: u32) -> Option<Self> {
        match algorithm_id {
            SHA256_ALG_ID => Some(Sha256Context(Sha256::new())),
            SHA3_256_ALG_ID => Some(Sha3_256Context(Sha3_256::new())),
            _ => None,
        }
    }
    pub fn udpate(&mut self, input: &[u8]) {
        match self {
            Sha256Context(hasher) => hasher.update(input),
            Sha3_256Context(hasher) => hasher.update(input),
        };
    }
    pub fn finalize(&mut self, output: &mut [u8]) {
        match self {
            Sha256Context(hasher) => hasher.finalize_into_reset(output.into()),
            Sha3_256Context(hasher) => hasher.finalize_into_reset(output.into()),
        };
    }

    pub fn output_size(&self) -> usize {
        match self {
            Sha256Context(_) => Sha256::output_size(),
            Sha3_256Context(_) => Sha3_256::output_size(),
        }
    }
}
