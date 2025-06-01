use std::fmt::{Display, Formatter, Result};

#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
pub enum Language {
    Java,
    Python,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
