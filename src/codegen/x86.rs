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
            Self::Let { identifier, value, is_mutable } => {
                let code = value.codegen_x86(context);
                context.declare_variable(identifier, is_mutable);
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

            Self::AddAssign { identifier, value } => {
                let mut code = String::new();
                code += value.codegen_x86(context)?.as_str();
                code += context.get_variable(&identifier)
                    .ok_or(CodegenError::IdentifierNotDeclared(identifier.clone()))?
                    .as_str();
                code += context.pop("rax").as_str();
                code += context.pop("rbx").as_str();
                code += "    add rax, rbx\n";
                code += context.set_variable(&identifier, "rax")?
                    .as_str();
                Ok(code)
            },
            Self::SubAssign { identifier, value } => {
                let mut code = String::new();
                code += value.codegen_x86(context)?.as_str();
                code += context.get_variable(&identifier)
                    .ok_or(CodegenError::IdentifierNotDeclared(identifier.clone()))?
                    .as_str();
                code += context.pop("rax").as_str();
                code += context.pop("rbx").as_str();
                code += "    sub rax, rbx\n";
                code += context.set_variable(&identifier, "rax")?
                    .as_str();
                Ok(code)
            },
            Self::MulAssign { identifier, value } => {
                let mut code = String::new();
                code += value.codegen_x86(context)?.as_str();
                code += context.get_variable(&identifier)
                    .ok_or(CodegenError::IdentifierNotDeclared(identifier.clone()))?
                    .as_str();
                code += context.pop("rax").as_str();
                code += context.pop("rbx").as_str();
                code += "    mul rbx\n";
                code += context.set_variable(&identifier, "rax")?
                    .as_str();
                Ok(code)
            },
            Self::DivAssign { identifier, value } => {
                let mut code = String::new();
                code += value.codegen_x86(context)?.as_str();
                code += context.get_variable(&identifier)
                    .ok_or(CodegenError::IdentifierNotDeclared(identifier.clone()))?
                    .as_str();
                code += context.pop("rax").as_str();
                code += context.pop("rbx").as_str();
                code += "    div rbx\n";
                code += context.set_variable(&identifier, "rax")?
                    .as_str();
                Ok(code)
            },
            Self::ModAssign { identifier, value } => {
                let mut code = String::new();
                code += value.codegen_x86(context)?.as_str();
                code += context.get_variable(&identifier)
                    .ok_or(CodegenError::IdentifierNotDeclared(identifier.clone()))?
                    .as_str();
                code += context.pop("rax").as_str();
                code += context.pop("rbx").as_str();
                code += "    div rbx\n";
                code += context.set_variable(&identifier, "rdx")?
                    .as_str();
                Ok(code)
            },

            Self::Equality(a, b) => {
                let mut code = Self::prepare_binop_registers(context, *a, *b)?;
                code += "    mov rcx, 0\n";
                code += "    cmp rax, rbx\n";
                code += "    sete cl\n";
                code += context.push("rcx").as_str();
                Ok(code)
            },
            Self::NonEquality(a, b) => {
                let mut code = Self::prepare_binop_registers(context, *a, *b)?;
                code += "    mov rcx, 0\n";
                code += "    cmp rax, rbx\n";
                code += "    setne cl\n";
                code += context.push("rcx").as_str();
                Ok(code)
            },
            Self::Less(a, b) => {
                let mut code = Self::prepare_binop_registers(context, *a, *b)?;
                code += "    mov rcx, 0\n";
                code += "    cmp rax, rbx\n";
                code += "    setl cl\n";
                code += context.push("rcx").as_str();
                Ok(code)
            },
            Self::LessEq(a, b) => {
                let mut code = Self::prepare_binop_registers(context, *a, *b)?;
                code += "    mov rcx, 0\n";
                code += "    cmp rax, rbx\n";
                code += "    setle cl\n";
                code += context.push("rcx").as_str();
                Ok(code)
            },
            Self::Greater(a, b) => {
                let mut code = Self::prepare_binop_registers(context, *a, *b)?;
                code += "    mov rcx, 0\n";
                code += "    cmp rax, rbx\n";
                code += "    setg cl\n";
                code += context.push("rcx").as_str();
                Ok(code)
            },
            Self::GreaterEq(a, b) => {
                let mut code = Self::prepare_binop_registers(context, *a, *b)?;
                code += "    mov rcx, 0\n";
                code += "    cmp rax, rbx\n";
                code += "    setge cl\n";
                code += context.push("rcx").as_str();
                Ok(code)
            },

            Self::IntegerLiteral(value) => Ok(context.push(value)),
            Self::Identifier(ident) => Ok(
                context.get_variable(&ident)
                    .ok_or(CodegenError::IdentifierNotDeclared(ident))?
            ),

            Self::Block(stmts) => {
                let mut code = context.enter();
                code += stmts
                    .into_iter()
                    .map(|stmt| stmt.codegen_x86(context))
                    .reduce(|a, b| Ok(a? + &b?))
                    .unwrap_or(Ok(String::new()))?
                    .as_str();
                code += context.exit().as_str();
                Ok(code)
            }
            Self::If { check, body, els } => {
                let if_label = context.create_label("if");
                let else_label = context.create_label("else");
                let endif_label = context.create_label("endif");

                let mut code = format!("{if_label}:\n");
                code += check.codegen_x86(context)?.as_str();
                code += context.pop("rax").as_str();
                code += "    or rax, rax\n";
                code += format!("    jz {else_label}\n").as_str();
                code += context.enter().as_str();
                code += body.codegen_x86(context)?.as_str();
                code += context.exit().as_str();
                code += format!("    jmp {endif_label}\n").as_str();
                code += format!("{else_label}:\n").as_str();
                if let Some(els) = els {
                    code += context.enter().as_str();
                    code += els.codegen_x86(context)?.as_str();
                    code += context.exit().as_str();
                }
                code += format!("{endif_label}:\n").as_str();
                Ok(code)
            },
            Self::While { check, body } => {
                let while_label = context.create_label("while");
                let endwhile_label = context.create_label("endwhile");

                let mut check_code = check.codegen_x86(context)?;
                check_code += context.pop("rax").as_str();
                check_code += "    or rax, rax\n";

                let mut code = check_code.clone();
                code += format!("    jz {endwhile_label}\n").as_str();
                code += format!("{while_label}:\n").as_str();
                code += body.codegen_x86(context)?.as_str();
                code += check_code.as_str();
                code += format!("    jnz {while_label}\n").as_str();
                code += format!("{endwhile_label}:\n").as_str();
                Ok(code)
            }
        }
    }
}

