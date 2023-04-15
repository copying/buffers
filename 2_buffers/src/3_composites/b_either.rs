use std::marker::PhantomData;

use crate::interface::Buffer;

pub enum Never {}
pub enum EitherBuffer<T, A: Buffer<T>, B: Buffer<T>> {
    First(A),
    Second(B),
    _InternalMarker(Never, PhantomData<T>),
}
