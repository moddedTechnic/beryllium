mod ast;
mod codegen;
mod context;
mod iter;
mod parser;
mod tokenize;
mod type_registry;

use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use crate::{
    codegen::CodegenError,
    parser::ParseError,
    tokenize::{Token, TokenizerError},
    type_registry::TypeRegistry,
};

use crate::context::Context;


trait RunCommand {
    fn run(&mut self) -> Result<(), CompileError>;
}

impl RunCommand for std::process::Command {
    fn run(&mut self) -> Result<(), CompileError> {
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


#[derive(Debug)]
pub enum CompileError {
    IdentifierNotDeclared(String),
    FunctionNotDeclared(String),
    ChangedImmutableVariable(String),
    UnexpectedToken(Token),
    UnrecognizedCharacter(char),
    IOError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self:?}")
    }
}

impl std::error::Error for CompileError {}

impl From<CodegenError> for CompileError {
    fn from(value: CodegenError) -> Self {
        match value {
            CodegenError::IdentifierNotDeclared(ident) => Self::IdentifierNotDeclared(ident),
            CodegenError::ChangedImmutableVariable(ident) => Self::ChangedImmutableVariable(ident),
            CodegenError::FunctionNotDeclared(ident) => Self::FunctionNotDeclared(ident),
        }
    }
}

impl From<ParseError> for CompileError {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::UnexpectedToken(tok) => Self::UnexpectedToken(tok),
            ParseError::TokenizerError(err) => err.into(),
        }
    }
}

impl From<TokenizerError> for CompileError {
    fn from(value: TokenizerError) -> Self {
        match value {
            TokenizerError::UnrecognizedCharacter(c) => Self::UnrecognizedCharacter(c),
        }
    }
}

impl From<std::io::Error> for CompileError {
    fn from(value: std::io::Error) -> Self {
        CompileError::IOError(value)
    }
}

impl From<std::string::FromUtf8Error> for CompileError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        CompileError::FromUtf8Error(value)
    }
}


pub fn compile(args: &CompileArgs) -> Result<(), CompileError> {
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

    println!("    registering types");
    let type_checker = TypeRegistry::from(&tree);

    println!("    codegen");
    use crate::codegen::x86::Codegen;
    let mut context = Context::new(type_checker);
    let generated_code = tree.codegen_x86(&mut context)?;

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

