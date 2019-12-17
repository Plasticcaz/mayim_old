//!
//! This crate handles tokenizing and parsing a `mayim` source file into a `Vec` of `Expression`s.
//!
mod expression;
mod located;
mod location;
mod parser;
#[cfg(test)]
mod tests;
mod token;
mod tokenizer;

pub use string_cache::DefaultAtom as Atom;

pub use crate::{
    expression::{Expression, Literal},
    tokenizer::tokenize,
    located::Located,
    location::Location,
    token::Token
};

pub fn parse(filename: &str, source: &str) -> Vec<Expression> {
    parser::parse(tokenize(filename, source))
}
