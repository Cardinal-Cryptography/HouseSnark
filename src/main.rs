use clap::Parser;

use crate::config::{Cli, Command, GenerateCmd};

mod config;
mod relations;

fn main() {
    let cli: Cli = Cli::parse();
    match cli.command {
        Command::Generate(GenerateCmd { relation }) => println!("generate {:?}", relation),
        Command::RedWedding => println!("red wedding"),
    }
}
