use crate::Location;
use std::fmt;

#[derive(Debug)]
pub struct Located<T: fmt::Debug> {
    pub location: Location,
    pub data: T,
}

impl<T: fmt::Debug> Located<T> {
    pub fn new(location: Location, data: T) -> Located<T> {
        Located {
            location,
            data
        }
    }
}
