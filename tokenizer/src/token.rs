use string_cache::DefaultAtom as Atom;

///
/// A `Token` is an individual lexeme in source file that has been identified as having
/// particular importance to the mayim compiler.
///
/// Certain `Token`s contain an Atom which can be used to distinguish one such token from another.
/// For instance `Token::Identifier(_)` contains an `Atom` which contains the particular
/// identifier that this token contains.
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    EndOfFile,
    Unknown(Atom),
    Identifier(Atom),
    Integer(Atom),
    Decimal(Atom),
    Boolean(Atom),
    Let,
    Assign,
}
