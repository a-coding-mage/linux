/* SPDX-License-Identifier: GPL-2.0 */

//! Safe, constant-evaluable counterparts of the C bit-reversal header.
//!
//! C's constant, generic-table and architecture paths have the same result.
//! These helpers express that operation directly in Rust; architecture inline
//! functions and `swab32` are not external C symbols. Arguments are evaluated
//! once and results retain their original widths. Cast wider inputs to the
//! corresponding unsigned width, as the C macros' local variables do.

/// Reverses all 32 bits with the original constant-expression semantics.
#[inline]
pub const fn __constant_bitrev32(mut x: u32) -> u32 {
    x = (x >> 16) | (x << 16);
    x = ((x & 0xFF00FF00u32) >> 8) | ((x & 0x00FF00FFu32) << 8);
    x = ((x & 0xF0F0F0F0u32) >> 4) | ((x & 0x0F0F0F0Fu32) << 4);
    x = ((x & 0xCCCCCCCCu32) >> 2) | ((x & 0x33333333u32) << 2);
    ((x & 0xAAAAAAAAu32) >> 1) | ((x & 0x55555555u32) << 1)
}

/// Reverses all 16 bits with the original constant-expression semantics.
#[inline]
pub const fn __constant_bitrev16(mut x: u16) -> u16 {
    x = (x >> 8) | (x << 8);
    x = ((x & 0xF0F0u16) >> 4) | ((x & 0x0F0Fu16) << 4);
    x = ((x & 0xCCCCu16) >> 2) | ((x & 0x3333u16) << 2);
    ((x & 0xAAAAu16) >> 1) | ((x & 0x5555u16) << 1)
}

/// Reverses bits within each byte, preserving the four byte positions.
#[inline]
pub const fn __constant_bitrev8x4(mut x: u32) -> u32 {
    x = ((x & 0xF0F0F0F0u32) >> 4) | ((x & 0x0F0F0F0Fu32) << 4);
    x = ((x & 0xCCCCCCCCu32) >> 2) | ((x & 0x33333333u32) << 2);
    ((x & 0xAAAAAAAAu32) >> 1) | ((x & 0x55555555u32) << 1)
}

/// Reverses all eight bits with the original constant-expression semantics.
#[inline]
pub const fn __constant_bitrev8(mut x: u8) -> u8 {
    x = (x >> 4) | (x << 4);
    x = ((x & 0xCCu8) >> 2) | ((x & 0x33u8) << 2);
    ((x & 0xAAu8) >> 1) | ((x & 0x55u8) << 1)
}

/// Rust counterpart of the C architecture/generic 32-bit helper alias.
#[inline]
pub const fn __bitrev32(x: u32) -> u32 {
    __constant_bitrev32(x)
}

/// Rust counterpart of the C architecture/generic 16-bit helper alias.
#[inline]
pub const fn __bitrev16(x: u16) -> u16 {
    __constant_bitrev16(x)
}

/// Rust counterpart of the C architecture/generic byte helper alias.
#[inline]
pub const fn __bitrev8(x: u8) -> u8 {
    __constant_bitrev8(x)
}

/// Rust counterpart of reversing a byte-swapped word's bits.
#[inline]
pub const fn __bitrev8x4(x: u32) -> u32 {
    __constant_bitrev8x4(x)
}

/// Reverses every bit of a word, at runtime or during constant evaluation.
#[inline]
pub const fn bitrev32(x: u32) -> u32 {
    __constant_bitrev32(x)
}

/// Reverses every bit of a halfword, at runtime or during constant evaluation.
#[inline]
pub const fn bitrev16(x: u16) -> u16 {
    __constant_bitrev16(x)
}

/// Reverses each byte's bits without exchanging the four bytes.
#[inline]
pub const fn bitrev8x4(x: u32) -> u32 {
    __constant_bitrev8x4(x)
}

/// Reverses every bit of a byte, at runtime or during constant evaluation.
#[inline]
pub const fn bitrev8(x: u8) -> u8 {
    __constant_bitrev8(x)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
