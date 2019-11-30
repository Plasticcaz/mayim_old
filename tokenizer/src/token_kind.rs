use string_cache::DefaultAtom as Atom;

///
/// A `TokenKind` is the type, or kind of `Token` that has been read from a source file.
///
/// Certain `TokenKind`s contain an Atom which can be used to distinguish one such token from another.
/// For instance `TokenKind::Identifier(_)` contains an `Atom` which contains the particular
/// identifier that this token contains.
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
    EndOfFile,
    Unknown(Atom),
    Identifier(Atom),
    Integer(Atom),
    Decimal(Atom),
    Let,
    Assign,
}
