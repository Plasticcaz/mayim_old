//!
//! This crate handles tokenizing and parsing a `mayim` source file into a `Vec` of `Expression`s.
//!
mod expression;
mod parser;
#[cfg(test)]
mod tests;
mod tokenizer;

pub use crate::expression::{Expression, Literal};
use crate::tokenizer::tokenize;
pub use crate::tokenizer::{Atom, Located, Location, Token};

pub fn parse(filename: &str, source: &str) -> Vec<Expression> {
    parser::parse(tokenize(filename, source))
}
