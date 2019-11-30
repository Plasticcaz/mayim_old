//!
//! This crate converts a `Vec` of tokens into a `Vec` of statements _or_ a `Vec` of `SyntaxError`s,
//! depending on whether or not the tokens make sense as a list of top-level expressions.
//!
mod expression;
mod parser;

pub use crate::expression::{Expression, Literal};
use tokenizer::Token;

pub fn parse(tokens: Vec<Token>) -> Vec<Expression> {
    parser::parse(tokens)
}

#[cfg(test)]
mod tests {
    use crate::{parse, Expression, Literal};
    use tokenizer::{tokenize, Atom};

    const FILENAME: &str = "test.mayim";

    // TODO(zac): Write tests for various errors.

    #[test]
    fn should_parse_binding_declaration() {
        let source = "let x := 3";

        let top_level = parse(tokenize(FILENAME, source));

        let top = &top_level[0];
        dbg!(top);
        if let Expression::BindingDeclaration {
            identifier: (_, identifier),
            expression,
            ..
        } = &top
        {
            assert_eq!(identifier, "x");
            if let Expression::Literal((_, literal)) = expression.as_ref() {
                assert_eq!(literal, &Literal::Integer(Atom::from("3")))
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}
