use std::{fs, path::PathBuf};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use clap::Parser;

use crate::{
    config::{Cli, Command, GenerateKeysCmd, GenerateProofCmd},
    environment::{Environment, Fr, GrothEnv, Proof, ProvingKey, VerifyingKey},
    rains_of_castamere::kill_all_snarks,
    relations::{PureKeys, PureProvingArtifacts, SnarkRelation},
    serialization::{serialize_keys, serialize_proving_artifacts},
};

mod config;
mod environment;
mod rains_of_castamere;
mod relations;
mod serialization;

fn save_keys<Env: Environment>(rel_name: &str, keys: PureKeys<Env>)
where
    VerifyingKey<Env>: CanonicalSerialize,
    ProvingKey<Env>: CanonicalSerialize,
{
    let ser_keys = serialize_keys(&keys);

    fs::write(format!("{}.vk.bytes", rel_name), ser_keys.verifying_key).unwrap();
    fs::write(format!("{}.pk.bytes", rel_name), ser_keys.proving_key).unwrap();
}

fn save_proving_artifacts<Env: Environment>(rel_name: &str, artifacts: PureProvingArtifacts<Env>)
where
    Proof<Env>: CanonicalSerialize,
    Fr<Env>: CanonicalSerialize,
{
    let ser_artifacts = serialize_proving_artifacts(&artifacts);

    fs::write(format!("{}.proof.bytes", rel_name), ser_artifacts.proof).unwrap();
    fs::write(
        format!("{}.public_input.bytes", rel_name),
        ser_artifacts.public_input,
    )
    .unwrap();
}

fn generate_keys_for<Env: Environment>(relation: impl SnarkRelation<Env>)
where
    VerifyingKey<Env>: CanonicalSerialize,
    ProvingKey<Env>: CanonicalSerialize,
{
    let keys = relation.generate_keys();
    save_keys::<Env>(relation.id(), keys);
}

fn generate_proving_artifacts_for<Env: Environment>(
    relation: impl SnarkRelation<Env>,
    pk_file: PathBuf,
) where
    ProvingKey<Env>: CanonicalDeserialize,
    Proof<Env>: CanonicalSerialize,
    Fr<Env>: CanonicalSerialize,
{
    let pk_serialized = fs::read(pk_file).unwrap();
    let proving_key =
        <ProvingKey<Env> as CanonicalDeserialize>::deserialize(&*pk_serialized).unwrap();

    let artifacts = relation.generate_proof(proving_key);
    save_proving_artifacts::<Env>(relation.id(), artifacts);
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
        Command::GenerateKeys(GenerateKeysCmd { relation, .. }) => {
            generate_keys_for::<_>(relation.as_snark_relation::<GrothEnv>())
        }
        Command::GenerateProof(GenerateProofCmd {
            relation,
            proving_key_file,
            ..
        }) => generate_proving_artifacts_for::<_>(
            relation.as_snark_relation::<GrothEnv>(),
            proving_key_file,
        ),
        Command::RedWedding => red_wedding(),
    }
}
