/* SPDX-License-Identifier: GPL-2.0 */

#[inline(always)]
pub unsafe fn __arch_bitrev32(mut x: u32) -> u32 {
    core::arch::asm!("rbit {0}, {1}", out(reg) x, in(reg) x);
    x
}

#[inline(always)]
pub unsafe fn __arch_bitrev16(x: u16) -> u16 {
    (__arch_bitrev32(x as u32) >> 16) as u16
}

#[inline(always)]
pub unsafe fn __arch_bitrev8(x: u8) -> u8 {
    (__arch_bitrev32(x as u32) >> 24) as u8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
