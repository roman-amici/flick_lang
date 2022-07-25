use crate::lexer::token::SpanIdentifier;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

// TODO: display result
pub struct LexerError {
    pub id: SpanIdentifier,
    pub message: String,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
