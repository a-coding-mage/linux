/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <linux/swab.h>; __swab32 and __swab16 are supplied by
// the corresponding Rust dependency.

#[inline(always)]
#[must_use]
fn __arch_bitrev32(x: u32) -> u32 {
    let mut ret: u32;

    unsafe {
        core::arch::asm!(
            "bitswap {ret}, {input}",
            ret = out(reg) ret,
            input = in(reg) __swab32(x),
        );
    }
    ret
}

#[inline(always)]
#[must_use]
fn __arch_bitrev16(x: u16) -> u16 {
    let mut ret: u16;

    unsafe {
        core::arch::asm!(
            "bitswap {ret}, {input}",
            ret = out(reg) ret,
            input = in(reg) __swab16(x),
        );
    }
    ret
}

#[inline(always)]
#[must_use]
fn __arch_bitrev8(x: u8) -> u8 {
    let mut ret: u8;

    unsafe {
        core::arch::asm!(
            "bitswap {ret}, {input}",
            ret = out(reg) ret,
            input = in(reg) x,
        );
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
