#![feature(alloc, heap_api)]

use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use alloc::heap;

struct Unique<T> {
    ptr: *const T,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Unique<T> {}
unsafe impl<T: Sync> Sync for Unique<T> {}

impl<T> Unique<T> {
    pub fn new(ptr: *mut T) -> Self {
        Unique {
            ptr: ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Unique<T> {
    type Target = *mut T;
    fn deref(&self) -> &*mut T {
        unsafe { mem::transmute(&self.ptr) }
    }
}

struct Vec<T> {
    data: Unique<T>,
    len: usize,
    cap: usize,
}

impl <T> Vec<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "we are not ready to handle ZSTs");
        Vec{ ptr: Unique::new(heap)}
    }
}

#[cfg(test)]
mod tests {
    use super::Vec;
    #[test]
    fn it_works() {
        Vec::
    }
}
