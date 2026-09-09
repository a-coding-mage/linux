/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2021 Cadence Design Systems Inc. */

/*
 *  System call like services offered by the GDBIO host.
 *
 * The original C header uses target-specific Xtensa register assignments and
 * inline assembly.  The Rust translation preserves those assignments.
 */

pub const SYS_open: i32 = -2;
pub const SYS_close: i32 = -3;
pub const SYS_read: i32 = -4;
pub const SYS_write: i32 = -5;
pub const SYS_lseek: i32 = -6;

static mut errno: i32 = 0;

#[inline]
pub unsafe fn __simc(a: i32, b: i32, c: i32, d: i32) -> i32 {
    let mut a1 = a;
    let b1 = b;
    let mut c1 = c;
    let d1 = d;

    core::arch::asm!(
        "break 1, 14",
        inout("a2") a1,
        in("a6") b1,
        inout("a3") c1,
        in("a4") d1,
        options(nostack)
    );

    errno = c1;
    a1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
