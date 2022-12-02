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

pub mod casting {
    use std::str::FromStr;

    use ark_ff::BigInteger256;

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
            Self(CircuitField::new(BigInteger256::new([
                u64::from_le_bytes(frontend_account[0..8].try_into().unwrap()),
                u64::from_le_bytes(frontend_account[8..16].try_into().unwrap()),
                u64::from_le_bytes(frontend_account[16..24].try_into().unwrap()),
                u64::from_le_bytes(frontend_account[24..32].try_into().unwrap()),
            ])))
        }
    }

    macro_rules! parse_frontend_type {
        ($frontend_type:tt, $inner_type:ty) => {
            impl FromStr for $frontend_type {
                type Err = String;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(Self(
                        <$inner_type>::from_str(s)
                            .map_err(|e| format!("Failed to parse: {e:?}"))?,
                    ))
                }
            }
        };
    }

    parse_frontend_type!(FrontendNullifier, u64);
    parse_frontend_type!(FrontendTrapdoor, u64);
    parse_frontend_type!(FrontendTokenId, u16);
    parse_frontend_type!(FrontendTokenAmount, u64);
    parse_frontend_type!(FrontendLeafIndex, u64);
}
