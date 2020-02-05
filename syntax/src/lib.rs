//!
//! This crate handles tokenizing and parsing a `mayim` source file into a `Vec` of `Expression`s.
//!
mod expression;
mod location;
mod parser;
#[cfg(test)]
mod tests;
mod token;
mod tokenizer;

pub use string_cache::DefaultAtom as Atom;

pub use crate::{
    expression::{Error, Expression},
    location::Location,
    token::{AtomToken, Token},
    tokenizer::tokenize,
};

pub fn parse(filename: &str, source: &str) -> Vec<Expression> {
    parser::parse(tokenize(filename, source))
}
