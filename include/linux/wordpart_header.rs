/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/wordpart.h. C preprocessor header guards and includes
// are not represented in Rust.

/**
 * upper_32_bits - return bits 32-63 of a number
 * @n: the number we're accessing
 *
 * A basic shift-right of a 64- or 32-bit quantity.  Use this to suppress
 * the "right shift count >= width of type" warning when that quantity is
 * 32-bits.
 */
#[macro_export]
macro_rules! upper_32_bits {
    ($n:expr) => {{ (($n >> 16) >> 16) as u32 }};
}

/**
 * lower_32_bits - return bits 0-31 of a number
 * @n: the number we're accessing
 */
#[macro_export]
macro_rules! lower_32_bits {
    ($n:expr) => {{ (($n & 0xffff_ffff) as u32) }};
}

/**
 * upper_16_bits - return bits 16-31 of a number
 * @n: the number we're accessing
 */
#[macro_export]
macro_rules! upper_16_bits {
    ($n:expr) => {{ (($n >> 16) as u16) }};
}

/**
 * lower_16_bits - return bits 0-15 of a number
 * @n: the number we're accessing
 */
#[macro_export]
macro_rules! lower_16_bits {
    ($n:expr) => {{ (($n & 0xffff) as u16) }};
}

/**
 * REPEAT_BYTE - repeat the value @x multiple times as an unsigned long value
 * @x: value to repeat
 *
 * NOTE: @x is not checked for > 0xff; larger values produce odd results.
 */
#[macro_export]
macro_rules! REPEAT_BYTE {
    ($x:expr) => {{
        ((usize::MAX / 0xffusize).wrapping_mul($x as usize))
    }};
}

/**
 * REPEAT_BYTE_U32 - repeat the value @x multiple times as a u32 value
 * @x: value to repeat
 *
 * NOTE: @x is not checked for > 0xff; larger values produce odd results.
 */
#[macro_export]
macro_rules! REPEAT_BYTE_U32 {
    ($x:expr) => {{ $crate::lower_32_bits!($crate::REPEAT_BYTE!($x)) }};
}

/* Set bits in the first 'n' bytes when loaded from memory */
#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! aligned_byte_mask {
    ($n:expr) => {{ (1usize << (8 * ($n))) - 1 }};
}

#[cfg(not(target_endian = "little"))]
#[macro_export]
macro_rules! aligned_byte_mask {
    ($n:expr) => {{
        !0xffusize << (usize::BITS - 8 - 8 * ($n))
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
