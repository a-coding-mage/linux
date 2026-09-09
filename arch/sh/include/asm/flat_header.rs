/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/flat.h
 *
 * uClinux flat-format executables
 *
 * Copyright (C) 2003  Paul Mundt
 */

// The original header depends on Linux's unaligned access helpers.

#[inline]
pub unsafe fn flat_get_addr_from_rp(
    rp: *mut u32,
    _relval: u32,
    _flags: u32,
    addr: *mut u32,
) -> i32 {
    *addr = core::ptr::read_unaligned(rp as *const u32);
    0
}

#[inline]
pub unsafe fn flat_put_addr_at_rp(rp: *mut u32, addr: u32, _rel: u32) -> i32 {
    core::ptr::write_unaligned(rp, addr);
    0
}

#[macro_export]
macro_rules! FLAT_PLAT_INIT {
    ($r:expr) => {{
        $r.regs[0] = 0;
        $r.regs[1] = 0;
        $r.regs[2] = 0;
        $r.regs[3] = 0;
        $r.regs[4] = 0;
        $r.regs[5] = 0;
        $r.regs[6] = 0;
        $r.regs[7] = 0;
        $r.regs[8] = 0;
        $r.regs[9] = 0;
        $r.regs[10] = 0;
        $r.regs[11] = 0;
        $r.regs[12] = 0;
        $r.regs[13] = 0;
        $r.regs[14] = 0;
        $r.sr = SR_FD;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
