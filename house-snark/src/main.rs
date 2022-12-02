extern crate core;

use clap::Parser;

use crate::{
    config::{
        Cli, Command, GenerateKeysCmd, GenerateKeysFromSrsCmd, GenerateProofCmd, GenerateSrsCmd,
    },
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

fn main() {
    setup_eyre();
    env_logger::init();

    let cli: Cli = Cli::parse();
    match cli.command {
        Command::GenerateSrs(GenerateSrsCmd {
            system,
            num_constraints,
            num_variables,
            degree,
        }) => {
            let srs = system.generate_srs(num_constraints, num_variables, degree);
            save_srs(&srs, &system.id());
        }
        Command::GenerateKeysFromSrs(GenerateKeysFromSrsCmd {
            relation,
            system,
            srs_file,
        }) => {
            let srs = read_srs(srs_file);
            let keys = system.generate_keys(relation.clone(), srs);
            save_keys(&relation.id(), &system.id(), &keys.pk, &keys.vk);
        }
        Command::GenerateKeys(GenerateKeysCmd { relation, system }) => {
            let keys = system.generate_keys(relation.clone());
            save_keys(&relation.id(), &system.id(), &keys.pk, &keys.vk);
        }
        Command::GenerateProof(GenerateProofCmd {
            relation,
            system,
            proving_key_file,
        }) => {
            let proving_key = read_proving_key(proving_key_file);
            let proof = system.prove(relation.clone(), proving_key);
            let public_input = serialize(&relation.public_input());
            save_proving_artifacts(&relation.id(), &system.id(), &proof, &public_input);
        }
        Command::RedWedding => match kill_all_snarks() {
            Ok(_) => println!("Cleaning succeeded"),
            Err(e) => eprintln!("Cleaning failed: {:?}", e),
        },
    }
}
