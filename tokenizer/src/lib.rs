//!
//! This library handles converting of a mayim source file into a list of tokens.
//!

pub use string_cache::DefaultAtom as Atom;

pub use crate::{located::Located, location::Location, token::Token};

mod located;
mod location;
mod token;
mod tokenizer;

///
/// Convert the `source` into a `Vec` of `Token`s associated with the specified `filename`.
///
pub fn tokenize(filename: &str, source: &str) -> Vec<Located<Token>> {
    tokenizer::Tokenizer::new(filename, source).tokenize()
}

#[cfg(test)]
mod tests {
    use std::vec::IntoIter;

    use super::{tokenize, Atom, Located, Token};

    const FILENAME: &str = "test.mayim";

    fn assert_next_is(tokens: &mut IntoIter<Located<Token>>, expected: Token) {
        let got = tokens.next().map(|it| it.data).unwrap();
        assert_eq!(got, expected);
    }

    #[test]
    fn should_tokenize_let_expression() {
        let source = "let x := 3";
        let tokens = &mut tokenize(FILENAME, source).into_iter();

        assert_next_is(tokens, Token::Let);
        assert_next_is(tokens, Token::Identifier(Atom::from("x")));
        assert_next_is(tokens, Token::Assign);
        assert_next_is(tokens, Token::Integer(Atom::from("3")));
        assert_next_is(tokens, Token::EndOfFile);
        assert!(tokens.next().is_none());
    }
}
