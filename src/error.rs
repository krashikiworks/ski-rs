use std::error::Error;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct LexiconError {
    error_point: u32,
    error_char: char,
}

impl fmt::Display for LexiconError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.error_point, self.error_char)
    }
}

impl Error for LexiconError {}

impl LexiconError {
    pub fn new(error_point: u32, error_char: char) -> Self {
        LexiconError {
            error_point,
            error_char,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum FormulaError {
    NotEnoughAtoms,
    SurplusTokens,
}

impl fmt::Display for FormulaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for FormulaError {}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum InvalidError {
    LexiconError(LexiconError),
    FormulaError(FormulaError),
}

impl fmt::Display for InvalidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LexiconError(err) => err.fmt(f),
            Self::FormulaError(err) => err.fmt(f),
        }
    }
}

impl Error for InvalidError {}

impl From<LexiconError> for InvalidError {
    fn from(e: LexiconError) -> Self {
        InvalidError::LexiconError(e)
    }
}

impl From<FormulaError> for InvalidError {
    fn from(e: FormulaError) -> Self {
        InvalidError::FormulaError(e)
    }
}
