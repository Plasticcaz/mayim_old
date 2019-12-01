//!
//! This crate converts a `Vec` of tokens into a `Vec` of statements _or_ a `Vec` of `SyntaxError`s,
//! depending on whether or not the tokens make sense as a list of top-level expressions.
//!
mod expression;
mod parser;

pub use crate::expression::{Expression, Literal};
use tokenizer::{Located, Token};

pub fn parse(tokens: Vec<Located<Token>>) -> Vec<Expression> {
    parser::parse(tokens)
}

#[cfg(test)]
mod tests {
    use crate::{parse, Expression, Literal};
    use tokenizer::tokenize;

    const FILENAME: &str = "test.mayim";

    // TODO(zac): Write tests for various errors.

    #[test]
    fn should_parse_literal_expressions() {
        let source = "42 3.14 true false";

        let mut expressions = parse(tokenize(FILENAME, source)).into_iter();

        match expressions.next().unwrap() {
            Expression::Literal((_, Literal::Integer(literal))) => assert_eq!(&literal, "42"),
            _ => assert!(false),
        }

        match expressions.next().unwrap() {
            Expression::Literal((_, Literal::Decimal(literal))) => assert_eq!(&literal, "3.14"),
            _ => assert!(false),
        }

        match expressions.next().unwrap() {
            Expression::Literal((_, Literal::Boolean(literal))) => assert_eq!(&literal, "true"),
            _ => assert!(false),
        }

        match expressions.next().unwrap() {
            Expression::Literal((_, Literal::Boolean(literal))) => assert_eq!(&literal, "false"),
            _ => assert!(false),
        }
    }

    #[test]
    fn should_parse_binding_declaration() {
        let source = "let x := 3";

        let mut expressions = parse(tokenize(FILENAME, source)).into_iter();

        match expressions.next().unwrap() {
            Expression::BindingDeclaration {
                identifier: (_, identifier),
                expression,
                ..
            } => {
                assert_eq!(&identifier, "x");
                match expression.as_ref() {
                    Expression::Literal((_, Literal::Integer(atom))) => assert_eq!(atom, "3"),
                    _ => assert!(false),
                }
            }
            _ => assert!(false)
        }
    }
}
