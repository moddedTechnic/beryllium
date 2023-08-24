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
    Plus, Minus, Star, Slash,
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

