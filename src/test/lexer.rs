#![cfg(test)]


#[test]
fn integer_literal_tokenizes() {
    use fallible_iterator::FallibleIterator;
    use crate::lexer::{Tokenize, Token};

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
    use crate::lexer::{Tokenize, Token};

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
    use crate::lexer::{Tokenize, Token};

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
    use crate::lexer::{Tokenize, Token};

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
    use crate::lexer::{Tokenize, Keyword, Token};

    let tokens: Result<Vec<_>, _> = "exit".tokenize().collect();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 1);
    let token = tokens.get(0).unwrap().clone();
    assert_eq!(token, Token::Keyword(Keyword::Exit));
}

