
#[derive(Clone, Debug)]
pub struct Program(pub Vec<Statement>);


#[derive(Clone, Debug)]
pub enum Statement {
    Exit { value: Expr },
    Expr(Expr),
    Let { identifier: String, value: Expr },
}


#[derive(Clone, Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),

    IntegerLiteral(String),
    Identifier(String),
    If { check: Box<Expr>, body: Box<Statement>, els: Option<Box<Statement>> },
}

