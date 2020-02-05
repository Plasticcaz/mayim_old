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
    BindingDeclaration(Box<BindingDeclaration>),
    Error(Error),
}

impl Expression {
    fn location(&self) -> Location {
        use Expression::*;
        match self {
            IntegerLiteral(token)
            | DecimalLiteral(token)
            | Identifier(token) => token.location.clone(),
            BindingDeclaration(declaration) => declaration.let_keyword.clone(),
            Error(error) => error.location.clone(),
        }
    }
}

#[derive(Debug)]
pub struct BindingDeclaration {
    pub let_keyword: Location,
    pub identifier: AtomToken,
    pub assign_operator: Location,
    pub initialized_to: Expression,
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
