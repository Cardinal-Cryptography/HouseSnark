use super::CircuitField;

/// The circuit lifting for `CircuitField`.
pub(super) type FpVar = ark_r1cs_std::fields::fp::FpVar<CircuitField>;
/// The circuit lifting for the byte type.
pub(super) type ByteVar = ark_r1cs_std::uint8::UInt8<CircuitField>;

/// Underlying type for all notes, hashes, merkle roots etc. Just the result of hashing.
type Hash = [u64; 4];

// Types accepted by the relation constructors.
//
// These are 1-tuple types instead of aliases in order to avoid any mistake and provide proper
// casting functions.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FrontendNullifier(pub u64);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FrontendTrapdoor(pub u64);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FrontendNote(pub Hash);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FrontendTokenId(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FrontendTokenAmount(pub u64);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FrontendMerkleRoot(pub Hash);
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FrontendMerklePath(pub Vec<Hash>);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FrontendLeafIndex(pub u64);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FrontendAccount(pub [u8; 32]);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FrontendMerklePathNode(pub Hash);

// Types used internally by the relations (but still outside circuit environment).
//
// These are 1-tuple types instead of aliases in order to avoid any mistake and provide proper
// casting functions.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(super) struct BackendNullifier(pub CircuitField);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(super) struct BackendTrapdoor(pub CircuitField);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(super) struct BackendNote(pub CircuitField);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(super) struct BackendTokenId(pub CircuitField);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(super) struct BackendTokenAmount(pub CircuitField);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(super) struct BackendMerkleRoot(pub CircuitField);
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(super) struct BackendMerklePath(pub Vec<CircuitField>);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(super) struct BackendLeafIndex(pub CircuitField);
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(super) struct BackendAccount(pub CircuitField);

/// This module contains implementations for casting between frontend and backend types and some
/// other useful translations.
///
/// Where it made sense and was possible, we used macros, so that to ensure that there is at most
/// one implementation for particular primitive conversion (like `u64` -> `CircuitField`).
mod casting {
    use ark_ff::{BigInteger, BigInteger256};

    use super::*;

    //--------------------
    // Frontend -> Backend
    //--------------------

    /// Generate casting between `frontend_type` and `backend_type`, where:
    ///  - `frontend_type` is assumed to be a 1-tuple struct wrapping a primitive integer type on at
    ///    most 64 bits,
    ///  - `backend_type` is assumed to be a 1-tuple struct wrapping `CircuitField`
    macro_rules! cast_integer {
        ($frontend_type:tt, $backend_type:tt) => {
            impl From<$frontend_type> for $backend_type {
                fn from($frontend_type(frontend_value): $frontend_type) -> Self {
                    // Self(CircuitField::new(BigInteger256::from(
                    //     frontend_value as u64,
                    // )))
                    Self(CircuitField::from(frontend_value as u64))
                }
            }
        };
    }

    /// Generate casting between `frontend_type` and `backend_type`, where:
    ///  - `frontend_type` is assumed to be a 1-tuple struct wrapping `Hash` type,
    ///  - `backend_type` is assumed to be a 1-tuple struct wrapping `CircuitField`
    macro_rules! cast_hash {
        ($frontend_type:tt, $backend_type:tt) => {
            impl From<$frontend_type> for $backend_type {
                fn from($frontend_type(frontend_value): $frontend_type) -> Self {
                    Self(CircuitField::new(BigInteger256::new(frontend_value)))
                }
            }
        };
    }

    cast_integer!(FrontendNullifier, BackendNullifier);
    cast_integer!(FrontendTrapdoor, BackendTrapdoor);
    cast_integer!(FrontendTokenId, BackendTokenId);
    cast_integer!(FrontendTokenAmount, BackendTokenAmount);
    cast_integer!(FrontendLeafIndex, BackendLeafIndex);

    cast_hash!(FrontendNote, BackendNote);
    cast_hash!(FrontendMerkleRoot, BackendMerkleRoot);

    impl From<FrontendMerklePath> for BackendMerklePath {
        fn from(FrontendMerklePath(frontend_path): FrontendMerklePath) -> Self {
            Self(
                frontend_path
                    .iter()
                    .map(|node| BackendNote::from(FrontendNote(*node)).0)
                    .collect(),
            )
        }
    }

    impl From<FrontendAccount> for BackendAccount {
        fn from(FrontendAccount(frontend_account): FrontendAccount) -> Self {
            Self(CircuitField::new(BigInteger256::new(bytes_to_4xu64(
                &frontend_account,
            ))))
        }
    }

    //--------------------
    // Byte representation
    //--------------------

    impl FrontendNote {
        /// Create a note from the first 32 bytes of `bytes`.
        pub fn from_bytes(bytes: &[u8]) -> FrontendNote {
            FrontendNote(bytes_to_4xu64(bytes))
        }

        /// Serializes note to bytes.
        pub fn to_bytes(self) -> Vec<u8> {
            self.0
                .iter()
                .map(|elem| elem.to_le_bytes().to_vec())
                .collect::<Vec<_>>()
                .concat()
        }
    }

    macro_rules! to_bytes_through_backend {
        ($frontend_type:ty, $backend_type:ty) => {
            impl $frontend_type {
                pub fn to_bytes_through_backend(self) -> Vec<u8> {
                    BigInteger256::from(self.0 as u64).to_bytes_le()
                }
            }
        };
    }

    to_bytes_through_backend!(FrontendTokenId, BackendTokenId);
    to_bytes_through_backend!(FrontendTokenAmount, BackendTokenAmount);
    to_bytes_through_backend!(FrontendTrapdoor, BackendTrapdoor);
    to_bytes_through_backend!(FrontendNullifier, BackendNullifier);

    // -----------------
    // Auxiliary methods
    // -----------------

    fn bytes_to_4xu64(bytes: &[u8]) -> [u64; 4] {
        [
            u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
            u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
            u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
        ]
    }
}
