pub mod x86;


#[derive(Clone, Debug)]
pub enum CodegenError {
    IdentifierNotDeclared(String),
    ChangedImmutableVariable(String),
}

impl std::fmt::Display for CodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self:?}")
    }
}

impl std::error::Error for CodegenError {}


type Result = std::result::Result<String, CodegenError>;


