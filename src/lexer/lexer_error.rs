use crate::common::span_identifier::SpanIdentifier;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

// TODO: display result
pub struct LexerError {
    pub id: Rc<SpanIdentifier>,
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
