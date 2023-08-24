use std::path::PathBuf;

use clap::{Args, Parser as ArgParser, Subcommand};


#[derive(ArgParser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Compile(CompileArgs),
}

#[derive(Args)]
pub struct CompileArgs {
    pub source_file: PathBuf,
    target_file: Option<PathBuf>,
}

impl CompileArgs {
    pub fn get_target_file(&self) -> PathBuf {
        match &self.target_file {
            Some(target_file) => target_file.clone(),
            None => self.source_file.with_extension("asm"),
        }
    }
}

