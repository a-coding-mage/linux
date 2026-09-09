/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependency supplied by the corresponding Linux swab translation.

#[inline(always)]
pub unsafe fn __arch_bitrev32(x: u32) -> u32 {
    let ret: u32;

    core::arch::asm!(
        "bitrev.w {0}, {1}",
        out(reg) ret,
        in(reg) x,
    );
    ret
}

#[inline(always)]
pub unsafe fn __arch_bitrev16(x: u16) -> u16 {
    let ret: u16;

    core::arch::asm!(
        "bitrev.4b {0}, {1}",
        out(reg) ret,
        in(reg) __swab16(x),
    );
    ret
}

#[inline(always)]
pub unsafe fn __arch_bitrev8(x: u8) -> u8 {
    let ret: u8;

    core::arch::asm!(
        "bitrev.4b {0}, {1}",
        out(reg) ret,
        in(reg) x,
    );
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
