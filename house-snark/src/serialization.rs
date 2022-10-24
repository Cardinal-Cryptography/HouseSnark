use std::{fs, path::PathBuf};

use ark_serialize::CanonicalSerialize;

pub fn serialize<T: CanonicalSerialize>(t: &T) -> Vec<u8> {
    let mut bytes = vec![0; t.serialized_size()];
    t.serialize(&mut bytes[..]).expect("Failed to serialize");
    bytes.to_vec()
}

fn save_bytes(bytes: Vec<u8>, prefix: &str, identifier: &str) {
    let path = format!("{}.{}.bytes", prefix, identifier);
    fs::write(path, bytes).unwrap_or_else(|_| panic!("Failed to save {}", identifier));
}

pub fn save_srs(srs: Vec<u8>, env_id: String) {
    save_bytes(srs, &env_id, "srs");
}

pub fn save_keys(rel_name: String, env_id: String, pk: Vec<u8>, vk: Vec<u8>) {
    let prefix = format!("{}.{}", rel_name, env_id);
    save_bytes(pk, &prefix, "pk");
    save_bytes(vk, &prefix, "vk");
}

pub fn save_proving_artifacts(rel_name: String, env_id: String, proof: Vec<u8>, input: Vec<u8>) {
    let prefix = format!("{}.{}", rel_name, env_id);
    save_bytes(proof, &prefix, "proof");
    save_bytes(input, &prefix, "public_input");
}

pub fn read_srs(srs_file: PathBuf) -> Vec<u8> {
    fs::read(srs_file).expect("Cannot read SRS from the provided path")
}

pub fn read_proving_key(proving_key_file: PathBuf) -> Vec<u8> {
    fs::read(proving_key_file).expect("Cannot read proving key from the provided path")
}
