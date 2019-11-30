use crate::{Location, TokenKind};

///
/// A `Token` is an individual lexeme in source file that has been identified as having
/// particular importance to the mayim compiler.
///
/// It has a type or `kind`, which informs us what this lexeme represents, and a `location`, which
/// tells us where, in which source file this lexeme came from.
///
#[derive(Debug, Clone)]
pub struct Token {
    ///
    /// Where in which source file this `Token` was found.
    ///
    pub location: Location,
    ///
    /// The type, or `kind` of token this is.
    ///
    pub kind: TokenKind,
}

impl Token {
    pub fn new(location: Location, kind: TokenKind) -> Token {
        Token { location, kind }
    }
}
