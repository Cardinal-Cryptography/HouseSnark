mod config;

use clap::Parser;

use crate::config::CliConfig;

// The binary is supposed to be compiled from the root crate directory.
#[subxt::subxt(runtime_metadata_path = "artifacts/aleph_metadata.scale")]
pub mod aleph {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: CliConfig = CliConfig::parse();
    println!("{:?}", config);
    Ok(())
}
