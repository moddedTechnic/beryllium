use std::collections::VecDeque;

use fallible_iterator::FallibleIterator;

use crate::{
    tokenize::{
        Keyword, Symbol,
        TokenStream, Token, TokenData,
        TokenizerError,
    },
    ast::{
        Param, Expr,
        Program, Statement, Item,
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
            program.push(self.parse_item()?);
        }
        Ok(Program(program))
    }

    fn parse_item(&mut self) -> Result<Item, ParseError> {
        match self.peek()?.expect("a token") {
            Token { data: TokenData::Keyword(Keyword::Fn), location: _ } => {
                self.consume()?;
                let name = match self.consume()?.expect("an identifier") {
                    Token { data: TokenData::Identifier(ident), location: _ } => ident,
                    tok => return Err(ParseError::UnexpectedToken(tok)),
                };
                match self.consume()?.expect("a left parenthesis") {
                    Token { data: TokenData::Symbol(Symbol::LParen), location: _ } => (),
                    tok => return Err(ParseError::UnexpectedToken(tok))
                };
                let params = self.parse_params()?;
                match self.consume()?.expect("a right parenthesis") {
                    Token { data: TokenData::Symbol(Symbol::RParen), location: _ } => (),
                    tok => return Err(ParseError::UnexpectedToken(tok))
                };
                let body = self.parse_statement()?;
                Ok(Item::Function { name, params, body })
            },
            tok => Err(ParseError::UnexpectedToken(tok)),
        }
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, ParseError> {
        let name = match self.peek()?.expect("an identifier or a right parenthesis") {
            Token { data: TokenData::Symbol(Symbol::RParen), location: _ } => return Ok(vec![]),
            Token { data: TokenData::Identifier(ident), location: _ } => ident,
            tok => return Err(ParseError::UnexpectedToken(tok)),
        };
        self.consume()?;
        let mut params = vec![Param { name }];
        match self.peek()?.expect("a comma or a right parenthesis") {
            Token { data: TokenData::Symbol(Symbol::RParen), location: _ } => (),
            Token { data: TokenData::Symbol(Symbol::Comma), location: _ } => { self.consume()?; params.extend(self.parse_params()?); },
            tok => return Err(ParseError::UnexpectedToken(tok)),
        };
        Ok(params)
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
                    let is_mutable = match self.peek()?.expect("an identifier or `mut`") {
                        Token { data: TokenData::Keyword(Keyword::Mut), location: _ } => {
                            self.consume()?;
                            true
                        },
                        _ => false,
                    };
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
                    Ok(Statement::Let { identifier, value, is_mutable })
                },
                Keyword::If => self.parse_if().map(Statement::Expr),
                Keyword::Loop => self.parse_loop().map(Statement::Expr),
                Keyword::While => self.parse_while().map(Statement::Expr),

                Keyword::Break => {
                    self.consume()?;
                    match self.consume()?.expect("a semicolon") {
                        Token { data: TokenData::Symbol(Symbol::Semi), location: _ } => (),
                        tok => return Err(ParseError::UnexpectedToken(tok))
                    };
                    Ok(Statement::Break)
                },
                Keyword::Continue => {
                    self.consume()?;
                    match self.consume()?.expect("a semicolon") {
                        Token { data: TokenData::Symbol(Symbol::Semi), location: _ } => (),
                        tok => return Err(ParseError::UnexpectedToken(tok))
                    };
                    Ok(Statement::Continue)
                },

                Keyword::Return => {
                    self.consume()?;
                    let value = self.parse_expression()?;
                    match self.consume()?.expect("a semicolon") {
                        Token { data: TokenData::Symbol(Symbol::Semi), location: _ } => (),
                        tok => return Err(ParseError::UnexpectedToken(tok))
                    };
                    Ok(Statement::Return(value))
                }

                kwd => Err(ParseError::UnexpectedToken(Token { data: TokenData::Keyword(kwd), location })),
            },
            Token {
                data: TokenData::Symbol(Symbol::LBrace),
                location: _,
            } => self.parse_block().map(Statement::Expr),
            _ => {
                let expr = self.parse_expression()?;
                match self.consume()?.expect("a semicolon `;`") {
                    Token {
                        data: TokenData::Symbol(Symbol::Semi),
                        location: _
                    } => Ok(Statement::Expr(expr)),
                    tok => Err(ParseError::UnexpectedToken(tok)),
                }
            },
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_assign_expr()
    }

    fn parse_assign_expr(&mut self) -> Result<Expr, ParseError> {
        let identifier = match self.peek()?.expect("a token") {
            Token { data: TokenData::Identifier(ident), location: _ } => ident,
            _ => return self.parse_expression_cmp_part(),
        };
        let symbol = match self.peek_ahead(1)?.expect("an operator") {
            Token { data:TokenData::Symbol(symbol), location: _ } => symbol,
            _ => return self.parse_expression_cmp_part(),
        };
        match symbol {
            Symbol::PlusEq => {
                self.consume()?;
                self.consume()?;
                Ok(Expr::AddAssign {
                    identifier,
                    value: Box::new(self.parse_expression()?),
                })
            },
            Symbol::MinusEq => {
                self.consume()?;
                self.consume()?;
                Ok(Expr::SubAssign {
                    identifier,
                    value: Box::new(self.parse_expression()?),
                })
            },
            Symbol::StarEq => {
                self.consume()?;
                self.consume()?;
                Ok(Expr::MulAssign {
                    identifier,
                    value: Box::new(self.parse_expression()?),
                })
            },
            Symbol::SlashEq => {
                self.consume()?;
                self.consume()?;
                Ok(Expr::DivAssign {
                    identifier,
                    value: Box::new(self.parse_expression()?),
                })
            },
            Symbol::PercentEq => {
                self.consume()?;
                self.consume()?;
                Ok(Expr::ModAssign {
                    identifier,
                    value: Box::new(self.parse_expression()?),
                })
            },
            _ => self.parse_expression_cmp_part(),
        }
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
        let expr = self.parse_expression_mul_part()?;
        Ok(
            if let Some(Token { data, location: _ }) = self.peek()? {
                match data {
                    TokenData::Symbol(Symbol::Plus) => {
                        self.consume()?;
                        match self.parse_expression_add_part()? {
                            base @ Expr::Add(_, _) | base @ Expr::Sub(_, _)
                                => base.map_left(|lhs| Expr::Add(Box::new(expr.clone()), lhs)),
                            other => Expr::Add(Box::new(expr), Box::new(other)),
                        }
                    },
                    TokenData::Symbol(Symbol::Minus) => {
                        self.consume()?;
                        match self.parse_expression_add_part()? {
                            base @ Expr::Add(_, _) | base @ Expr::Sub(_, _)
                                => base.map_left(|lhs| Expr::Sub(Box::new(expr.clone()), lhs)),
                            other => Expr::Sub(Box::new(expr), Box::new(other)),
                        }
                    },
                    _ => expr,
                }
            } else {
                expr
            }
        )
    }

    fn parse_expression_mul_part(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_atom()?;
        Ok(
            if let Some(Token { data, location: _ }) = self.peek()? {
                match data {
                    TokenData::Symbol(Symbol::Star) => {
                        self.consume()?;
                        match self.parse_expression_mul_part()? {
                            base @ Expr::Mul(_, _) | base @ Expr::Div(_, _) | base @ Expr::Mod(_, _)
                                => base.map_left(|lhs| Expr::Mul(Box::new(expr.clone()), lhs)),
                            other => Expr::Mul(Box::new(expr), Box::new(other)),
                        }
                    },
                    TokenData::Symbol(Symbol::Slash) => {
                        self.consume()?;
                        match self.parse_expression_mul_part()? {
                            base @ Expr::Mul(_, _) | base @ Expr::Div(_, _) | base @ Expr::Mod(_, _)
                                => base.map_left(|lhs| Expr::Div(Box::new(expr.clone()), lhs)),
                            other => Expr::Div(Box::new(expr), Box::new(other)),
                        }
                    },
                    TokenData::Symbol(Symbol::Percent) => {
                        self.consume()?;
                        match self.parse_expression_mul_part()? {
                            base @ Expr::Mul(_, _) | base @ Expr::Div(_, _) | base @ Expr::Mod(_, _)
                                => base.map_left(|lhs| Expr::Mod(Box::new(expr.clone()), lhs)),
                            other => Expr::Mod(Box::new(expr), Box::new(other)),
                        }
                    },
                    _ => expr,
                }
            } else {
                expr
            }
        )
    }

    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        match self.peek()?.expect("a token") {
            Token { data: TokenData::IntegerLiteral(lit), location: _ } => { self.consume()?; Ok(Expr::IntegerLiteral(lit)) },
            Token { data: TokenData::Identifier(ident), location: _ } => {
                self.consume()?;
                match self.peek()? {
                    Some(Token { data: TokenData::Symbol(Symbol::LParen), location: _ }) => {
                        self.consume()?;
                        let args = self.parse_args()?;
                        match self.consume()?.expect("a right parenthesis `)`") {
                            Token { data: TokenData::Symbol(Symbol::RParen), location: _ } => (),
                            tok => return Err(ParseError::UnexpectedToken(tok)),
                        };
                        Ok(Expr::FunctionCall { name: ident, args })
                    },
                    _ => Ok(Expr::Identifier(ident)),
                }
            }

            Token { data: TokenData::Symbol(Symbol::LBrace), location: _ } => self.parse_block(),
            Token { data: TokenData::Keyword(Keyword::If), location: _ } => self.parse_if(),
            Token { data: TokenData::Keyword(Keyword::Loop), location: _ } => self.parse_loop(),
            Token { data: TokenData::Keyword(Keyword::While), location: _ } => self.parse_while(),
            tok => Err(ParseError::UnexpectedToken(tok)),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        let expr = match self.peek()?.expect("an identifier or a right parenthesis") {
            Token { data: TokenData::Symbol(Symbol::RParen), location: _ } => return Ok(vec![]),
            _ => self.parse_expression()?,
        };
        let mut args = vec![expr];
        match self.peek()?.expect("a comma or a right parenthesis") {
            Token { data: TokenData::Symbol(Symbol::RParen), location: _ } => (),
            Token { data: TokenData::Symbol(Symbol::Comma), location: _ } => { self.consume()?; args.extend(self.parse_args()?); },
            tok => return Err(ParseError::UnexpectedToken(tok)),
        };
        Ok(args)
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

    fn parse_loop(&mut self) -> Result<Expr, ParseError> {
        match self.consume()?.expect("keyword `loop`") {
            Token { data: TokenData::Keyword(Keyword::Loop), location: _ } => (),
            tok => return Err(ParseError::UnexpectedToken(tok)),
        }
        let body = Box::new(self.parse_statement()?);
        Ok(Expr::Loop { body })
    }

    fn parse_while(&mut self) -> Result<Expr, ParseError> {
        match self.consume()?.expect("keyword `while`") {
            Token { data: TokenData::Keyword(Keyword::While), location: _ } => (),
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
        Ok(Expr::While { check, body })
    }

    fn is_empty(&mut self) -> Result<bool, TokenizerError> {
        Ok(self.peek()?.is_none())
    }

    fn peek(&mut self) -> Result<Option<Token>, TokenizerError> {
        self.peek_ahead(0)
    }

    fn peek_ahead(&mut self, count: usize) -> Result<Option<Token>, TokenizerError> {
        while self.buffer.len() < count + 1 {
            match self.tokens.next()? {
                Some(token) => self.buffer.push_back(token),
                None => return Ok(None),
            };
        };
        Ok(self.buffer.get(count).cloned())

    }

    fn consume(&mut self) -> Result<Option<Token>, TokenizerError> {
        if self.buffer.is_empty() {
            self.tokens.next()
        } else {
            Ok(self.buffer.pop_front())
        }
    }
}

