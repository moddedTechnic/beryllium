use crate::{
    ast::*,
    context::Context,
};
use super::{
    CodegenError,
    Result,
};


pub trait Codegen {
    fn codegen_x86(self, context: &mut Context) -> Result;
}


impl Codegen for Program {
    fn codegen_x86(self, context: &mut Context) -> Result {
        let mut code = String::from("global _start\n_start:\n");
        for stmt in self.0 {
            code.push_str(stmt.codegen_x86(context)?.as_str());
        }
        code.push_str("    mov rax, 60\n");
        code.push_str("    mov rdi, 0\n");
        code.push_str("    syscall\n");
        Ok(code)
    }
}


impl Codegen for Statement {
    fn codegen_x86(self, context: &mut Context) -> Result {
        match self {
            Self::Exit { value } => {
                let mut code = value.codegen_x86(context)?;
                code.push_str("    mov rax, 60\n");
                code.push_str(context.pop("rdi").as_str());
                code.push_str("    syscall\n");
                Ok(code)
            },
            Self::Expr(value) => value.codegen_x86(context),
            Self::Let { identifier, value } => {
                let code = value.codegen_x86(context);
                context.declare_variable(identifier);
                code
            },
        }
    }
}


impl Expr {
    fn prepare_binop_registers(context: &mut Context, a: Expr, b: Expr) -> Result {
        let mut code = String::new();
        code.push_str(a.codegen_x86(context)?.as_str());
        code.push_str(b.codegen_x86(context)?.as_str());
        code.push_str(context.pop("rbx").as_str());
        code.push_str(context.pop("rax").as_str());
        Ok(code)
    }
}

impl Codegen for Expr {
    fn codegen_x86(self, context: &mut Context) -> Result {
        match self {
            Self::Add(a, b) => {
                let mut code = Self::prepare_binop_registers(context, *a, *b)?;
                code.push_str("    add rax, rbx\n");
                code.push_str(context.push("rax").as_str());
                Ok(code)
            },
            Self::Sub(a, b) => {
                let mut code = Self::prepare_binop_registers(context, *a, *b)?;
                code.push_str("    sub rax, rbx\n");
                code.push_str(context.push("rax").as_str());
                Ok(code)
            },
            Self::Mul(a, b) => {
                let mut code = Self::prepare_binop_registers(context, *a, *b)?;
                code.push_str("    mul rbx\n");
                code.push_str(context.push("rax").as_str());
                Ok(code)
            },
            Self::Div(a, b) => {
                let mut code = Self::prepare_binop_registers(context, *a, *b)?;
                code.push_str("    div rbx\n");
                code.push_str(context.push("rax").as_str());
                Ok(code)
            },
            Self::Mod(a, b) => {
                let mut code = Self::prepare_binop_registers(context, *a, *b)?;
                code.push_str("    div rbx\n");
                code.push_str(context.push("rdx").as_str());
                Ok(code)
            },

            Self::IntegerLiteral(value) => Ok(context.push(value)),
            Self::Identifier(ident) => Ok(
                context.get_variable(&ident)
                    .ok_or(CodegenError::IdentifierNotDeclared(ident))?
            ),
            Self::If { check, body, els } => {
                let else_label = context.create_label("else");
                let endif_label = context.create_label("endif");

                let mut code = check.codegen_x86(context)?;
                code += context.pop("rax").as_str();
                code += "    or rax, rax\n";
                code += format!("    jz {else_label}\n").as_str();
                code += body.codegen_x86(context)?.as_str();
                code += format!("    jmp {endif_label}\n").as_str();
                code += format!("{else_label}:\n").as_str();
                if let Some(els) = els {
                    code += els.codegen_x86(context)?.as_str();
                }
                code += format!("{endif_label}:\n").as_str();
                Ok(code)
            },
        }
    }
}

