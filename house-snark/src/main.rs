extern crate core;

use std::fmt::Debug;

use clap::Parser;

use crate::{
    config::{
        Cli, Command, GenerateKeysCmd, GenerateKeysFromSrsCmd, GenerateProofCmd, GenerateSrsCmd,
    },
    environment::CircuitField,
    rains_of_castamere::kill_all_snarks,
    relations::GetPublicInput,
    serialization::{
        read_proving_key, read_srs, save_keys, save_proving_artifacts, save_srs, serialize,
    },
};

mod config;
mod environment;
mod rains_of_castamere;
mod relations;
mod serialization;

fn setup_eyre() {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install().expect("Cannot install `eyre`");
}

fn system_id<S: Debug>(system: &S) -> String {
    format!("{:?}", system).to_lowercase()
}

fn main() {
    setup_eyre();
    env_logger::init();

    let cli: Cli = Cli::parse();
    match cli.command {
        Command::GenerateSrs(GenerateSrsCmd { system }) => {
            let srs = system.generate_srs();
            save_srs(&srs, &system_id(&system));
        }

        Command::GenerateKeysFromSrs(GenerateKeysFromSrsCmd {
            relation,
            system,
            srs_file,
        }) => {
            let srs = read_srs(srs_file);
            let keys = system.generate_keys(relation, srs);
            save_keys(&relation.id(), &system_id(&system), &keys.pk, &keys.vk);
        }

        Command::GenerateKeys(GenerateKeysCmd { relation, system }) => {
            let keys = system.generate_keys(relation);
            save_keys(&relation.id(), &system_id(&system), &keys.pk, &keys.vk);
        }

        Command::GenerateProof(GenerateProofCmd {
            relation,
            system,
            proving_key_file,
        }) => {
            let proving_key = read_proving_key(proving_key_file);
            let proof = system.prove(relation, proving_key);
            let public_input = serialize(&relation.public_input::<CircuitField>());
            save_proving_artifacts(&relation.id(), &system_id(&system), &proof, &public_input);
        }

        Command::RedWedding => match kill_all_snarks() {
            Ok(_) => println!("Cleaning succeeded"),
            Err(e) => eprintln!("Cleaning failed: {:?}", e),
        },
    }
}
