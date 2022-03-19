use std::{marker::PhantomData, ptr::NonNull};

struct Vec<T> {
    data: NonNull<T>,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
}

#[cfg(test)]
mod tests {
    use super::Vec;
    #[test]
    fn it_works() {}
}
