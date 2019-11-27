use crate::token::Token;

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
pub enum InvalidError {
    NotEnoughAtoms,
    SurplusTokens,
}

impl fmt::Display for InvalidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InvalidError {}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct InvalidTokenAsArgument(Token);

impl InvalidTokenAsArgument {
    pub fn new(arg: Token) -> Self {
        InvalidTokenAsArgument(arg)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ParseAstError {
    LexiconError(LexiconError),
    InvalidError(InvalidError),
}
