use std::{default::Default, fs};
use ark_ec::PairingEngine;

use ark_serialize::CanonicalSerialize;
use clap::Parser;

use crate::{
    config::{Cli, Command, GenerateCmd},
    rains_of_castamere::kill_all_snarks,
    relations::{
        Artifacts, MerkleTreeRelation, PureArtifacts, Relation, SnarkRelation, XorRelation,
    },
};

mod config;
mod rains_of_castamere;
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

fn save_artifacts<Pairing: PairingEngine, FieldElement: CanonicalSerialize>(
    rel_name: &str,
    artifacts: PureArtifacts<Pairing, FieldElement>,
) {
    let SerializedArtifacts {
        verifying_key,
        proof,
        public_input,
    } = serialize_artifacts(&artifacts);

    fs::write(format!("{}.vk.bytes", rel_name), verifying_key).unwrap();
    fs::write(format!("{}.proof.bytes", rel_name), proof).unwrap();
    fs::write(format!("{}.public_input.bytes", rel_name), public_input).unwrap();
}

fn generate_output_from(relation: Relation) {
    let relation = match relation {
        Relation::Xor => XorRelation::default(),
        Relation::MerkleTree => todo!(),
    };
    let artifacts = relation.generate_artifacts();

    save_artifacts(relation.id(), artifacts);
}

fn red_wedding() {
    match kill_all_snarks() {
        Ok(_) => println!("Cleaning succeeded"),
        Err(e) => eprintln!("Cleaning failed: {:?}", e),
    }
}

fn main() {
    env_logger::init();

    let cli: Cli = Cli::parse();
    match cli.command {
        Command::Generate(GenerateCmd { relation }) => generate_output_from(relation),
        Command::RedWedding => red_wedding(),
    }
}
