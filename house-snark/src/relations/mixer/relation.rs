use ark_crypto_primitives::{
    crh::{TwoToOneCRH, TwoToOneCRHGadget},
    merkle_tree::Config,
    CRHGadget, Path, PathVar,
};
use ark_ff::{PrimeField, ToBytes};
use ark_r1cs_std::{
    prelude::{AllocVar, Boolean, EqGadget, UInt8},
    uint128::UInt128,
    ToBytesGadget,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::hash::Hash;

use super::{
    gadgets::LeafHashGadget,
    hash_functions::LeafHash,
    tree::{PoseidonMerkleTreePath, Root},
};
use crate::{
    relations::mixer::{
        gadgets::{TwoToOneHashGadget, TwoToOneHashParamsVar},
        hash_functions::TwoToOneHash,
    },
    CircuitField, GetPublicInput,
};

// R1CS equivalent types
pub type NoteVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, CircuitField>>::OutputVar;
pub type Note = <TwoToOneHash as TwoToOneCRH>::Output;

#[derive(Clone)]
pub struct DepositRelation {
    // public
    pub value: u128,
    pub token_id: u128,
    pub note: Vec<u8>, //Vec<UInt8<CircuitField>>,
    // private
    pub trapdoor: u128,
    pub nullifier: u128,
    // params for CRH
    // pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
}

/// constraints for the deposit relation
///
/// note = CHR(nullifier, trapdoor, token_id, value)
impl ConstraintSynthesizer<CircuitField> for DepositRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let value = UInt128::new_input(ark_relations::ns!(cs.clone(), "value_var"), || {
            Ok(&self.value)
        })?;
        let token_id = UInt128::new_input(ark_relations::ns!(cs.clone(), "token_id_var"), || {
            Ok(&self.token_id)
        })?;

        let trapdoor = UInt128::new_witness(ark_relations::ns!(cs.clone(), "value_var"), || {
            Ok(&self.trapdoor)
        })?;

        let nullifier = UInt128::new_witness(ark_relations::ns!(cs.clone(), "value_var"), || {
            Ok(&self.nullifier)
        })?;

        // TODOs
        // - type note as output of a hash function (in the field)
        // - hash the byte array of inputs
        // - enforce equality with note (instead of dumb byte by byte check)

        let bytes: Vec<UInt8<CircuitField>> = [
            nullifier.to_bytes().unwrap(),
            trapdoor.to_bytes().unwrap(),
            token_id.to_bytes().unwrap(),
            value.to_bytes().unwrap(),
        ]
        .concat();

        self.note
            .into_iter()
            .zip(bytes.iter())
            .try_for_each(|(a, b)| {
                let note_byte_in_field =
                    UInt8::new_input(ark_relations::ns!(cs.clone(), "note_byte"), || Ok(&a))?;
                note_byte_in_field.enforce_equal(&b.clone())
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ark_crypto_primitives::CRH;
    use ark_relations::r1cs::ConstraintSystem;

    use super::*;
    use crate::relations::mixer::tree::PoseidonMerkleTree;
    #[test]
    fn deposit_constraints_correctness() {
        let cs = ConstraintSystem::new_ref();

        let trapdoor = 0u128;
        let nullifier = 0u128;

        let value = 1u128;
        let token_id = 2u128;

        let note: Vec<u8> = [
            nullifier.to_le_bytes(),
            trapdoor.to_le_bytes(),
            token_id.to_le_bytes(),
            value.to_le_bytes(),
        ]
        .concat();

        let circuit = DepositRelation {
            trapdoor,
            nullifier,
            value,
            token_id,
            note, // _to_one_crh_params,
        };

        circuit.generate_constraints(cs.clone()).unwrap();

        let is_satisfied = cs.is_satisfied().unwrap();
        if !is_satisfied {
            println!("{:?}", cs.which_is_unsatisfied());
        }

        assert!(is_satisfied);
    }

    #[test]
    fn withdraw_constraints_correctness() {
        let mut rng = ark_std::test_rng();
        let leaf_crh_params = LeafHash::setup(&mut rng).unwrap();
        let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRH>::setup(&mut rng).unwrap();

        let nullifier = 0u128;
        let token_id = 2u128;
        let value_out = 1u128;
        let new_value = 1u128;
        let value = 1u128;
        let trapdoor = 0u128;
        let new_trapdoor = 0u128;
        let new_nullifier = 0u128;

        // TODO : construct proper note (from the bytes)
        let note = vec![1u8];
        // TODO : construct proper note (from the bytes)
        let new_note = vec![1u8];

        // a tree of notes
        let tree = PoseidonMerkleTree::new(
            &leaf_crh_params,
            &two_to_one_crh_params,
            // TODO : use notes (from bytes)
            &[vec![1u8], vec![2u8], vec![3u8]], // the i-th entry is the i-th leaf.
        )
        .unwrap();

        let merkle_root = tree.root();

        // TODO : proof should match the note, duh
        let merkle_proof = tree.generate_proof(2).unwrap(); // we're 0-indexing!

        let cs = ConstraintSystem::new_ref();

        let circuit = WithdrawRelation {
            nullifier,
            merkle_root,
            new_note,
            token_id,
            value_out,
            new_value,
            value,
            note,
            merkle_proof,
            trapdoor,
            new_trapdoor,
            new_nullifier,
        };

        circuit.generate_constraints(cs.clone()).unwrap();

        let is_satisfied = cs.is_satisfied().unwrap();
        if !is_satisfied {
            println!("{:?}", cs.which_is_unsatisfied());
        }

        assert!(is_satisfied);
    }
}

#[derive(Clone)]
pub struct WithdrawRelation {
    // public
    nullifier: u128,
    merkle_root: Root,
    new_note: Vec<u8>,
    token_id: u128,
    value_out: u128,
    // private
    new_value: u128,
    value: u128,
    note: Vec<u8>,
    merkle_proof: PoseidonMerkleTreePath,
    trapdoor: u128,
    new_trapdoor: u128,
    new_nullifier: u128,
    //     pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
    //     pub leaf_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
}

impl ConstraintSynthesizer<CircuitField> for WithdrawRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        todo!()
    }
}

// #[derive(Clone)]
// pub struct DepositRelation<Trapdoor, Nullifier, TokenId> {
//     // private
//     pub trapdoor: Trapdoor,
//     pub nullifier: Nullifier,

//     // public
//     pub note: NoteVar,
//     pub token_id: TokenId,
//     pub value: u128,

//     pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
// }

// impl<Trapdoor, Nullifier, TokenId> ConstraintSynthesizer<CircuitField>
//     for DepositRelation<Trapdoor, Nullifier, TokenId>
// where
//     Trapdoor: ToBytesGadget<CircuitField>,
//     Nullifier: ToBytesGadget<CircuitField>,
//     TokenId: ToBytesGadget<CircuitField>,
// {
//     fn generate_constraints(
//         self,
//         cs: ConstraintSystemRef<CircuitField>,
//     ) -> Result<(), SynthesisError> {
//         let two_to_one_crh_params =
//             TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

//         let n = self.nullifier.to_bytes()?;
//         let n = n.as_slice();
//         let t = self.trapdoor.to_bytes()?;
//         let t = t.as_slice();

//         let token_id = self.token_id.to_bytes()?;
//         let token_id = token_id.as_slice();
//         let value = UInt128::new_input(ark_relations::ns!(cs, "value_var"), || Ok(&self.value))?;

//         let left_hash = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, n, t)?;
//         let right_hash = TwoToOneHashGadget::evaluate(
//             &two_to_one_crh_params,
//             token_id,
//             value.to_bytes()?.as_slice(),
//         )?;
//         let final_hash = TwoToOneHashGadget::evaluate(
//             &two_to_one_crh_params,
//             left_hash.to_bytes()?.as_slice(),
//             right_hash.to_bytes()?.as_slice(),
//         )?;

//         final_hash.enforce_equal(&self.note)?;

//         Ok(())
//     }
// }

// impl<Trapdoor, Nullifier, TokenId> GetPublicInput<CircuitField>
//     for DepositRelation<Trapdoor, Nullifier, TokenId>
// {
// }

// /// The R1CS equivalent of the the Merkle tree root.
// pub type RootVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, CircuitField>>::OutputVar;

// #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
// pub struct MerkleConfig;
// impl Config for MerkleConfig {
//     type LeafHash = LeafHash;
//     type TwoToOneHash = TwoToOneHash;
// }

// /// The root of the byte Merkle tree.
// pub type Root = <TwoToOneHash as TwoToOneCRH>::Output;

// #[derive(Clone)]
// pub struct MerklePathWrapper {
//     path: Path<MerkleConfig>,
// }

// #[derive(Clone)]
// pub struct WithdrawRelation<Trapdoor: Clone, Nullifier: Clone, TokenId: Clone> {
//     pub old_t: Trapdoor,
//     pub new_t: Trapdoor,
//     pub new_value: u128,
//     pub merkle_proof: MerklePathWrapper,
//     pub old_n: Nullifier,
//     pub old_note: NoteVar,

//     pub token_id: TokenId,
//     pub value: u128,
//     pub value_out: u128,
//     pub merkle_root: Root,
//     pub new_n: Nullifier,
//     pub new_note: NoteVar,
//     pub _recipient: u128,
//     pub _fee: u128,

//     pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
//     pub leaf_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
// }

// impl<Trapdoor, Nullifier, TokenId> WithdrawRelation<Trapdoor, Nullifier, TokenId>
// where
//     Trapdoor: ToBytesGadget<CircuitField> + Clone,
//     Nullifier: ToBytesGadget<CircuitField> + Clone,
//     TokenId: ToBytesGadget<CircuitField> + Clone,
// {
//     fn verify_old_inputs(
//         self,
//         cs: ConstraintSystemRef<CircuitField>,
//     ) -> Result<(), SynthesisError> {
//         let two_to_one_crh_params =
//             TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

//         let old_n = self.old_n.to_bytes()?;
//         let old_n = old_n.as_slice();
//         let old_t = self.old_t.to_bytes()?;
//         let old_t = old_t.as_slice();

//         let token_id = self.token_id.to_bytes()?;
//         let token_id = token_id.as_slice();
//         let value_out = UInt128::new_input(ark_relations::ns!(cs, "value_out_var"), || {
//             Ok(&self.value_out)
//         })?;

//         let left_hash = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, old_n, old_t)?;
//         let right_hash = TwoToOneHashGadget::evaluate(
//             &two_to_one_crh_params,
//             token_id,
//             value_out.to_bytes()?.as_slice(),
//         )?;
//         let final_hash = TwoToOneHashGadget::evaluate(
//             &two_to_one_crh_params,
//             left_hash.to_bytes()?.as_slice(),
//             right_hash.to_bytes()?.as_slice(),
//         )?;

//         final_hash.enforce_equal(&self.old_note)?;

//         Ok(())
//     }

//     fn verify_new_inputs(
//         self,
//         cs: ConstraintSystemRef<CircuitField>,
//     ) -> Result<(), SynthesisError> {
//         let two_to_one_crh_params =
//             TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

//         let new_n = self.new_n.to_bytes()?;
//         let new_n = new_n.as_slice();
//         let new_t = self.new_t.to_bytes()?;
//         let new_t = new_t.as_slice();

//         let token_id = self.token_id.to_bytes()?;
//         let token_id = token_id.as_slice();
//         let new_value = UInt128::new_input(ark_relations::ns!(cs, "new_value_var"), || {
//             Ok(&self.new_value)
//         })?;
//         let value_out = UInt128::new_input(ark_relations::ns!(cs, "value_out_var"), || {
//             Ok(&self.value_out)
//         })?;

//         let value = UInt128::new_input(ark_relations::ns!(cs, "value_var"), || Ok(&self.value))?;
//         let sum = UInt128::addmany(&[new_value.clone(), value_out])?;
//         sum.enforce_equal(&value)?;

//         let left_hash = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, new_n, new_t)?;
//         let right_hash = TwoToOneHashGadget::evaluate(
//             &two_to_one_crh_params,
//             token_id,
//             new_value.to_bytes()?.as_slice(),
//         )?;
//         let final_hash = TwoToOneHashGadget::evaluate(
//             &two_to_one_crh_params,
//             left_hash.to_bytes()?.as_slice(),
//             right_hash.to_bytes()?.as_slice(),
//         )?;

//         final_hash.enforce_equal(&self.old_note)?;

//         Ok(())
//     }

//     fn verify_merkle_proof(
//         self,
//         cs: ConstraintSystemRef<CircuitField>,
//     ) -> Result<(), SynthesisError> {
//         let root =
//             RootVar::new_input(ark_relations::ns!(cs, "root_var"), || Ok(&self.merkle_root))?;
//         let path: PathVar<_, LeafHashGadget, TwoToOneHashGadget, _> =
//             PathVar::new_input(ark_relations::ns!(cs, "merkle_proof_var"), || {
//                 Ok(&self.merkle_proof.path)
//             })?;
//         let two_to_one_crh_params =
//             TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;
//         let leaf_crh_params = TwoToOneHashParamsVar::new_constant(cs, &self.leaf_crh_params)?;

//         path.verify_membership(
//             &leaf_crh_params,
//             &two_to_one_crh_params,
//             &root,
//             &self.old_note,
//         )?
//         .enforce_equal(&Boolean::TRUE)?;

//         Ok(())
//     }
// }

// impl<Trapdoor, Nullifier, TokenId> ConstraintSynthesizer<CircuitField>
//     for WithdrawRelation<Trapdoor, Nullifier, TokenId>
// where
//     Trapdoor: ToBytesGadget<CircuitField> + Clone,
//     Nullifier: ToBytesGadget<CircuitField> + Clone,
//     TokenId: ToBytesGadget<CircuitField> + Clone,
// {
//     fn generate_constraints(
//         self,
//         cs: ConstraintSystemRef<CircuitField>,
//     ) -> Result<(), SynthesisError> {
//         self.clone().verify_merkle_proof(cs.clone())?;
//         self.clone().verify_old_inputs(cs.clone())?;
//         self.verify_new_inputs(cs)?;

//         Ok(())
//     }
// }

// impl<Trapdoor: Clone, Nullifier: Clone, TokenId: Clone> GetPublicInput<CircuitField>
//     for WithdrawRelation<Trapdoor, Nullifier, TokenId>
// {
// }
