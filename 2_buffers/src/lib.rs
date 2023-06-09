#![feature(dropck_eyepatch)]
#![feature(maybe_uninit_uninit_array)]
#![cfg_attr(feature = "allocator", feature(allocator_api))]

use base_buffers::heap::HeapBuffer;
use composites::{svo::SvoBuffer, zsto::ZstOptBuffer};

#[path = "1_interface/_mod.rs"]
pub mod interface;

#[path = "2_base_buffers/_mod.rs"]
pub mod base_buffers;

#[path = "3_composites/_mod.rs"]
pub mod composites;

pub type DefaultBuffer<T, const SMALL_VECTOR_SIZE: usize = 256> =
    ZstOptBuffer<T, SvoBuffer<T, HeapBuffer<T>, SMALL_VECTOR_SIZE>>;
