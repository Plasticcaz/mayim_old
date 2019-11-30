use crate::{Expression, Literal};
use std::{iter::Peekable, vec::IntoIter};
use tokenizer::{Location, Token, TokenKind};

pub(crate) type Tokens = Peekable<IntoIter<Token>>;

pub(crate) fn parse(tokens: Vec<Token>) -> Vec<Expression> {
    let tokens = &mut tokens.into_iter().peekable();
    let mut top_level = Vec::new();
    while let Some(token) = tokens.peek() {
        if token.kind != TokenKind::EndOfFile {
            let expression = parse_expression(tokens);
            top_level.push(expression);
        } else {
            let _ = tokens.next();
        }
    }
    top_level
}

pub(crate) fn parse_expression(tokens: &mut Tokens) -> Expression {
    match tokens.next().expect(NO_EOF) {
        Token {
            location,
            kind: TokenKind::Let,
        } => parse_binding_declaration(tokens, location),
        Token {
            location,
            kind: TokenKind::Integer(atom),
        } => Expression::Literal((location, Literal::Integer(atom))),
        Token {
            location,
            kind: unexpected,
        } => Expression::Error(format!(
            "{} Error: Unexpected token '{:?}.'",
            location, unexpected
        )),
    }
}

fn parse_binding_declaration(tokens: &mut Tokens, let_keyword: Location) -> Expression {
    let identifier = match tokens.next().expect(NO_EOF) {
        Token {
            location,
            kind: TokenKind::Identifier(atom),
        } => (location, atom),
        Token {
            location,
            kind: unexpected,
        } => {
            return Expression::Error(format!(
                "{}: Error: Expected an identifier, but found '{:?}'.",
                location, unexpected
            ));
        }
    };
    let assign = match tokens.next().expect(NO_EOF) {
        Token {
            location,
            kind: TokenKind::Assign,
        } => location,
        Token {
            location,
            kind: unexpected,
        } => {
            return Expression::Error(format!(
                "{}: Error: Expected ':=', but found '{:?}'.",
                location, unexpected
            ));
        }
    };

    let expression = parse_expression(tokens);

    Expression::BindingDeclaration {
        let_keyword,
        identifier,
        assign,
        expression: Box::new(expression),
    }
}

const NO_EOF: &str = "Expected at least an EndOfFile token at this point.";
