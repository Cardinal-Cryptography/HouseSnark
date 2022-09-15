use std::default::Default;

use clap::Parser;

use crate::{
    config::{Cli, Command, GenerateCmd},
    relations::{MerkleTreeRelation, Relation, SnarkRelation, XorRelation},
};

mod config;
mod relations;

fn main() {
    let cli: Cli = Cli::parse();
    match cli.command {
        Command::Generate(GenerateCmd { relation }) => {
            let artifacts = match relation {
                Relation::Xor => XorRelation::default().generate_artifacts(),
                Relation::MerkleTree => {
                    MerkleTreeRelation::default().generate_artifacts()
                }
            };
            println!("{:?}", artifacts);
        }
        Command::RedWedding => println!("red wedding"),
    }
}
