use std::collections::VecDeque;

use fallible_iterator::FallibleIterator;


pub trait Tokenize {
    fn tokenize(self) -> TokenStream;
}

impl Tokenize for String {
    fn tokenize(self) -> TokenStream {
        TokenStream::new(self.chars().collect())
    }
}

impl Tokenize for &str {
    fn tokenize(self) -> TokenStream {
        TokenStream::new(self.chars().collect())
    }
}

impl Tokenize for Vec<char> {
    fn tokenize(self) -> TokenStream {
        TokenStream::new(self.into_iter().collect())
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    IntegerLiteral(String),
    Keyword(Keyword),
    Symbol(Symbol),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    Exit,
    Let,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    LParen, RParen,
    Semi,
    Equals,
    Plus, Minus, Star,
    Slash, Percent,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenizerError {
    UnrecognizedCharacter(char),
}

impl std::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for TokenizerError {}


pub struct TokenStream {
    source: VecDeque<char>,
}

impl TokenStream {
    pub fn new(source: VecDeque<char>) -> Self {
        TokenStream { source }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(0).copied()
    }

    fn consume(&mut self) -> Option<char> {
        self.source.pop_front()
    }

    fn lex_identifier(&mut self) -> Token {
        let mut buffer = String::new();
        while let Some(character) = self.peek() {
            if !character.is_alphanumeric() && character != '_' {
                break;
            }
            buffer.push(self.consume().unwrap());
        };
        match buffer.as_str() {
            "exit" => Token::Keyword(Keyword::Exit),
            "let" => Token::Keyword(Keyword::Let),
            _ => Token::Identifier(buffer),
        }
    }

    fn lex_number(&mut self) -> Token {
        let mut buffer = String::new();
        while let Some(character) = self.peek() {
            if !character.is_numeric() {
                break;
            }
            buffer.push(self.consume().unwrap());
        };
        Token::IntegerLiteral(buffer)
    }

    fn lex_symbol(&mut self) -> Result<Symbol, TokenizerError> {
        let character = self .consume()
            .ok_or(TokenizerError::UnrecognizedCharacter(0 as char))?;
        match character {
            '=' => Ok(Symbol::Equals),
            '(' => Ok(Symbol::LParen),
            ')' => Ok(Symbol::RParen),
            ';' => Ok(Symbol::Semi),

            '+' => Ok(Symbol::Plus),
            '-' => Ok(Symbol::Minus),
            '*' => Ok(Symbol::Star),
            '/' => Ok(Symbol::Slash),
            '%' => Ok(Symbol::Percent),
            _ => Err(
                TokenizerError::UnrecognizedCharacter(character)
            ),
        }
    }
}

impl FallibleIterator for TokenStream {
    type Item = Token;
    type Error = TokenizerError;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        while let Some(character) = self.peek() {
            return if character.is_alphabetic() || character == '_' {
                Ok(Some(self.lex_identifier()))
            } else if character.is_numeric() {
                Ok(Some(self.lex_number()))
            } else if character.is_whitespace() {
                self.consume();
                continue;
            } else {
                Ok(Some(Token::Symbol(self.lex_symbol()?)))
            };
        }
        Ok(None)
    }
}


/********************************************************/
/*                                                      */
/*                         TESTS                        */
/*                                                      */
/********************************************************/


#[test]
fn integer_literal_tokenizes() {
    use fallible_iterator::FallibleIterator;
    use crate::tokenize::{Tokenize, Token};

    let tokens: Result<Vec<_>, _> = "123".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 1);
    let token = tokens.get(0).unwrap().clone();
    assert_eq!(token, Token::IntegerLiteral("123".into()));
}


#[test]
fn many_integer_literals_tokenize() {
    use fallible_iterator::FallibleIterator;
    use crate::tokenize::{Tokenize, Token};

    let tokens: Result<Vec<_>, _> = "123 456 789".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 3);

    let token = tokens.get(0).unwrap().clone();
    assert_eq!(token, Token::IntegerLiteral("123".into()));

    let token = tokens.get(1).unwrap().clone();
    assert_eq!(token, Token::IntegerLiteral("456".into()));

    let token = tokens.get(2).unwrap().clone();
    assert_eq!(token, Token::IntegerLiteral("789".into()));
}

#[test]
fn identifier_tokenizes() {
    use fallible_iterator::FallibleIterator;
    use crate::tokenize::{Tokenize, Token};

    let tokens: Result<Vec<_>, _> = "main".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 1);
    let token = tokens.get(0).unwrap().clone();
    assert_eq!(token, Token::Identifier("main".into()));
}

#[test]
fn many_identifiers_tokenize() {
    use fallible_iterator::FallibleIterator;
    use crate::tokenize::{Tokenize, Token};

    let tokens: Result<Vec<_>, _> = "main _start foo".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 3);

    let token = tokens.get(0).unwrap().clone();
    assert_eq!(token, Token::Identifier("main".into()));

    let token = tokens.get(1).unwrap().clone();
    assert_eq!(token, Token::Identifier("_start".into()));

    let token = tokens.get(2).unwrap().clone();
    assert_eq!(token, Token::Identifier("foo".into()));
}

#[test]
fn keyword_exit_tokenizes() {
    use fallible_iterator::FallibleIterator;
    use crate::tokenize::{Tokenize, Keyword, Token};

    let tokens: Result<Vec<_>, _> = "exit".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 1);
    let token = tokens.get(0).unwrap().clone();
    assert_eq!(token, Token::Keyword(Keyword::Exit));
}

#[test]
fn keyword_let_tokenizes() {
    use fallible_iterator::FallibleIterator;
    use crate::tokenize::{Tokenize, Keyword, Token};

    let tokens: Result<Vec<_>, _> = "let".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 1);
    let token = tokens.get(0).unwrap().clone();
    assert_eq!(token, Token::Keyword(Keyword::Let));
}

#[test]
fn maths_expression_tokenizes() {
    use fallible_iterator::FallibleIterator;
    use crate::tokenize::{Tokenize, Symbol, Token};

    let tokens: Result<Vec<_>, _> = "1 + 2 - 3 * 4 / 5 % 6".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 11);

    let expected_tokens = vec![
        Token::IntegerLiteral(String::from("1")),
        Token::Symbol(Symbol::Plus),
        Token::IntegerLiteral(String::from("2")),
        Token::Symbol(Symbol::Minus),
        Token::IntegerLiteral(String::from("3")),
        Token::Symbol(Symbol::Star),
        Token::IntegerLiteral(String::from("4")),
        Token::Symbol(Symbol::Slash),
        Token::IntegerLiteral(String::from("5")),
        Token::Symbol(Symbol::Percent),
        Token::IntegerLiteral(String::from("6")),
    ];

    for (token, expected_token) in tokens.into_iter().zip(expected_tokens) {
        assert_eq!(token, expected_token);
    }
}

