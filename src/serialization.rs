use ark_serialize::CanonicalSerialize;

use crate::relations::{Keys, ProvingArtifacts};

pub type SerializedKeys = Keys<Vec<u8>, Vec<u8>>;
pub type SerializedProvingArtifacts = ProvingArtifacts<Vec<u8>, Vec<u8>>;

fn serialize<T: CanonicalSerialize>(t: &T) -> Vec<u8> {
    let mut bytes = vec![0; t.serialized_size()];
    t.serialize(&mut bytes[..]).unwrap();
    bytes.to_vec()
}

pub fn serialize_keys<VK: CanonicalSerialize, PK: CanonicalSerialize>(
    keys: &Keys<VK, PK>,
) -> SerializedKeys {
    SerializedKeys {
        verifying_key: serialize(&keys.verifying_key),
        proving_key: serialize(&keys.proving_key),
    }
}

pub fn serialize_proving_artifacts<P: CanonicalSerialize, PI: CanonicalSerialize>(
    artifacts: &ProvingArtifacts<P, PI>,
) -> SerializedProvingArtifacts {
    SerializedProvingArtifacts {
        proof: serialize(&artifacts.proof),
        public_input: serialize(&artifacts.public_input),
    }
}
