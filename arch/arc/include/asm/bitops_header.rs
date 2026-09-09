/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// The C header guards, include directives, assembler-only guard, and generic
// bitops includes are represented by the surrounding translation unit.

#[cfg(feature = "CONFIG_ISA_ARCOMPACT")]
#[inline]
pub unsafe fn clz(x: u32) -> i32 {
    let res: u32;
    core::arch::asm!(
        "norm.f {0}, {1}",
        "mov.n {0}, 0",
        "add.p {0}, {0}, 1",
        out(reg) res,
        in(reg) x,
        options(nostack, preserves_flags),
    );
    res as i32
}

#[cfg(feature = "CONFIG_ISA_ARCOMPACT")]
#[inline]
pub const fn constant_fls(mut x: u32) -> i32 {
    let mut r: i32 = 32;
    if x == 0 { return 0; }
    if (x & 0xffff0000u32) == 0 { x <<= 16; r -= 16; }
    if (x & 0xff000000u32) == 0 { x <<= 8; r -= 8; }
    if (x & 0xf0000000u32) == 0 { x <<= 4; r -= 4; }
    if (x & 0xc0000000u32) == 0 { x <<= 2; r -= 2; }
    if (x & 0x80000000u32) == 0 { r -= 1; }
    r
}

#[cfg(feature = "CONFIG_ISA_ARCOMPACT")]
#[inline]
pub unsafe fn fls(x: u32) -> i32 {
    // __builtin_constant_p(x) is a compiler-folding condition in C.
    if x == 0 { return 0; }
    32 - clz(x)
}

#[cfg(feature = "CONFIG_ISA_ARCOMPACT")]
#[inline]
pub unsafe fn __fls(x: usize) -> usize {
    if x == 0 { 0 } else { (fls(x as u32) - 1) as usize }
}

#[cfg(feature = "CONFIG_ISA_ARCOMPACT")]
#[macro_export]
macro_rules! ffs {
    ($x:expr) => {{
        let __t: usize = $x as usize;
        unsafe { $crate::fls((__t & __t.wrapping_neg()) as u32) }
    }};
}

#[cfg(feature = "CONFIG_ISA_ARCOMPACT")]
#[inline]
pub unsafe fn __ffs(word: usize) -> usize {
    if word == 0 { word } else { (ffs!(word) - 1) as usize }
}

#[cfg(not(feature = "CONFIG_ISA_ARCOMPACT"))]
#[inline]
pub unsafe fn fls(x: u32) -> i32 {
    let n: u32;
    core::arch::asm!(
        "fls.f {0}, {1}",
        "add.nz {0}, {0}, 1",
        out(reg) n,
        in(reg) x,
        options(nostack, preserves_flags),
    );
    n as i32
}

#[cfg(not(feature = "CONFIG_ISA_ARCOMPACT"))]
#[inline]
pub unsafe fn __fls(x: usize) -> usize {
    // __builtin_constant_p(x) selects the equivalent compile-time expression.
    if x != 0 { (usize::BITS - 1 - x.leading_zeros()) as usize } else { 0 }
}

#[cfg(not(feature = "CONFIG_ISA_ARCOMPACT"))]
#[inline]
pub unsafe fn ffs(x: u32) -> i32 {
    let n: u32;
    core::arch::asm!(
        "ffs.f {0}, {1}",
        "add.nz {0}, {0}, 1",
        "mov.z {0}, 0",
        out(reg) n,
        in(reg) x,
        options(nostack, preserves_flags),
    );
    n as i32
}

#[cfg(not(feature = "CONFIG_ISA_ARCOMPACT"))]
#[inline]
pub unsafe fn __ffs(x: usize) -> usize {
    let n: usize;
    core::arch::asm!(
        "ffs.f {0}, {1}",
        "mov.z {0}, 0",
        out(reg) n,
        in(reg) x,
        options(nostack, preserves_flags),
    );
    n
}

#[inline]
pub unsafe fn ffz(x: usize) -> usize {
    __ffs(!x)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
