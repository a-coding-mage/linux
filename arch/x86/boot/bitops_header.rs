/* SPDX-License-Identifier: GPL-2.0-only */
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/*
 * Very simple bitops for the boot code.
 *
 * The C header includes <linux/types.h> and <asm/asm.h>; their Rust
 * equivalents are supplied by the surrounding translation unit.
 */

#[inline]
pub unsafe fn constant_test_bit(nr: i32, addr: *const core::ffi::c_void) -> bool {
    let p = addr as *const u32;
    ((1u32.wrapping_shl((nr & 31) as u32)) & *p.add((nr >> 5) as usize)) != 0
}

#[inline]
pub unsafe fn variable_test_bit(nr: i32, addr: *const core::ffi::c_void) -> bool {
    let p = addr as *const u32;
    let mut v: u8;

    core::arch::asm!(
        "btl {nr}, [{ptr}]",
        "setc {v}",
        nr = in(reg) nr,
        ptr = in(reg) p,
        v = lateout(reg_byte) v,
        options(nostack, preserves_flags),
    );
    v != 0
}

/* Rust has no direct equivalent of GCC's __builtin_constant_p. */
#[macro_export]
macro_rules! test_bit {
    ($nr:expr, $addr:expr) => {{
        $crate::variable_test_bit($nr, $addr)
    }};
}

#[inline]
pub unsafe fn set_bit(nr: i32, addr: *mut core::ffi::c_void) {
    let p = addr as *mut u32;

    core::arch::asm!(
        "btsl {nr}, [{ptr}]",
        nr = in(reg) nr,
        ptr = in(reg) p,
        options(nostack, preserves_flags),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
