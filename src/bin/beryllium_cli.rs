
use std::path::PathBuf;

use clap::{Args, Parser as ArgParser, Subcommand};


#[derive(ArgParser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Compile(CompileArgs),
}

#[derive(Args)]
pub struct CompileArgs {
    source_file: PathBuf,
    target_file: Option<PathBuf>,
}

impl From<CompileArgs> for beryllium::CompileArgs {
    fn from(value: CompileArgs) -> Self {
        Self {
            source_file: value.source_file,
            target_file: value.target_file,
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Cli::parse();
    match command.command {
        Command::Compile(args) => beryllium::compile(&args.into())?,
    };
    Ok(())
}

