use std::mem;

use super::{
    HEAP_MASK,
    MAX_SIZE,
};

mod arc;
use arc::ArcString;

const PADDING_SIZE: usize = MAX_SIZE - mem::size_of::<ArcString>();
const PADDING: [u8; PADDING_SIZE] = [HEAP_MASK; PADDING_SIZE];

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HeapString {
    padding: [u8; PADDING_SIZE],
    pub string: ArcString,
}

impl HeapString {
    pub fn new(text: &str) -> Self {
        let padding = PADDING;
        let string = text.into();

        HeapString { padding, string }
    }
}

impl From<String> for HeapString {
    fn from(s: String) -> Self {
        let padding = PADDING;
        let string = s.as_str().into();

        HeapString { padding, string }
    }
}

static_assertions::assert_eq_size!(HeapString, String);
