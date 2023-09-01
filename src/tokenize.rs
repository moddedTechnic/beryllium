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


#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Location {
    pub index: u64,
    pub line: u64,
    pub column: u64,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub data: TokenData,
    pub location: Location,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenData {
    Identifier(String),
    IntegerLiteral(String),
    Keyword(Keyword),
    Symbol(Symbol),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    Exit,
    Let,
    If, Else,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    LParen, RParen,
    LBrace, RBrace,
    LAngle, RAngle,
    Semi,
    Equals,
    Plus, Minus, Star,
    Slash, Percent,
    Equality, NonEquality,
    GreaterEqual, LesserEqual,
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
    location: Location,
}

impl TokenStream {
    pub fn new(source: VecDeque<char>) -> Self {
        TokenStream { source, location: Location::default() }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(0).copied()
    }

    fn consume(&mut self) -> Option<char> {
        match self.source.pop_front() {
            Some(char) => {
                self.location.index += 1;
                self.location.column += 1;
                if char == '\n' {
                    self.location.line += 1;
                    self.location.column = 0;
                }
                Some(char)
            },
            None => None,
        }
    }

    fn lex_identifier(&mut self) -> Token {
        let mut buffer = String::new();
        let location = self.location;
        while let Some(character) = self.peek() {
            if !character.is_alphanumeric() && character != '_' {
                break;
            }
            buffer.push(self.consume().unwrap());
        };
        let data = match buffer.as_str() {
            "exit" => TokenData::Keyword(Keyword::Exit),
            "let"  => TokenData::Keyword(Keyword::Let),
            "if"   => TokenData::Keyword(Keyword::If),
            "else" => TokenData::Keyword(Keyword::Else),
            _ => TokenData::Identifier(buffer),
        };
        Token { data, location }
    }

    fn lex_number(&mut self) -> Token {
        let mut buffer = String::new();
        let location = self.location;
        while let Some(character) = self.peek() {
            if !character.is_numeric() {
                break;
            }
            buffer.push(self.consume().unwrap());
        };
        Token {
            data: TokenData::IntegerLiteral(buffer),
            location,
        }
    }

    fn lex_symbol(&mut self) -> Result<Symbol, TokenizerError> {
        let character = self.consume()
            .ok_or(TokenizerError::UnrecognizedCharacter(0 as char))?;
        match character {
            '(' => Ok(Symbol::LParen),
            ')' => Ok(Symbol::RParen),
            '{' => Ok(Symbol::LBrace),
            '}' => Ok(Symbol::RBrace),
            '<' => match self.peek()
                    .ok_or(TokenizerError::UnrecognizedCharacter(0 as char))? {
                '=' => { self.consume(); Ok(Symbol::LesserEqual) },
                _ => Ok(Symbol::LAngle)
            },
            '>' => match self.peek()
                    .ok_or(TokenizerError::UnrecognizedCharacter(0 as char))? {
                '=' => { self.consume(); Ok(Symbol::GreaterEqual) },
                _ => Ok(Symbol::RAngle)
            },

            '!' => match self.peek()
                    .ok_or(TokenizerError::UnrecognizedCharacter(0 as char))? {
                '=' => { self.consume(); Ok(Symbol::NonEquality) },
                _ => Err(TokenizerError::UnrecognizedCharacter('!')),
            }
            '=' => match self.peek()
                    .ok_or(TokenizerError::UnrecognizedCharacter(0 as char))? {
                '=' => { self.consume(); Ok(Symbol::Equality) },
                _ => Ok(Symbol::Equals)
            },
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
                let location = self.location;
                Ok(Some(Token {
                    data: TokenData::Symbol(self.lex_symbol()?),
                    location,
                }))
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
    let tokens: Result<Vec<_>, _> = "123".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 1);
    let token = tokens.get(0).unwrap().clone().data;
    assert_eq!(token, TokenData::IntegerLiteral("123".into()));
}


#[test]
fn many_integer_literals_tokenize() {
    let tokens: Result<Vec<_>, _> = "123 456 789".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 3);

    let token = tokens.get(0).unwrap().clone().data;
    assert_eq!(token, TokenData::IntegerLiteral("123".into()));

    let token = tokens.get(1).unwrap().clone().data;
    assert_eq!(token, TokenData::IntegerLiteral("456".into()));

    let token = tokens.get(2).unwrap().clone().data;
    assert_eq!(token, TokenData::IntegerLiteral("789".into()));
}

#[test]
fn identifier_tokenizes() {
    let tokens: Result<Vec<_>, _> = "main".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 1);
    let token = tokens.get(0).unwrap().clone().data;
    assert_eq!(token, TokenData::Identifier("main".into()));
}

#[test]
fn many_identifiers_tokenize() {
    let tokens: Result<Vec<_>, _> = "main _start foo".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 3);

    let token = tokens.get(0).unwrap().clone().data;
    assert_eq!(token, TokenData::Identifier("main".into()));

    let token = tokens.get(1).unwrap().clone().data;
    assert_eq!(token, TokenData::Identifier("_start".into()));

    let token = tokens.get(2).unwrap().clone().data;
    assert_eq!(token, TokenData::Identifier("foo".into()));
}

#[test]
fn keyword_exit_tokenizes() {
    let tokens: Result<Vec<_>, _> = "exit".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 1);
    let token = tokens.get(0).unwrap().clone().data;
    assert_eq!(token, TokenData::Keyword(Keyword::Exit));
}

#[test]
fn keyword_let_tokenizes() {
    let tokens: Result<Vec<_>, _> = "let".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 1);
    let token = tokens.get(0).unwrap().clone().data;
    assert_eq!(token, TokenData::Keyword(Keyword::Let));
}

#[test]
fn keyword_if_tokenizes() {
    let tokens: Result<Vec<_>, _> = "if".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 1);
    let token = tokens.get(0).unwrap().clone().data;
    assert_eq!(token, TokenData::Keyword(Keyword::If));
}

#[test]
fn maths_expression_tokenizes() {
    let tokens: Result<Vec<_>, _> = "1 + 2 - 3 * 4 / 5 % 6".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 11);

    let expected_tokens = vec![
        TokenData::IntegerLiteral(String::from("1")),
        TokenData::Symbol(Symbol::Plus),
        TokenData::IntegerLiteral(String::from("2")),
        TokenData::Symbol(Symbol::Minus),
        TokenData::IntegerLiteral(String::from("3")),
        TokenData::Symbol(Symbol::Star),
        TokenData::IntegerLiteral(String::from("4")),
        TokenData::Symbol(Symbol::Slash),
        TokenData::IntegerLiteral(String::from("5")),
        TokenData::Symbol(Symbol::Percent),
        TokenData::IntegerLiteral(String::from("6")),
    ];

    for (token, expected_token) in tokens.into_iter().zip(expected_tokens) {
        assert_eq!(token.data, expected_token);
    }
}

#[test]
fn brackets_tokenize() {
    let tokens: Result<Vec<_>, _> = "( ) { }".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();

    let expected_tokens = vec![
        TokenData::Symbol(Symbol::LParen),
        TokenData::Symbol(Symbol::RParen),
        TokenData::Symbol(Symbol::LBrace),
        TokenData::Symbol(Symbol::RBrace),
    ];

    assert_eq!(tokens.len(), expected_tokens.len());
    for (token, expected_token) in tokens.into_iter().zip(expected_tokens) {
        assert_eq!(token.data, expected_token);
    }
}

#[test]
fn comparison_operators_tokenize() {
    let tokens: Result<Vec<_>, _> = "== != < <= > >=".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();

    let expected_tokens = vec![
        TokenData::Symbol(Symbol::Equality),
        TokenData::Symbol(Symbol::NonEquality),
        TokenData::Symbol(Symbol::LAngle),
        TokenData::Symbol(Symbol::LesserEqual),
        TokenData::Symbol(Symbol::RAngle),
        TokenData::Symbol(Symbol::GreaterEqual),
    ];

    assert_eq!(tokens.len(), expected_tokens.len());
    for (token, expected_token) in tokens.into_iter().zip(expected_tokens) {
        assert_eq!(token.data, expected_token);
    }
}

