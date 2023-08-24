mod ast;
mod cli;
mod parser;
mod tokenize;

#[cfg(test)]
mod test;

use std::{
    fs::File,
    io::{Read, Write},
};

use clap::Parser;

use crate::cli::Cli;


trait RunCommand {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

impl RunCommand for std::process::Command {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let output = self.output()?;
        if !output.status.success() {
            eprintln!("assembly failed with code {}", output.status);
            eprintln!("{}", String::from_utf8(output.stderr)?);
            eprintln!("{}", String::from_utf8(output.stdout)?);
            panic!();
        }
        Ok(())
    }
}


fn compile(args: &cli::CompileArgs) -> Result<(), Box<dyn std::error::Error>> {
    use crate::{
        parser::Parser,
        tokenize::Tokenize,
    };

    println!("Compiling {:?}", args.source_file);

    let source_code = {
        let mut buffer = String::new();
        File::open(&args.source_file)?.read_to_string(&mut buffer)?;
        buffer
    };

    println!("    lexing");
    let tokens = source_code.tokenize();

    println!("    parsing");
    let mut parser = Parser::new(tokens);
    let tree = parser.parse()?;

    println!("    codegen");
    let generated_code = tree.codegen().unwrap();

    println!("    writing");
    File::create(args.get_target_file())?.write_all(generated_code.as_bytes())?;

    println!("    assembling");
    use std::process::Command;
    let mut command = Command::new("nasm");
    command.arg("-felf64")
           .arg(&args.source_file.with_extension("asm"));
    println!("        running `{:?}`", command);
    command.run()?;
    
    println!("    linking");
    let mut command = Command::new("ld");
    command.arg(args.source_file.with_extension("o"))
           .arg("-o").arg(args.source_file.with_extension(""));
    println!("        running `{:?}`", command);
    command.run()?;

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::cli::Command;

    let cli = Cli::parse();
    match &cli.command {
        Command::Compile(args) => compile(args),
    }
}

