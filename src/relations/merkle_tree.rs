use crate::relations::{Artifacts, SnarkRelation};

pub struct MerkleTreeRelation;

impl Default for MerkleTreeRelation {
    fn default() -> Self {
        MerkleTreeRelation
    }
}

impl SnarkRelation for MerkleTreeRelation {
    fn id() -> &'static str {
        "merkle-tree"
    }

    fn generate_artifacts(&self) -> Artifacts {
        Artifacts {
            verifying_key: "vk".to_string().into_bytes(),
            proof: "proof".into(),
            public_input: "input".into(),
        }
    }
}
