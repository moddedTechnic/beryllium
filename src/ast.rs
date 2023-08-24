use crate::context::Context;


#[derive(Clone, Debug)]
pub enum CodegenError {
    IdentifierNotDeclared(String),
}

impl std::fmt::Display for CodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self:?}")
    }
}

impl std::error::Error for CodegenError {}


type Result = std::result::Result<String, CodegenError>;


pub trait Codegen {
    fn codegen(self, context: &mut Context) -> Result;
}


#[derive(Clone, Debug)]
pub struct Program(pub Vec<Statement>);

impl Codegen for Program {
    fn codegen(self, context: &mut Context) -> Result {
        let mut code = String::from("global _start\n_start:\n");
        for stmt in self.0 {
            code.push_str(stmt.codegen(context)?.as_str());
        }
        code.push_str("    mov rax, 60\n");
        code.push_str("    mov rdi, 0\n");
        code.push_str("    syscall\n");
        Ok(code)
    }
}


#[derive(Clone, Debug)]
pub enum Statement {
    Exit { value: Expr },
    Let { identifier: String, value: Expr },
}

impl Codegen for Statement {
    fn codegen(self, context: &mut Context) -> Result {
        match self {
            Self::Exit { value } => {
                let mut code = value.codegen(context)?;
                code.push_str("    mov rax, 60\n");
                code.push_str(context.pop("rdi").as_str());
                code.push_str("    syscall\n");
                Ok(code)
            },
            Self::Let { identifier, value } => {
                let code = value.codegen(context);
                context.declare_variable(identifier);
                code
            }
        }
    }
}


#[derive(Clone, Debug)]
pub enum Expr {
    Atom(Atom),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

impl Codegen for Expr {
    fn codegen(self, context: &mut Context) -> Result {
        match self {
            Self::Atom(atom) => atom.codegen(context),
            Self::Add(a, b) => {
                let mut code = String::new();
                code.push_str(a.codegen(context)?.as_str());
                code.push_str(b.codegen(context)?.as_str());
                code.push_str(context.pop("rbx").as_str());
                code.push_str(context.pop("rax").as_str());
                code.push_str("    add rax, rbx\n");
                code.push_str(context.push("rax").as_str());
                Ok(code)
            },
            Self::Sub(a, b) => {
                let mut code = String::new();
                code.push_str(a.codegen(context)?.as_str());
                code.push_str(b.codegen(context)?.as_str());
                code.push_str(context.pop("rbx").as_str());
                code.push_str(context.pop("rax").as_str());
                code.push_str("    sub rax, rbx\n");
                code.push_str(context.push("rax").as_str());
                Ok(code)
            },
        }
    }
}


#[derive(Clone, Debug)]
pub enum Atom {
    IntegerLiteral(String),
    Identifier(String),
}

impl Codegen for Atom {
    fn codegen(self, context: &mut Context) -> Result {
        match self {
            Self::IntegerLiteral(value) => Ok(context.push(value)),
            Self::Identifier(ident) => Ok(
                context.get_variable(&ident)
                    .ok_or(CodegenError::IdentifierNotDeclared(ident))?
            ),
        }
    }
}

