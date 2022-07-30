pub trait GCObject {
    fn mark(&mut self, to_mark: &mut Vec<usize>) {}
}

// Base Types
impl GCObject for u64 {}
impl GCObject for usize {}
impl GCObject for i64 {}
impl GCObject for String {}
impl GCObject for bool {}
