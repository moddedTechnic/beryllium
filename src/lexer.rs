use std::collections::VecDeque;

use fallible_iterator::FallibleIterator;


pub trait Tokenize {
    fn tokenize(self) -> Lexer;
}

impl Tokenize for String {
    fn tokenize(self) -> Lexer {
        Lexer::new(self.chars().collect())
    }
}

impl Tokenize for &str {
    fn tokenize(self) -> Lexer {
        Lexer::new(self.chars().collect())
    }
}

impl Tokenize for Vec<char> {
    fn tokenize(self) -> Lexer {
        Lexer::new(self.into_iter().collect())
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    LParen, RParen,
    Semi,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LexerError {
    UnrecognizedCharacter(char),
}


pub struct Lexer {
    source: VecDeque<char>,
}

impl Lexer {
    pub fn new(source: VecDeque<char>) -> Self {
        Lexer { source }
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

    fn lex_symbol(&mut self) -> Result<Symbol, LexerError> {
        let character = self .consume()
            .ok_or(LexerError::UnrecognizedCharacter(0 as char))?;
        match character {
            '(' => Ok(Symbol::LParen),
            ')' => Ok(Symbol::RParen),
            ';' => Ok(Symbol::Semi),
            _ => Err(
                LexerError::UnrecognizedCharacter(character)
            ),
        }
    }
}

impl FallibleIterator for Lexer {
    type Item = Token;
    type Error = LexerError;

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

