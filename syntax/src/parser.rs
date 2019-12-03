use crate::{Expression, Literal, Located, Location, Token};
use std::{iter::Peekable, vec::IntoIter};

pub(crate) type Tokens = Peekable<IntoIter<Located<Token>>>;

pub(crate) fn parse(tokens: Vec<Located<Token>>) -> Vec<Expression> {
    let tokens = &mut tokens.into_iter().peekable();
    let mut top_level = Vec::new();
    while let Some(Located { data: peeked, .. }) = tokens.peek() {
        if *peeked != Token::EndOfFile {
            let expression = parse_expression(tokens);
            top_level.push(expression);
        } else {
            // Eat the EOF file. We can ignore it.
            let _ = tokens.next();
        }
    }
    top_level
}

pub(crate) fn parse_expression(tokens: &mut Tokens) -> Expression {
    match tokens.next().expect(NO_EOF) {
        Located {
            location,
            data: Token::Let,
        } => parse_binding_declaration(tokens, location),
        Located {
            location,
            data: Token::Decimal(atom),
        } => Expression::Literal(Located::new(location, Literal::Decimal(atom))),
        Located {
            location,
            data: Token::Integer(atom),
        } => Expression::Literal(Located::new(location, Literal::Integer(atom))),
        Located {
            location,
            data: Token::Identifier(atom),
        } => Expression::Identifier(Located::new(location, atom)),
        Located {
            location,
            data: Token::Boolean(atom),
        } => Expression::Literal(Located::new(location, Literal::Boolean(atom))),
        Located {
            location,
            data: unexpected,
        } => Expression::Error(Located::new(
            location,
            format!("Unexpected token '{:?}.'", unexpected),
        )),
    }
}

fn parse_binding_declaration(tokens: &mut Tokens, let_keyword: Location) -> Expression {
    let identifier = match tokens.next().expect(NO_EOF) {
        Located {
            location,
            data: Token::Identifier(atom),
        } => Located::new(location, atom),
        Located {
            location,
            data: unexpected,
        } => {
            return Expression::Error(Located::new(
                location,
                format!("Expected an identifier, but found '{:?}'.", unexpected),
            ));
        }
    };
    let assign = match tokens.next().expect(NO_EOF) {
        Located {
            location,
            data: Token::Assign,
        } => location,
        Located {
            location,
            data: unexpected,
        } => {
            return Expression::Error(Located::new(
                location,
                format!("Expected ':=', but found '{:?}'.", unexpected),
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
