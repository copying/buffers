use std::ops::Bound::*;
use std::ops::RangeBounds;

use super::Buffer;

pub trait BufferShift: Buffer {
    /// Shift a range of values to the right.
    /// # Safety
    /// The values must exist and the new location should be itself or an empty spot
    ///
    /// There should be enough space to the right
    unsafe fn shift_right<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize);

    /// Shift a range of values to the left.
    ///
    /// # Safety
    /// The values must exist and the new location should be itself or an empty spot
    ///
    /// There should be enough space to the left
    unsafe fn shift_left<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize);
}

pub trait ShiftOneByOne: Buffer {}
impl<T: ShiftOneByOne> BufferShift for T {
    unsafe fn shift_right<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {}

    unsafe fn shift_left<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        todo!()
    }
}

fn start_end<B: Buffer + ?Sized, R: RangeBounds<usize>>(buffer: &B, range: R) -> (usize, usize) {
    let start: usize = match range.start_bound() {
        Included(index) => *index,
        Excluded(index) => *index + 1,
        Unbounded => 0,
    };
    let end: usize = match range.end_bound() {
        Included(index) => *index + 1,
        Excluded(index) => *index,
        Unbounded => buffer.capacity(),
    };
    (start, end)
}
