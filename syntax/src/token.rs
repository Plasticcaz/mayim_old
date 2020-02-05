use crate::Location;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Clone)]
pub enum Token {
    EndOfFile(Location),
    Unknown(AtomToken),
    Identifier(AtomToken),
    Boolean(AtomToken),
    Integer(AtomToken),
    Decimal(AtomToken),
    Let(Location),
    Assign(Location),
}

impl Token {
    pub fn location(&self) -> &Location {
        match self {
            Token::EndOfFile(location)
            | Token::Let(location)
            | Token::Assign(location) => location,
            Token::Unknown(token)
            | Token::Boolean(token)
            | Token::Integer(token)
            | Token::Decimal(token)
            | Token::Identifier(token) => &token.location,
        }
    }

    pub fn description(&self) -> String {
        match self {
            Token::EndOfFile(_) => "EOF",
            Token::Let(_) => "let",
            Token::Assign(_) => "assignment",
            _ => todo!(),
        }
        .into()
    }
}

#[derive(Debug, Clone)]
pub struct AtomToken {
    pub location: Location,
    pub atom: Atom,
}

impl AtomToken {
    pub fn new(location: Location, atom: Atom) -> AtomToken {
        AtomToken { location, atom }
    }
}
