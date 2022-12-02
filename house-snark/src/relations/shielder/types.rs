use super::CircuitField;

/// The circuit lifting for `CircuitField`.
pub(super) type FpVar = ark_r1cs_std::fields::fp::FpVar<CircuitField>;
/// The circuit lifting for the byte type.
pub(super) type ByteVar = ark_r1cs_std::uint8::UInt8<CircuitField>;

/// Underlying type for all notes, hashes, merkle roots etc. Just the result of hashing.
type Hash = [u64; 4];

pub type FrontendNullifier = u64;
pub type FrontendTrapdoor = u64;
pub type FrontendNote = [u64; 4];
pub type FrontendTokenId = u16;
pub type FrontendTokenAmount = u64;
pub type FrontendMerkleRoot = [u64; 4];
pub type FrontendMerklePath = Vec<[u64; 4]>;
pub type FrontendLeafIndex = u64;
pub type FrontendAccount = [u8; 32];
pub type FrontendMerklePathNode = [u64; 4];

pub(super) type BackendNullifier = CircuitField;
pub(super) type BackendTrapdoor = CircuitField;
pub(super) type BackendNote = CircuitField;
pub(super) type BackendTokenId = CircuitField;
pub(super) type BackendTokenAmount = CircuitField;
pub(super) type BackendMerkleRoot = CircuitField;
pub(super) type BackendMerklePath = Vec<CircuitField>;
pub(super) type BackendLeafIndex = CircuitField;
pub(super) type BackendAccount = CircuitField;

// Types accepted by the relation constructors.
//
// These are 1-tuple types instead of aliases in order to avoid any mistake and provide proper
// casting functions.
pub struct _FrontendNullifier(pub u64);
pub struct _FrontendTrapdoor(pub u64);
pub struct _FrontendNote(pub Hash);
pub struct _FrontendTokenId(pub u16);
pub struct _FrontendTokenAmount(pub u64);
pub struct _FrontendMerkleRoot(pub Hash);
pub struct _FrontendMerklePath(pub Vec<Hash>);
pub struct _FrontendLeafIndex(pub u64);
pub struct _FrontendAccount(pub [u8; 32]);
pub struct _FrontendMerklePathNode(pub Hash);

// Types used internally by the relations (but still outside circuit environment).
//
// These are 1-tuple types instead of aliases in order to avoid any mistake and provide proper
// casting functions.
pub(super) struct _BackendNullifier(pub CircuitField);
pub(super) struct _BackendTrapdoor(pub CircuitField);
pub(super) struct _BackendNote(pub CircuitField);
pub(super) struct _BackendTokenId(pub CircuitField);
pub(super) struct _BackendTokenAmount(pub CircuitField);
pub(super) struct _BackendMerkleRoot(pub CircuitField);
pub(super) struct _BackendMerklePath(pub Vec<CircuitField>);
pub(super) struct _BackendLeafIndex(pub CircuitField);
pub(super) struct _BackendAccount(pub CircuitField);

mod casting {
    use ark_ff::BigInteger256;

    use super::*;

    /// Generate casting between `frontend_type` and `backend_type`, where:
    ///  - `frontend_type` is assumed to be a 1-tuple struct wrapping a primitive integer type on at
    ///    most 64 bits,
    ///  - `backend_type` is assumed to be a 1-tuple struct wrapping `CircuitField`
    macro_rules! cast_integer {
        ($frontend_type:tt, $backend_type:tt) => {
            impl From<$frontend_type> for $backend_type {
                fn from($frontend_type(frontend_value): $frontend_type) -> Self {
                    Self(CircuitField::new(BigInteger256::from(
                        frontend_value as u64,
                    )))
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

    cast_integer!(_FrontendNullifier, _BackendNullifier);
    cast_integer!(_FrontendTrapdoor, _BackendTrapdoor);
    cast_integer!(_FrontendTokenId, _BackendTokenId);
    cast_integer!(_FrontendTokenAmount, _BackendTokenAmount);
    cast_integer!(_FrontendLeafIndex, _BackendLeafIndex);

    cast_hash!(_FrontendNote, _BackendNote);
    cast_hash!(_FrontendMerkleRoot, _BackendMerkleRoot);

    impl From<_FrontendMerklePath> for _BackendMerklePath {
        fn from(_FrontendMerklePath(frontend_path): _FrontendMerklePath) -> Self {
            Self(
                frontend_path
                    .iter()
                    .map(|node| _BackendNote::from(_FrontendNote(*node)).0)
                    .collect(),
            )
        }
    }

    impl From<_FrontendAccount> for _BackendAccount {
        fn from(_FrontendAccount(frontend_account): _FrontendAccount) -> Self {
            Self(CircuitField::new(BigInteger256::new([
                u64::from_le_bytes(frontend_account[0..8].try_into().unwrap()),
                u64::from_le_bytes(frontend_account[8..16].try_into().unwrap()),
                u64::from_le_bytes(frontend_account[16..24].try_into().unwrap()),
                u64::from_le_bytes(frontend_account[24..32].try_into().unwrap()),
            ])))
        }
    }
}
