use std::ops::Range;

use crate::{
    base_buffers::inline::InlineBuffer,
    interface::{resize_error::ResizeError, Buffer},
};

use super::either::EitherBuffer;

pub struct SvoBuffer<T, B: Buffer<T>, const SMALL_SIZE: usize> {
    inner: EitherBuffer<T, InlineBuffer<T, SMALL_SIZE>, B>,
}

impl<T, B: Buffer<T>, const SMALL_SIZE: usize> Buffer<T> for SvoBuffer<T, B, SMALL_SIZE> {
    fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    unsafe fn read_value(&self, index: usize) -> T {
        self.inner.read_value(index)
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        self.inner.write_value(index, value)
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        self.inner.manually_drop(index)
    }

    unsafe fn manually_drop_range(&mut self, values_range: Range<usize>) {
        self.inner.manually_drop_range(values_range)
    }
    unsafe fn try_grow(&mut self, _target: usize) -> Result<(), ResizeError> {
        todo!()
    }
    unsafe fn try_shrink(&mut self, _target: usize) -> Result<(), ResizeError> {
        match self.inner {
            EitherBuffer::First(_) => todo!(),
            EitherBuffer::Second(_) => todo!(),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }
}
