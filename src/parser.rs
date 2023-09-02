use std::collections::VecDeque;

use fallible_iterator::FallibleIterator;

use crate::{
    tokenize::{
        Keyword, Symbol,
        TokenStream, Token, TokenData,
        TokenizerError,
    },
    ast::{
        Expr,
        Program, Statement,
    },
};


#[derive(Clone, Debug)]
pub enum ParseError {
    TokenizerError(TokenizerError),
    UnexpectedToken(Token),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {}

impl From<TokenizerError> for ParseError {
    fn from(value: TokenizerError) -> Self {
        Self::TokenizerError(value)
    }
}


pub struct Parser {
    tokens: TokenStream,
    buffer: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens, buffer: VecDeque::new() }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Vec::new();
        while !self.is_empty()? {
            program.push(self.parse_statement()?);
        }
        Ok(Program(program))
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.peek()?.expect("a token") {
            Token { data: TokenData::Keyword(kwd), location } => match kwd {
                Keyword::Exit => {
                    self.consume()?;
                    match self.consume()?.expect("a left parenthesis") {
                        Token { data: TokenData::Symbol(Symbol::LParen), location: _ } => (),
                        tok => return Err(ParseError::UnexpectedToken(tok))
                    };
                    let value = self.parse_expression()?;
                    match self.consume()?.expect("a right parenthesis") {
                        Token { data: TokenData::Symbol(Symbol::RParen), location: _ } => (),
                        tok => return Err(ParseError::UnexpectedToken(tok))
                    };
                    match self.consume()?.expect("a semicolon") {
                        Token { data: TokenData::Symbol(Symbol::Semi), location: _ } => (),
                        tok => return Err(ParseError::UnexpectedToken(tok))
                    };
                    Ok(Statement::Exit { value })
                },
                Keyword::Let => {
                    self.consume()?;
                    let identifier = match self.consume()?.expect("an identifier") {
                        Token { data: TokenData::Identifier(identifier), location: _ } => identifier,
                        tok => return Err(ParseError::UnexpectedToken(tok)),
                    };
                    match self.consume()?.expect("an equals sign") {
                        Token { data: TokenData::Symbol(Symbol::Equals), location: _ } => (),
                        tok => return Err(ParseError::UnexpectedToken(tok))
                    };
                    let value = self.parse_expression()?;
                    match self.consume()?.expect("a semicolon") {
                        Token { data: TokenData::Symbol(Symbol::Semi), location: _ } => (),
                        tok => return Err(ParseError::UnexpectedToken(tok))
                    };
                    Ok(Statement::Let { identifier, value })
                },
                Keyword::If => self.parse_if().map(Statement::Expr),
                kwd => Err(ParseError::UnexpectedToken(Token { data: TokenData::Keyword(kwd), location })),
            },
            Token {
                data: TokenData::Symbol(Symbol::LBrace),
                location: _
            } => self.parse_block().map(Statement::Expr),
            tok => Err(ParseError::UnexpectedToken(tok)),
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_expression_cmp_part()
    }

    fn parse_expression_cmp_part(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_expression_add_part()?;
        if let Some(Token { data, location: _ }) = self.peek()? {
            match data {
                TokenData::Symbol(Symbol::Equality) => {
                    self.consume()?;
                    expr = Expr::Equality(
                        Box::new(expr),
                        Box::new(self.parse_expression_add_part()?)
                    );
                },
                TokenData::Symbol(Symbol::NonEquality) => {
                    self.consume()?;
                    expr = Expr::NonEquality(
                        Box::new(expr),
                        Box::new(self.parse_expression_add_part()?)
                    );
                },
                TokenData::Symbol(Symbol::LAngle) => {
                    self.consume()?;
                    expr = Expr::Less(
                        Box::new(expr),
                        Box::new(self.parse_expression_add_part()?)
                    );
                },
                TokenData::Symbol(Symbol::LesserEqual) => {
                    self.consume()?;
                    expr = Expr::LessEq(
                        Box::new(expr),
                        Box::new(self.parse_expression_add_part()?)
                    );
                },
                TokenData::Symbol(Symbol::RAngle) => {
                    self.consume()?;
                    expr = Expr::Greater(
                        Box::new(expr),
                        Box::new(self.parse_expression_add_part()?)
                    );
                },
                TokenData::Symbol(Symbol::GreaterEqual) => {
                    self.consume()?;
                    expr = Expr::GreaterEq(
                        Box::new(expr),
                        Box::new(self.parse_expression_add_part()?)
                    );
                },
                _ => (),
            }
        }
        Ok(expr)
    }

    fn parse_expression_add_part(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_expression_mul_part()?;
        if let Some(Token { data, location: _ }) = self.peek()? {
            match data {
                TokenData::Symbol(Symbol::Plus) => {
                    self.consume()?;
                    expr = Expr::Add(
                        Box::new(expr),
                        Box::new(self.parse_expression_mul_part()?)
                    );
                },
                TokenData::Symbol(Symbol::Minus) => {
                    self.consume()?;
                    expr = Expr::Sub(
                        Box::new(expr),
                        Box::new(self.parse_expression_mul_part()?)
                    );
                },
                _ => (),
            }
        }
        Ok(expr)
    }

    fn parse_expression_mul_part(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_atom()?;
        if let Some(Token { data, location: _ }) = self.peek()? {
            match data {
                TokenData::Symbol(Symbol::Star) => {
                    self.consume()?;
                    expr = Expr::Mul(
                        Box::new(expr),
                        Box::new(self.parse_atom()?)
                    );
                },
                TokenData::Symbol(Symbol::Slash) => {
                    self.consume()?;
                    expr = Expr::Div(
                        Box::new(expr),
                        Box::new(self.parse_atom()?)
                    );
                },
                TokenData::Symbol(Symbol::Percent) => {
                    self.consume()?;
                    expr = Expr::Mod(
                        Box::new(expr),
                        Box::new(self.parse_atom()?)
                    );
                },
                _ => (),
            }
        }
        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        match self.peek()?.expect("a token") {
            Token { data: TokenData::IntegerLiteral(lit), location: _ } => { self.consume()?; Ok(Expr::IntegerLiteral(lit)) },
            Token { data: TokenData::Identifier(ident), location: _ } => { self.consume()?; Ok(Expr::Identifier(ident)) }

            Token { data: TokenData::Symbol(Symbol::LBrace), location: _ } => self.parse_block(),
            Token { data: TokenData::Keyword(Keyword::If), location: _ } => self.parse_if(),
            tok => Err(ParseError::UnexpectedToken(tok)),
        }
    }

    fn parse_block(&mut self) -> Result<Expr, ParseError> {
        match self.consume()?.expect("a left brace `{`") {
            Token { data: TokenData::Symbol(Symbol::LBrace), location: _ } => (),
            tok => return Err(ParseError::UnexpectedToken(tok)),
        };
        let mut stmts = Vec::new();
        loop {
            match self.peek()?.expect("a statement or right brace `}`") {
                Token { data: TokenData::Symbol(Symbol::RBrace), location: _ } => { self.consume()?; break },
                _ => stmts.push(self.parse_statement()?),
            }
        };
        Ok(Expr::Block(stmts))
    }

    fn parse_if(&mut self) -> Result<Expr, ParseError> {
        match self.consume()?.expect("keyword `if`") {
            Token { data: TokenData::Keyword(Keyword::If), location: _ } => (),
            tok => return Err(ParseError::UnexpectedToken(tok)),
        }
        match self.consume()?.expect("a left parenthesis") {
            Token { data: TokenData::Symbol(Symbol::LParen), location: _ } => (),
            tok => return Err(ParseError::UnexpectedToken(tok))
        };
        let check = Box::new(self.parse_expression()?);
        match self.consume()?.expect("a right parenthesis") {
            Token { data: TokenData::Symbol(Symbol::RParen), location: _ } => (),
            tok => return Err(ParseError::UnexpectedToken(tok))
        };
        let body = Box::new(self.parse_statement()?);
        let els = match self.peek()? {
            Some(Token { data: TokenData::Keyword(Keyword::Else), location: _ }) => {
                self.consume()?;
                Some(Box::new(self.parse_statement()?))
            },
            Some(_) | None => None,
        };
        Ok(Expr::If { check, body, els })
    }

    fn is_empty(&mut self) -> Result<bool, TokenizerError> {
        Ok(self.peek()?.is_none())
    }

    fn peek(&mut self) -> Result<Option<Token>, TokenizerError> {
        if self.buffer.is_empty() {
            match self.tokens.next()? {
                Some(token) => self.buffer.push_back(token),
                None => return Ok(None),
            };
        };
        Ok(self.buffer.get(0).cloned())
    }

    fn consume(&mut self) -> Result<Option<Token>, TokenizerError> {
        if self.buffer.is_empty() {
            self.tokens.next()
        } else {
            Ok(self.buffer.pop_front())
        }
    }
}

