use ark_serialize::CanonicalSerialize;

use crate::Artifacts;

pub type SerializedArtifacts = Artifacts<Vec<u8>, Vec<u8>, Vec<u8>>;

pub fn serialize_artifacts<
    VK: CanonicalSerialize,
    P: CanonicalSerialize,
    PI: CanonicalSerialize,
>(
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
