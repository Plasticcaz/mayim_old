use tokenizer::{Atom, Location};

///
/// The fundamental larger unit of code in `mayim`. In `mayim`, everything is an expression,
/// from a simple literal to a function declaration.
///
#[derive(Debug)]
pub enum Expression {
    BindingDeclaration {
        let_keyword: Location,
        identifier: (Location, Atom),
        assign: Location,
        expression: Box<Expression>,
    },
    Literal((Location, Literal)),
    ///
    /// Error encountered when trying to parse this expression node.
    ///
    Error(String),
}

///
/// The different types of Literal exrpessions.
///
#[derive(Debug, Eq, PartialEq)]
pub enum Literal {
    Integer(Atom),
    Decimal(Atom),
    Boolean(Atom),
}
