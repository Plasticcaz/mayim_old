use std::fmt;
use string_cache::DefaultAtom as Atom;

///
/// A `Location` contains all the data needed to identify where in which source file some item is.
///
#[derive(Debug, Clone)]
pub struct Location {
    pub filename: Atom,
    pub line: u32,
    pub column: u32,
}

impl Location {
    pub fn new(filename: Atom, line: u32, column: u32) -> Self {
        Location {
            filename,
            line,
            column,
        }
    }

    ///
    /// Generate a location based on the character that presumably has just been read.
    ///
    pub(crate) fn next(&self, c: char) -> Location {
        let (line, column) = match c {
            '\n' => (self.line + 1, 1),
            _ => (self.line, self.column + 1),
        };
        Location::new(self.filename.clone(), line, column)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_format_properly() {
        let location = Location::new(Atom::from("test.mayim"), 1, 2);

        assert_eq!(format!("{}", location), "test.mayim:1:2")
    }

    #[test]
    fn should_get_next_location_based_on_char() {
        let original = Location::new(Atom::from("test.mayim"), 1, 2);

        let read_non_newline = original.next('b');
        assert_eq!(read_non_newline.filename, original.filename);
        assert_eq!(read_non_newline.line, 1);
        assert_eq!(read_non_newline.column, 3);

        let read_newline = original.next('\n');
        assert_eq!(read_newline.filename, original.filename);
        assert_eq!(read_newline.line, 2);
        assert_eq!(read_newline.column, 1);
    }
}
