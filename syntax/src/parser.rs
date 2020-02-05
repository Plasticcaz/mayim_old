use crate::{AtomToken, Expression, Token};
use std::{fmt, vec::IntoIter};

pub struct Tokens {
    iterator: IntoIter<Token>,
    next: Option<Token>,
}

impl Tokens {
    pub fn new(iterator: IntoIter<Token>) -> Tokens {
        Tokens {
            iterator,
            next: None,
        }
    }

    pub fn next(&mut self) -> Token {
        self.next
            .take()
            .or_else(|| self.iterator.next())
            .expect("Expected at least an EndOfFile token at this point.")
    }

    pub fn has_next(&mut self) -> bool {
        self.next.is_some() || {
            if let Some(token) = self.iterator.next() {
                self.put_back(token);
                true
            } else {
                false
            }
        }
    }

    pub fn put_back(&mut self, token: Token) {
        // TODO(zac): Is there a way to enforce this? Maybe by performing an action inside a closure?
        assert!(self.next.is_none());
        self.next = Some(token)
    }
}

macro_rules! match_enum_instance {
    ($t:expr, $q:path) => {
        match $t {
            $q(c) => Some(c),
            _ => None,
        }
    };
}

pub(crate) fn parse(tokens: Vec<Token>) -> Vec<Expression> {
    let tokens = &mut Tokens::new(tokens.into_iter());

    let mut expressions = Vec::new();

    while tokens.has_next() {
        let token = tokens.next();
        if match_enum_instance!(&token, Token::EndOfFile).is_none() {
            tokens.put_back(token);
            expressions.push(parse_expression(tokens))
        }
    }
    expressions
}

pub(crate) fn parse_expression(tokens: &mut Tokens) -> Expression {
    match tokens.next() {
        Token::Identifier(identifier) => {
            parse_identifier_expression(tokens, identifier)
        }
        Token::Integer(literal) => Expression::IntegerLiteral(literal),
        Token::Decimal(literal) => Expression::DecimalLiteral(literal),
        unknown => todo!(),
    }
}

fn parse_identifier_expression(
    tokens: &mut Tokens,
    identifier: AtomToken,
) -> Expression {
    match tokens.next() {
        // TODO(zac): Check to see if it's another type of expression that starts with an identifier.
        other_token => {
            tokens.put_back(other_token);
            Expression::Identifier(identifier)
        }
    }
}
