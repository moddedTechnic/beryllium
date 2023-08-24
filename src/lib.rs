mod ast;
mod context;
mod iter;
mod parser;
mod tokenize;

use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use crate::context::Context;


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


#[derive(Clone, Debug)]
pub struct CompileArgs {
    pub source_file: PathBuf,
    pub target_file: Option<PathBuf>,
}

impl CompileArgs {
    pub fn get_target_file(&self) -> PathBuf {
        match &self.target_file {
            Some(target_file) => target_file.clone(),
            None => self.source_file.with_extension(""),
        }
    }
}


pub fn compile(args: &CompileArgs) -> Result<(), Box<dyn std::error::Error>> {
    use crate::{
        parser::Parser,
        tokenize::Tokenize,
    };

    println!("Compiling {:?}", args.source_file);

    let source_code = {
        let mut buffer = String::new();
        File::open(&args.source_file)?
            .read_to_string(&mut buffer)?;
        buffer
    };

    println!("    lexing");
    let tokens = source_code.tokenize();

    println!("    parsing");
    let mut parser = Parser::new(tokens);
    let tree = parser.parse()?;

    println!("    codegen");
    use crate::ast::Codegen;
    let mut context = Context::new();
    let generated_code = tree.codegen(&mut context)?;

    println!("    writing");
    let target_file = args.get_target_file();
    File::create(target_file.with_extension("asm"))?
        .write_all(generated_code.as_bytes())?;

    println!("    assembling");
    use std::process::Command;
    let mut command = Command::new("nasm");
    command.arg("-felf64")
           .arg(target_file.with_extension("asm"));
    println!("        running `{:?}`", command);
    command.run()?;
    
    println!("    linking");
    let mut command = Command::new("ld");
    command.arg(target_file.with_extension("o"))
           .arg("-o").arg(target_file);
    println!("        running `{:?}`", command);
    command.run()?;

    Ok(())
}

