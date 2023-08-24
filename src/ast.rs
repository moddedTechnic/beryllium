use crate::context::Context;


#[derive(Clone, Debug)]
pub struct Program(pub Vec<Statement>);

impl Program {
    pub fn codegen(self, context: &mut Context) -> Result<String, ()> {
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

impl Statement {
    fn codegen(self, context: &mut Context) -> Result<String, ()> {
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
    IntegerLiteral(String),
    Identifier(String),
}

impl Expr {
    fn codegen(self, context: &mut Context) -> Result<String, ()> {
        match self {
            Self::IntegerLiteral(value) => Ok(context.push(value)),
            Self::Identifier(ident) => Ok(context.get_variable(ident).ok_or(())?),
        }
    }
}

