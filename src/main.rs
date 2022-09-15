use std::default::Default;

use ark_serialize::CanonicalSerialize;
use clap::Parser;

use crate::{
    config::{Cli, Command, GenerateCmd},
    relations::{Artifacts, MerkleTreeRelation, Relation, SnarkRelation, XorRelation},
};

mod config;
mod relations;

type SerializedArtifacts = Artifacts<Vec<u8>, Vec<u8>, Vec<u8>>;

fn serialize_artifacts<VK: CanonicalSerialize, P: CanonicalSerialize, PI: CanonicalSerialize>(
    artifacts: &Artifacts<VK, P, PI>,
) -> SerializedArtifacts {
    let Artifacts {
        verifying_key: vk,
        public_input: input,
        proof,
    } = artifacts;

    let mut serialized_vk = vec![0; vk.serialized_size()];
    vk.serialize(&mut serialized_vk[..]).unwrap();

    let mut serialized_proof = vec![0; proof.serialized_size()];
    proof.serialize(&mut serialized_proof[..]).unwrap();

    let mut serialized_input = vec![0; input.serialized_size()];
    input.serialize(&mut serialized_input[..]).unwrap();

    SerializedArtifacts {
        verifying_key: serialized_vk,
        proof: serialized_proof,
        public_input: serialized_input,
    }
}

fn main() {
    let cli: Cli = Cli::parse();
    match cli.command {
        Command::Generate(GenerateCmd { relation }) => {
            let artifacts = match relation {
                Relation::Xor => XorRelation::default().generate_artifacts(),
                Relation::MerkleTree => todo!(),
            };
            println!("{:?}", serialize_artifacts(&artifacts));
        }
        Command::RedWedding => println!("red wedding"),
    }
}
