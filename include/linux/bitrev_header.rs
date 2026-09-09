/* SPDX-License-Identifier: GPL-2.0 */

// `linux/types.h` supplies the C integer types; Rust's fixed-width integers
// are used directly here.  The architecture/generic selection is a build
// configuration concern from the original header.

extern "C" {
    pub fn __arch_bitrev32(x: u32) -> u32;
    pub fn __arch_bitrev16(x: u16) -> u16;
    pub fn __arch_bitrev8(x: u8) -> u8;
    pub fn generic___bitrev32(x: u32) -> u32;
    pub fn generic___bitrev16(x: u16) -> u16;
    pub fn generic___bitrev8(x: u8) -> u8;
    pub fn swab32(x: u32) -> u32;
}

#[inline]
pub const fn __constant_bitrev32(mut x: u32) -> u32 {
    x = (x >> 16) | (x << 16);
    x = ((x & 0xFF00FF00u32) >> 8) | ((x & 0x00FF00FFu32) << 8);
    x = ((x & 0xF0F0F0F0u32) >> 4) | ((x & 0x0F0F0F0Fu32) << 4);
    x = ((x & 0xCCCCCCCCu32) >> 2) | ((x & 0x33333333u32) << 2);
    ((x & 0xAAAAAAAAu32) >> 1) | ((x & 0x55555555u32) << 1)
}

#[inline]
pub const fn __constant_bitrev16(mut x: u16) -> u16 {
    x = (x >> 8) | (x << 8);
    x = ((x & 0xF0F0u16) >> 4) | ((x & 0x0F0Fu16) << 4);
    x = ((x & 0xCCCCu16) >> 2) | ((x & 0x3333u16) << 2);
    ((x & 0xAAAAu16) >> 1) | ((x & 0x5555u16) << 1)
}

#[inline]
pub const fn __constant_bitrev8x4(mut x: u32) -> u32 {
    x = ((x & 0xF0F0F0F0u32) >> 4) | ((x & 0x0F0F0F0Fu32) << 4);
    x = ((x & 0xCCCCCCCCu32) >> 2) | ((x & 0x33333333u32) << 2);
    ((x & 0xAAAAAAAAu32) >> 1) | ((x & 0x55555555u32) << 1)
}

#[inline]
pub const fn __constant_bitrev8(mut x: u8) -> u8 {
    x = (x >> 4) | (x << 4);
    x = ((x & 0xCCu8) >> 2) | ((x & 0x33u8) << 2);
    ((x & 0xAAu8) >> 1) | ((x & 0x55u8) << 1)
}

#[inline]
pub unsafe fn __bitrev8x4(x: u32) -> u32 {
    __arch_bitrev32(swab32(x))
}

#[inline]
pub unsafe fn bitrev32(x: u32) -> u32 {
    __arch_bitrev32(x)
}

#[inline]
pub unsafe fn bitrev16(x: u16) -> u16 {
    __arch_bitrev16(x)
}

#[inline]
pub unsafe fn bitrev8x4(x: u32) -> u32 {
    __bitrev8x4(x)
}

#[inline]
pub unsafe fn bitrev8(x: u8) -> u8 {
    __arch_bitrev8(x)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
