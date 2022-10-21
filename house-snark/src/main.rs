extern crate core;

use std::path::PathBuf;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use clap::Parser;

use crate::{
    config::{Cli, Command, GenerateKeysCmd, GenerateProofCmd},
    environment::{
        Environment, Fr, GmEnv, GrothEnv, Proof, ProvingKey, ProvingSystem, VerifyingKey,
    },
    rains_of_castamere::kill_all_snarks,
    relations::{GetPublicInput, Relation},
    serialization::{read_proving_key, save_keys, save_proving_artifacts},
};

mod config;
mod environment;
mod environment2;
mod rains_of_castamere;
mod relations;
mod serialization;

fn generate_keys_for<Env: Environment>(relation: Relation)
where
    VerifyingKey<Env>: CanonicalSerialize,
    ProvingKey<Env>: CanonicalSerialize,
{
    let mut rng = StdRng::from_seed([0u8; 32]);

    let (pk, vk) =
        Env::setup(relation, &mut rng).unwrap_or_else(|e| panic!("Problems with setup: {:?}", e));

    save_keys::<Env>(relation.id(), pk, vk);
}

fn generate_proving_artifacts_for<Env: Environment>(relation: Relation, proving_key_file: PathBuf)
where
    ProvingKey<Env>: CanonicalDeserialize,
    Proof<Env>: CanonicalSerialize,
    Fr<Env>: CanonicalSerialize,
{
    let proving_key = read_proving_key::<Env>(proving_key_file);

    let mut rng = StdRng::from_seed([0u8; 32]);
    let proof = Env::prove(&proving_key, relation, &mut rng)
        .unwrap_or_else(|e| panic!("Cannot prove: {:?}", e));

    let public_input = relation.public_input();

    save_proving_artifacts::<Env>(relation.id(), proof, public_input);
}

fn red_wedding() {
    match kill_all_snarks() {
        Ok(_) => println!("Cleaning succeeded"),
        Err(e) => eprintln!("Cleaning failed: {:?}", e),
    }
}

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
        Command::GenerateKeys(GenerateKeysCmd { relation, system }) => match system {
            ProvingSystem::Groth16 => generate_keys_for::<GrothEnv>(relation),
            ProvingSystem::Gm17 => generate_keys_for::<GmEnv>(relation),
        },

        Command::GenerateProof(GenerateProofCmd {
            relation,
            system,
            proving_key_file,
        }) => match system {
            ProvingSystem::Groth16 => {
                generate_proving_artifacts_for::<GrothEnv>(relation, proving_key_file)
            }
            ProvingSystem::Gm17 => {
                generate_proving_artifacts_for::<GmEnv>(relation, proving_key_file)
            }
        },

        Command::RedWedding => red_wedding(),
    }
}
