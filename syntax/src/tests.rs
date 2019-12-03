//!
//! This file contains crate-level tests of the mayim `syntax` crate. This does not mean all tests
//! are found here, but behaviour-based tests of the crate as a whole are contained here.
//!
use crate::{parse, Expression, Literal, Located};

const FILENAME: &str = "test.mayim";

// TODO(zac): Write tests for various errors.

#[test]
fn should_parse_identifier_expressions() {
    let source = "john_j4cob j1ng1eh31mer schmidt _his_name_is_my_name_too";

    let mut expressions = parse(FILENAME, source).into_iter();

    match expressions.next().unwrap() {
        Expression::Identifier(Located { data: atom, .. }) => assert_eq!(&atom, "john_j4cob"),
        _ => assert!(false),
    }

    match expressions.next().unwrap() {
        Expression::Identifier(Located { data: atom, .. }) => assert_eq!(&atom, "j1ng1eh31mer"),
        _ => assert!(false),
    }

    match expressions.next().unwrap() {
        Expression::Identifier(Located { data: atom, .. }) => assert_eq!(&atom, "schmidt"),
        _ => assert!(false),
    }

    match expressions.next().unwrap() {
        Expression::Identifier(Located { data: atom, .. }) => {
            assert_eq!(&atom, "_his_name_is_my_name_too")
        }
        _ => assert!(false),
    }
}

#[test]
fn should_parse_literal_expressions() {
    let source = "42 3.14 true false";

    let mut expressions = parse(FILENAME, source).into_iter();

    match expressions.next().unwrap() {
        Expression::Literal(Located {
            data: Literal::Integer(literal),
            ..
        }) => assert_eq!(&literal, "42"),
        _ => assert!(false),
    }

    match expressions.next().unwrap() {
        Expression::Literal(Located {
            data: Literal::Decimal(literal),
            ..
        }) => assert_eq!(&literal, "3.14"),
        _ => assert!(false),
    }

    match expressions.next().unwrap() {
        Expression::Literal(Located {
            data: Literal::Boolean(literal),
            ..
        }) => assert_eq!(&literal, "true"),
        _ => assert!(false),
    }

    match expressions.next().unwrap() {
        Expression::Literal(Located {
            data: Literal::Boolean(literal),
            ..
        }) => assert_eq!(&literal, "false"),
        _ => assert!(false),
    }
}

#[test]
fn should_parse_binding_declaration() {
    let source = "let x := 3";

    let mut expressions = parse(FILENAME, source).into_iter();

    match expressions.next().unwrap() {
        Expression::BindingDeclaration {
            identifier: Located {
                data: identifier, ..
            },
            expression,
            ..
        } => {
            assert_eq!(&identifier, "x");
            match expression.as_ref() {
                Expression::Literal(Located {
                    data: Literal::Integer(atom),
                    ..
                }) => assert_eq!(atom, "3"),
                _ => assert!(false),
            }
        }
        _ => assert!(false),
    }
}
