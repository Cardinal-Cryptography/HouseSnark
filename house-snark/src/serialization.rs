use std::{fs, path::PathBuf};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

use crate::{Environment, Fr, Proof, ProvingKey, VerifyingKey};

pub fn serialize<T: CanonicalSerialize>(t: &T) -> Vec<u8> {
    let mut bytes = vec![0; t.serialized_size()];
    t.serialize(&mut bytes[..]).expect("Failed to serialize");
    bytes.to_vec()
}

fn save_bytes(bytes: Vec<u8>, prefix: &str, identifier: &str) {
    let path = format!("{}.{}.bytes", prefix, identifier);
    fs::write(path, bytes).unwrap_or_else(|_| panic!("Failed to save {}", identifier));
}

pub fn save_keys<Env: Environment>(rel_name: String, pk: ProvingKey<Env>, vk: VerifyingKey<Env>)
where
    VerifyingKey<Env>: CanonicalSerialize,
    ProvingKey<Env>: CanonicalSerialize,
{
    let prefix = format!("{}.{}", rel_name, Env::id());
    save_bytes(serialize(&vk), &prefix, "vk");
    save_bytes(serialize(&pk), &prefix, "pk");
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
    save_bytes(serialize(&proof), &prefix, "proof");
    save_bytes(serialize(&input), &prefix, "public_input");
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
