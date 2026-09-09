/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the SuperH 32-bit bit-manipulation header.

#[inline]
pub unsafe fn set_bl_bit() {
    let mut __dummy0: usize;
    let mut __dummy1: usize;

    core::arch::asm!(
        "stc sr, {0}",
        "or {2}, {0}",
        "and {3}, {0}",
        "ldc {0}, sr",
        lateout(reg) __dummy0,
        lateout(reg) __dummy1,
        in(reg) 0x10000000usize,
        in(reg) 0xffffff0fusize,
        options(nostack),
    );
}

#[inline]
pub unsafe fn clear_bl_bit() {
    let mut __dummy0: usize;
    let mut __dummy1: usize;

    core::arch::asm!(
        "stc sr, {0}",
        "and {2}, {0}",
        "ldc {0}, sr",
        lateout(reg) __dummy0,
        lateout(reg) __dummy1,
        in(reg) (!0x10000000usize),
        options(nostack),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
