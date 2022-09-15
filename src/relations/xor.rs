use crate::relations::{Artifacts, SnarkRelation};

pub struct XorRelation;

impl Default for XorRelation {
    fn default() -> Self {
        XorRelation
    }
}

impl SnarkRelation for XorRelation {
    fn id() -> &'static str {
        "xor"
    }

    fn generate_artifacts(&self) -> Artifacts {
        Artifacts {
            verifying_key: "vk".to_string().into_bytes(),
            proof: "proof".into(),
            public_input: "input".into(),
        }
    }
}
