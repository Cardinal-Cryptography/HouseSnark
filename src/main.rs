use std::{default::Default, fs};

use ark_ec::PairingEngine;
use ark_serialize::CanonicalSerialize;
use clap::Parser;

use crate::{
    config::{Cli, Command, GenerateCmd},
    rains_of_castamere::kill_all_snarks,
    relations::{Artifacts, LinearEqRelation, PureArtifacts, Relation, SnarkRelation, XorRelation},
    serialization::{serialize_artifacts, SerializedArtifacts},
};

mod config;
mod rains_of_castamere;
mod relations;
mod serialization;

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
    // We call `id()` and `generate_artifacts()` in every branch to avoid boxing stuff.
    let (rel_name, artifacts) = match relation {
        Relation::Xor => (
            XorRelation::id(),
            XorRelation::default().generate_artifacts(),
        ),
        Relation::LinearEquation => (
            LinearEqRelation::id(),
            LinearEqRelation::default().generate_artifacts(),
        ),
    };

    save_artifacts(rel_name, artifacts);
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
