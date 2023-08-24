use std::collections::VecDeque;

use fallible_iterator::FallibleIterator;

use crate::{tokenize::{Keyword, Symbol, TokenStream, Token, TokenizerError}, ast::{Program, Statement, Expr}};


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
            Token::Keyword(kwd) => match kwd {
                Keyword::Exit => {
                    self.consume()?;
                    match self.consume()?.expect("a left parenthesis") {
                        Token::Symbol(lparen) => match lparen {
                            Symbol::LParen => (),
                            sym => return Err(ParseError::UnexpectedToken(Token::Symbol(sym))),
                        },
                        tok => return Err(ParseError::UnexpectedToken(tok))
                    };
                    let value = self.parse_expression()?;
                    match self.consume()?.expect("a right parenthesis") {
                        Token::Symbol(rparen) => match rparen {
                            Symbol::RParen => (),
                            sym => return Err(ParseError::UnexpectedToken(Token::Symbol(sym))),
                        },
                        tok => return Err(ParseError::UnexpectedToken(tok))
                    };
                    match self.consume()?.expect("a semicolon") {
                        Token::Symbol(semi) => match semi {
                            Symbol::Semi => (),
                            sym => return Err(ParseError::UnexpectedToken(Token::Symbol(sym))),
                        },
                        tok => return Err(ParseError::UnexpectedToken(tok))
                    };
                    Ok(Statement::Exit { value })
                },
                kwd => Err(ParseError::UnexpectedToken(Token::Keyword(kwd)))
            },
            tok => Err(ParseError::UnexpectedToken(tok)),
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        match self.peek()?.expect("a token") {
            Token::IntegerLiteral(lit) => { self.consume()?; Ok(Expr::IntegerLiteral(lit)) },
            tok => Err(ParseError::UnexpectedToken(tok)),
        }
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

