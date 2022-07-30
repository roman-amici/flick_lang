use crate::common::span_identifier::SpanIdentifier;
use crate::interpreter::code::byte_code_value::ByteCodeValue;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum ByteCodeOp {
    Return(usize),   // (offset)
    Constant(usize), // (constant_index)
}

pub struct ByteCodeChunk {
    ops: Vec<ByteCodeOp>,
    constants: Vec<ByteCodeValue>,
    spans: Vec<Rc<SpanIdentifier>>,
}

impl ByteCodeChunk {
    pub fn new() -> Self {
        ByteCodeChunk {
            ops: vec![],
            constants: vec![],
            spans: vec![],
        }
    }

    pub fn add_constant(&mut self, value: ByteCodeValue) {
        self.constants.push(value);
    }
}
