use clap::{Args, Subcommand};
use relations::{
    CircuitField, ConstraintSynthesizer, ConstraintSystemRef, DepositRelation, FrontendAccount,
    FrontendLeafIndex, FrontendMerklePath, FrontendMerkleRoot, FrontendNote, FrontendNullifier,
    FrontendTokenAmount, FrontendTokenId, FrontendTrapdoor, GetPublicInput, LinearEquationRelation,
    MerkleTreeRelation, SynthesisError, WithdrawRelation, XorRelation,
};

use crate::parser::{
    parse_frontend_account, parse_frontend_merkle_path_single, parse_frontend_merkle_root,
    parse_frontend_note,
};

/// XOR relation: a ⊕ b = c
///
/// Relation with:
///  - 1 public input    (a | `public_xoree`)
///  - 1 private witness (b | `private_xoree`)
///  - 1 constant        (c | `result`)
/// such that: a ^ b = c.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct XorArgs {
    // ToDo: Especially for Groth16, it is better to provide public input as a field element.
    // Otherwise, we have to provide it to circuit bit by bit.
    #[clap(long, short = 'a', default_value = "2")]
    pub public_xoree: u8,
    #[clap(long, short = 'b', default_value = "3")]
    pub private_xoree: u8,
    #[clap(long, short = 'c', default_value = "1")]
    pub result: u8,
}

impl From<XorArgs> for XorRelation {
    fn from(args: XorArgs) -> Self {
        let XorArgs {
            public_xoree,
            private_xoree,
            result,
        } = args;
        XorRelation::new(public_xoree, private_xoree, result)
    }
}

/// Linear equation relation: a*x + b = y
///
/// Relation with:
///  - 1 private witness (x)
///  - 3 constants        (a, b, y)
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct LinearEquationArgs {
    /// constant (a slope)
    #[clap(long, default_value = "2")]
    pub a: u32,
    /// private witness
    #[clap(long, default_value = "7")]
    pub x: u32,
    /// constant(an intercept)
    #[clap(long, default_value = "5")]
    pub b: u32,
    /// constant
    #[clap(long, default_value = "19")]
    pub y: u32,
}

impl From<LinearEquationArgs> for LinearEquationRelation {
    fn from(args: LinearEquationArgs) -> Self {
        let LinearEquationArgs { a, x, b, y } = args;
        LinearEquationRelation::new(a, x, b, y)
    }
}

/// Arguments for creating a MerkeTreeRelation
#[derive(Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct MerkleTreeArgs {
    /// Seed bytes for rng, the more the merrier
    #[clap(long)]
    pub seed: Option<String>,

    /// Tree leaves, used to calculate the tree root
    #[clap(long, value_delimiter = ',')]
    pub leaves: Vec<u8>,

    /// Leaf of which membership is to be proven, must be one of the leaves
    #[clap(long)]
    pub leaf: u8,
}

impl From<MerkleTreeArgs> for MerkleTreeRelation {
    fn from(item: MerkleTreeArgs) -> Self {
        let MerkleTreeArgs { seed, leaves, leaf } = item;
        MerkleTreeRelation::new(leaves, leaf, seed)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct DepositArgs {
    // Public inputs
    #[clap(long, value_parser = parse_frontend_note)]
    pub note: FrontendNote,
    #[clap(long)]
    pub token_id: FrontendTokenId,
    #[clap(long)]
    pub token_amount: FrontendTokenAmount,

    // Private inputs.
    #[clap(long)]
    pub trapdoor: FrontendTrapdoor,
    #[clap(long)]
    pub nullifier: FrontendNullifier,
}

impl From<DepositArgs> for DepositRelation {
    fn from(args: DepositArgs) -> Self {
        let DepositArgs {
            note,
            token_id,
            token_amount,
            trapdoor,
            nullifier,
        } = args;
        DepositRelation::new(note, token_id, token_amount, trapdoor, nullifier)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct WithdrawArgs {
    // Public inputs
    #[clap(long)]
    pub old_nullifier: FrontendNullifier,
    #[clap(long, value_parser = parse_frontend_merkle_root)]
    pub merkle_root: FrontendMerkleRoot,
    #[clap(long, value_parser = parse_frontend_note)]
    pub new_note: FrontendNote,
    #[clap(long)]
    pub token_id: FrontendTokenId,
    #[clap(long)]
    pub token_amount_out: FrontendTokenAmount,
    #[clap(long)]
    pub fee: FrontendTokenAmount,
    #[clap(long, value_parser = parse_frontend_account)]
    pub recipient: FrontendAccount,

    // Private inputs
    #[clap(long)]
    pub old_trapdoor: FrontendTrapdoor,
    #[clap(long)]
    pub new_trapdoor: FrontendTrapdoor,
    #[clap(long)]
    pub new_nullifier: FrontendNullifier,
    #[clap(long, value_delimiter = ',', value_parser = parse_frontend_merkle_path_single)]
    pub merkle_path: FrontendMerklePath,
    #[clap(long)]
    pub leaf_index: FrontendLeafIndex,
    #[clap(long, value_parser = parse_frontend_note)]
    pub old_note: FrontendNote,
    #[clap(long)]
    pub whole_token_amount: FrontendTokenAmount,
    #[clap(long)]
    pub new_token_amount: FrontendTokenAmount,
}

impl From<WithdrawArgs> for WithdrawRelation {
    fn from(args: WithdrawArgs) -> Self {
        let WithdrawArgs {
            old_nullifier,
            merkle_root,
            new_note,
            token_id,
            token_amount_out,
            old_trapdoor,
            new_trapdoor,
            new_nullifier,
            merkle_path,
            leaf_index,
            old_note,
            whole_token_amount,
            new_token_amount,
            fee,
            recipient,
        } = args;
        WithdrawRelation::new(
            old_nullifier,
            merkle_root,
            new_note,
            token_id,
            token_amount_out,
            old_trapdoor,
            new_trapdoor,
            new_nullifier,
            merkle_path,
            leaf_index,
            old_note,
            whole_token_amount,
            new_token_amount,
            fee,
            recipient,
        )
    }
}

/// All implemented relations.
///
/// They should have corresponding definition in submodule.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Subcommand)]
pub enum Relation {
    Xor(XorArgs),
    LinearEquation(LinearEquationArgs),
    MerkleTree(MerkleTreeArgs),
    Deposit(DepositArgs),
    Withdraw(WithdrawArgs),
}

impl Relation {
    /// Relation identifier.
    pub fn id(&self) -> String {
        match &self {
            Relation::Xor(_) => String::from("xor"),
            Relation::LinearEquation(_) => String::from("linear_equation"),
            Relation::MerkleTree(_) => String::from("merkle_tree"),
            Relation::Deposit(_) => String::from("deposit"),
            Relation::Withdraw(_) => String::from("withdraw"),
        }
    }
}

impl ConstraintSynthesizer<CircuitField> for Relation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        match self {
            Relation::Xor(args @ XorArgs { .. }) => {
                <XorArgs as Into<XorRelation>>::into(args).generate_constraints(cs)
            }

            Relation::LinearEquation(args @ LinearEquationArgs { .. }) => {
                <LinearEquationArgs as Into<LinearEquationRelation>>::into(args)
                    .generate_constraints(cs)
            }

            Relation::MerkleTree(args @ MerkleTreeArgs { .. }) => {
                <MerkleTreeArgs as Into<MerkleTreeRelation>>::into(args).generate_constraints(cs)
            }

            Relation::Deposit(args @ DepositArgs { .. }) => {
                <DepositArgs as Into<DepositRelation>>::into(args).generate_constraints(cs)
            }

            Relation::Withdraw(args @ WithdrawArgs { .. }) => {
                <WithdrawArgs as Into<WithdrawRelation>>::into(args).generate_constraints(cs)
            }
        }
    }
}

impl GetPublicInput<CircuitField> for Relation {
    fn public_input(&self) -> Vec<CircuitField> {
        match self {
            Relation::Xor(args @ XorArgs { .. }) => {
                <XorArgs as Into<XorRelation>>::into(args.to_owned()).public_input()
            }

            Relation::LinearEquation(args @ LinearEquationArgs { .. }) => {
                <LinearEquationArgs as Into<LinearEquationRelation>>::into(args.to_owned())
                    .public_input()
            }

            Relation::MerkleTree(args @ MerkleTreeArgs { .. }) => {
                <MerkleTreeArgs as Into<MerkleTreeRelation>>::into(args.to_owned()).public_input()
            }

            Relation::Deposit(args @ DepositArgs { .. }) => {
                <DepositArgs as Into<DepositRelation>>::into(*args).public_input()
            }

            Relation::Withdraw(args @ WithdrawArgs { .. }) => {
                <WithdrawArgs as Into<WithdrawRelation>>::into(args.to_owned()).public_input()
            }
        }
    }
}
