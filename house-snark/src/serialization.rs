use std::{fs, path::PathBuf};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

use crate::{Environment, Fr, Proof, ProvingKey, VerifyingKey};

fn serialize<T: CanonicalSerialize>(t: &T) -> Vec<u8> {
    let mut bytes = vec![0; t.serialized_size()];
    t.serialize(&mut bytes[..]).unwrap();
    bytes.to_vec()
}

pub fn save_keys<Env: Environment>(rel_name: String, pk: ProvingKey<Env>, vk: VerifyingKey<Env>)
where
    VerifyingKey<Env>: CanonicalSerialize,
    ProvingKey<Env>: CanonicalSerialize,
{
    let prefix = format!("{}.{}", rel_name, Env::id());
    fs::write(format!("{}.vk.bytes", prefix), serialize(&vk)).unwrap();
    fs::write(format!("{}.pk.bytes", prefix), serialize(&pk)).unwrap();
}

pub fn save_proving_artifacts<Env: Environment>(
    rel_name: String,
    proof: Proof<Env>,
    input: Vec<Fr<Env>>,
) where
    Proof<Env>: CanonicalSerialize,
    Fr<Env>: CanonicalSerialize,
{
    let prefix = format!("{}.{}", rel_name, Env::id());
    fs::write(format!("{}.proof.bytes", prefix), serialize(&proof)).unwrap();
    fs::write(format!("{}.public_input.bytes", prefix), serialize(&input)).unwrap();
}

pub fn read_proving_key<Env: Environment>(proving_key_file: PathBuf) -> ProvingKey<Env>
where
    ProvingKey<Env>: CanonicalDeserialize,
{
    let pk_serialized =
        fs::read(proving_key_file).expect("Cannot read proving key from the provided path");

    <ProvingKey<Env> as CanonicalDeserialize>::deserialize(&*pk_serialized)
        .expect("Cannot deserialize proving key")
}
