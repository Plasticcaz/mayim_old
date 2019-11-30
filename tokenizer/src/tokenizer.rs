//!
//! This module contains the guts of the tokenizer.
//!
//! To do the actual tokenizing, we construct a Tokenizer instance for the file we are tokenizing,
//! and call the tokenize method.
//!
//! However, this module is internal to this crate. The end user should just be able to call the
//! top-level `tokenize` function in lib.rs, and not care about this implementation detail.
//!
use crate::{Atom, Located, Location, Token};
use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub(crate) struct Tokenizer<'src> {
    location: Location,
    index: usize,
    chars: Peekable<Chars<'src>>,
    source: &'src str,
}

impl<'src> Tokenizer<'src> {
    pub(crate) fn new(filename: &str, source: &'src str) -> Tokenizer<'src> {
        Tokenizer {
            location: Location::new(Atom::from(filename), 1, 1),
            index: 0,
            chars: source.chars().peekable(),
            source,
        }
    }

    pub(crate) fn tokenize(mut self) -> Vec<Located<Token>> {
        let mut tokens = Vec::new();

        while let Some(c) = self.chars.peek().cloned() {
            let location = self.location.clone();
            let start = self.index;

            if c.is_whitespace() {
                self.eat_while(char::is_whitespace);
            } else if c.is_alphabetic() {
                self.eat_while(char::is_alphanumeric);
                let lexeme = &self.source[start..self.index];
                let kind = match lexeme {
                    "let" => Token::Let,
                    "true" | "false" => Token::Boolean(Atom::from(lexeme)),
                    _ => Token::Identifier(Atom::from(lexeme)),
                };

                tokens.push(Located::new(location, kind));
            } else if c.is_numeric() {
                let read_decimal = &mut false;
                self.eat_while(|c| {
                    if !*read_decimal && c == '.' {
                        *read_decimal = true;
                        true
                    } else {
                        c.is_numeric()
                    }
                });

                let lexeme = Atom::from(&self.source[start..self.index]);
                let kind = if *read_decimal {
                    Token::Decimal(lexeme)
                } else {
                    Token::Integer(lexeme)
                };

                tokens.push(Located::new(location, kind));
            } else {
                let kind = if self.eat(":=") {
                    Token::Assign
                } else {
                    self.advance();

                    let lexeme = Atom::from(&self.source[start..self.index]);
                    Token::Unknown(lexeme)
                };

                tokens.push(Located::new(location, kind));
            }
        }

        tokens.push(Located::new(self.location, Token::EndOfFile));

        tokens
    }

    fn eat(&mut self, expected: &str) -> bool {
        let end_index = self.index + expected.len();
        let within_bounds = end_index <= self.source.len();

        if within_bounds && &self.source[self.index..end_index] == expected {
            for _ in 0..expected.len() {
                self.advance();
            }
            true
        } else {
            false
        }
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while self.chars.peek().filter(|c| predicate(**c)).is_some() {
            self.advance();
        }
    }

    fn advance(&mut self) {
        if let Some(c) = self.chars.next() {
            self.index += 1;
            self.location = self.location.next(c);
        }
    }
}
