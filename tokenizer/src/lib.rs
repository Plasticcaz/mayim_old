//!
//! This library handles converting of a mayim source file into a list of tokens.
//!

pub use string_cache::DefaultAtom as Atom;

pub use crate::{location::Location, token::Token, token_kind::TokenKind};

mod location;
mod token;
mod token_kind;
mod tokenizer;

///
/// Convert the `source` into a `Vec` of `Token`s associated with the specified `filename`.
///
pub fn tokenize(filename: &str, source: &str) -> Vec<Token> {
    tokenizer::Tokenizer::new(filename, source).tokenize()
}

#[cfg(test)]
mod tests {
    use std::vec::IntoIter;

    use super::{tokenize, Atom, Token, TokenKind};

    const FILENAME: &str = "test.mayim";

    fn assert_next_is(tokens: &mut IntoIter<Token>, expected: TokenKind) {
        let got = tokens.next().map(|it| it.kind).unwrap();
        assert_eq!(got, expected);
    }

    #[test]
    fn should_tokenize_let_expression() {
        let source = "let x := 3";
        let tokens = &mut tokenize(FILENAME, source).into_iter();

        assert_next_is(tokens, TokenKind::Let);
        assert_next_is(tokens, TokenKind::Identifier(Atom::from("x")));
        assert_next_is(tokens, TokenKind::Assign);
        assert_next_is(tokens, TokenKind::Integer(Atom::from("3")));
        assert_next_is(tokens, TokenKind::EndOfFile);
        assert!(tokens.next().is_none());
    }
}
