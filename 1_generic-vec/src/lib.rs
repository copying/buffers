use std::marker::PhantomData;

use buffers::base_buffers::inline::InlineBuffer;

/// Implementation of a vector
pub struct Vector<T> {
    buf: InlineBuffer<T, 1234>,
}

impl<T> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector {
            buf: Default::default(),
        }
    }

    pub fn len(self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vector_should_be_build_with_new() {
        let _vector = Vector::<u32>::new();
    }

    #[test]
    fn empty_vector_should_have_no_length() {
        assert_eq!(Vector::<u32>::new().len(), 0)
    }
}
