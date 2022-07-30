use crate::gc::gc_object::GCObject;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct uGc<T: GCObject> {
    pub marked: bool,
    pub ptr: *mut T,
}

impl<T: GCObject> uGc<T> {
    pub fn free(&mut self) {
        use std::alloc::dealloc;
        use std::alloc::Layout;
        use std::ptr::drop_in_place;
        unsafe {
            drop_in_place(self.ptr);
            dealloc(self.ptr as *mut u8, Layout::new::<T>());
        }
    }

    pub fn new(a: T) -> uGc<T> {
        uGc {
            marked: false,
            ptr: Box::into_raw(Box::new(a)),
        }
    }
}

impl<'a, T: GCObject> Deref for uGc<T> {
    type Target = T;
    fn deref(&self) -> &<Self>::Target {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl<'a, T: GCObject> DerefMut for uGc<T> {
    fn deref_mut(&mut self) -> &mut <Self>::Target {
        unsafe { self.ptr.as_mut().unwrap() }
    }
}

impl<T: GCObject> GCObject for uGc<T> {
    fn mark(&mut self, to_sweep: &mut Vec<usize>) {
        if !self.marked {
            self.marked = true;
            to_sweep.push(self.ptr as usize);
            self.deref_mut().mark(to_sweep);
        }
    }
}

impl<T: GCObject> Clone for uGc<T> {
    fn clone(&self) -> Self {
        uGc {
            marked: self.marked,
            ptr: self.ptr,
        }
    }
}
