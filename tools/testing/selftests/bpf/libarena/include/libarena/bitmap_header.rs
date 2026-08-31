// Translated from bitmap.h.
// C header guard / pragma once omitted.

pub const BITS_PER_BYTE: usize = 8;

#[inline]
pub const fn BYTES_TO_BITS(nb: usize) -> usize {
    nb * BITS_PER_BYTE
}

pub const BITS_PER_LONG_LONG: usize = core::mem::size_of::<i64>() * BITS_PER_BYTE;

#[inline]
pub const fn BITS_TO_LONG_LONGS(nr: usize) -> usize {
    (nr + BITS_PER_LONG_LONG - 1) / BITS_PER_LONG_LONG
}

#[inline]
pub const fn BIT_MASK(nr: usize) -> u64 {
    1u64 << (nr % BITS_PER_LONG_LONG)
}

#[inline]
pub const fn BIT_WORD(nr: usize) -> usize {
    nr / BITS_PER_LONG_LONG
}

#[repr(C)]
pub struct arena_bitmap {
    pub bits: [u64; 0],
}

// The C declarations use the address-space/type qualifier "__arena" on
// arena_bitmap pointers. Rust has no direct file-local equivalent here, so the
// raw pointer type is preserved and the qualifier intent is noted by comment.
unsafe extern "C" {
    pub fn bmp_alloc(bits: usize) -> *mut arena_bitmap;
    pub fn bmp_free(bmp: *mut arena_bitmap);

    pub fn __bmp_set_bit(bit: u32, bmp: *mut arena_bitmap);
    pub fn __bmp_clear_bit(bit: u32, bmp: *mut arena_bitmap);
    pub fn bmp_set_bit(bit: u32, bmp: *mut arena_bitmap);
    pub fn bmp_clear_bit(bit: u32, bmp: *mut arena_bitmap);
    pub fn bmp_test_bit(bit: u32, bmp: *mut arena_bitmap) -> bool;
    pub fn bmp_test_and_clear_bit(bit: u32, bmp: *mut arena_bitmap) -> bool;
    pub fn bmp_test_and_set_bit(bit: u32, bmp: *mut arena_bitmap) -> bool;

    pub fn bmp_clear(bits: usize, bmp: *mut arena_bitmap);
    pub fn bmp_and(
        bits: usize,
        dst: *mut arena_bitmap,
        src1: *mut arena_bitmap,
        src2: *mut arena_bitmap,
    );
    pub fn bmp_or(
        bits: usize,
        dst: *mut arena_bitmap,
        src1: *mut arena_bitmap,
        src2: *mut arena_bitmap,
    );
    pub fn bmp_empty(bits: usize, bmp: *mut arena_bitmap) -> bool;
    pub fn bmp_copy(bits: usize, dst: *mut arena_bitmap, src: *mut arena_bitmap);

    pub fn bmp_intersects(bits: usize, arg1: *mut arena_bitmap, arg2: *mut arena_bitmap) -> bool;
    pub fn bmp_subset(bits: usize, big: *mut arena_bitmap, small: *mut arena_bitmap) -> bool;
    pub fn bmp_print(bits: usize, bmp: *mut arena_bitmap);
}
