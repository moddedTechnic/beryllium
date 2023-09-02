
#[derive(Clone, Debug)]
pub struct Program(pub Vec<Statement>);


#[derive(Clone, Debug)]
pub enum Statement {
    Exit { value: Expr },
    Expr(Expr),
    Let { identifier: String, value: Expr, is_mutable: bool },
}


#[derive(Clone, Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),

    AddAssign { identifier: String, value: Box<Expr> },
    SubAssign { identifier: String, value: Box<Expr> },
    MulAssign { identifier: String, value: Box<Expr> },
    DivAssign { identifier: String, value: Box<Expr> },
    ModAssign { identifier: String, value: Box<Expr> },

    Equality(Box<Expr>, Box<Expr>),
    NonEquality(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    LessEq(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    GreaterEq(Box<Expr>, Box<Expr>),

    IntegerLiteral(String),
    Identifier(String),

    Block(Vec<Statement>),
    If { check: Box<Expr>, body: Box<Statement>, els: Option<Box<Statement>> },
}

