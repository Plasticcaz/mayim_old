mod located;
mod location;
mod token;
mod tokenizer;

pub use crate::tokenizer::{located::Located, location::Location, token::Token};
pub use string_cache::DefaultAtom as Atom;

///
/// Convert the `source` into a `Vec` of `Token`s associated with the specified `filename`.
///
pub fn tokenize(filename: &str, source: &str) -> Vec<Located<Token>> {
    tokenizer::Tokenizer::new(filename, source).tokenize()
}
