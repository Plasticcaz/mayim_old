//!
//! This file contains crate-level tests of the mayim `syntax` crate. This does not mean all tests
//! are found here, but behaviour-based tests of the crate as a whole are contained here.
//!
use crate::{parse, AtomToken, BindingDeclaration, Error, Expression};

const FILENAME: &str = "test.mayim";

macro_rules! expect_some_instance {
    ($t:expr, $q:path, $assertions:expr) => {
        match $t {
            Some($q(c)) => $assertions(c),
            Some(other) => {
                panic!("Found unexpected enum instance: {:#?}", other)
            }
            None => panic!("Got None instead of an enum instance"),
        }
    };
}

#[test]
fn should_parse_simple_identifier_expression() {
    let source = "john_j4cob";

    let mut expressions = parse(FILENAME, source).into_iter();

    expect_some_instance!(
        expressions.next(),
        Expression::Identifier,
        |AtomToken { atom, .. }: AtomToken| { assert_eq!(&atom, "john_j4cob") }
    );
}

#[test]
fn should_parse_numeric_expressions() {
    let source = "42 3.14";

    let mut expressions = parse(FILENAME, source).into_iter();

    expect_some_instance!(
        expressions.next(),
        Expression::IntegerLiteral,
        |AtomToken { atom, .. }: AtomToken| { assert_eq!(&atom, "42") }
    );

    expect_some_instance!(
        expressions.next(),
        Expression::DecimalLiteral,
        |AtomToken { atom, .. }: AtomToken| { assert_eq!(&atom, "3.14") }
    );
}

#[test]
fn should_parse_simple_assignment_expression() {
    let source = "let x = 3";

    let mut expressions = parse(FILENAME, source).into_iter();

    expect_some_instance!(
        expressions.next(),
        Expression::BindingDeclaration,
        |binding_declaration: Box<BindingDeclaration>| {
            assert_eq!(&binding_declaration.identifier.atom, "x");
            expect_some_instance!(
                Some(binding_declaration.initialized_to),
                Expression::IntegerLiteral,
                |AtomToken { atom, .. }: AtomToken| { assert_eq!(&atom, "3") }
            )
        }
    );
}

#[test]
fn should_return_error_if_assignment_token_at_start_of_expression() {
    let source = "=";

    let mut expressions = parse(FILENAME, source).into_iter();

    expect_some_instance!(
        expressions.next(),
        Expression::Error,
        |Error { message, .. }: Error| {
            assert!(message.contains("Unexpected token"))
        }
    );
}
