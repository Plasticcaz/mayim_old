use crate::{Atom, AtomToken, Location};

///
/// The fundamental larger unit of code in `mayim`. In `mayim`, everything is an expression,
/// from a simple literal to a function declaration.
///

#[derive(Debug)]
pub enum Expression {
    Identifier(AtomToken),
    IntegerLiteral(AtomToken),
    DecimalLiteral(AtomToken),
    Error(Error),
}

impl Expression {
    fn location(&self) -> Location {
        use Expression::*;
        match self {
            IntegerLiteral(token)
            | DecimalLiteral(token)
            | Identifier(token) => token.location.clone(),
            Error(error) => error.location.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub location: Location,
    pub message: String,
}

impl Error {
    pub fn new<Msg: Into<String>>(location: Location, message: Msg) -> Error {
        Error {
            location,
            message: message.into(),
        }
    }
}
