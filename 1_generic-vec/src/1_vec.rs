use std::marker::PhantomData;

pub struct Vector<T> {
    _m: PhantomData<T>,
}

impl<T> Vector<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
