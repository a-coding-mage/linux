/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the m68k assembly hash implementation.
//
// If CONFIG_M68000=y (original mc68000/010), this file is included to work
// around the lack of a MULU.L instruction.

pub const HAVE_ARCH__HASH_32: u32 = 1;

/*
 * While it would be legal to substitute a different hash operation
 * entirely, keep it simple and use an optimized multiply by
 * GOLDEN_RATIO_32 = 0x61C88647.
 *
 * The original implementation uses one large assembly block to control the
 * exact m68k instruction sequence and avoid slow long shifts.
 */
#[inline]
pub unsafe fn __hash_32(x: u32) -> u32 {
    // Equivalent to the original m68k addition/shift chain:
    // a+b = x * 0x8647, modulo 2^32.
    let mut a = x.wrapping_shl(2);
    let mut b = a;
    a = a.wrapping_shl(7);
    a = a.wrapping_add(x);
    b = b.wrapping_add(a);
    a = a.wrapping_add(a);
    b = b.wrapping_add(a);
    a = a.wrapping_shl(5);

    (lower_16_bits(x.wrapping_mul(0x61c8)) << 16)
        .wrapping_add(a)
        .wrapping_add(b)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
