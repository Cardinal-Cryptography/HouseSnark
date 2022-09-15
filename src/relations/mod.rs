use clap::ValueEnum;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ValueEnum)]
pub enum Relation {
    Xor,
    MerkleTree,
}
