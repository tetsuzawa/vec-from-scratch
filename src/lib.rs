struct Vec<T> {
    data: *const T,
    len: usize,
    cap: usize,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        unimplemented!();
    }
}
