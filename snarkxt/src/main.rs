use subxt::config::Config;

#[subxt::subxt(runtime_metadata_path = "../artifacts/aleph_metadata.scale")]
pub mod aleph {}

#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
