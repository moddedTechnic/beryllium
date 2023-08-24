#![cfg(test)]


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

