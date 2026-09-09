/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2006 Michael Ellerman, IBM Corporation.
 */

use core::ffi::{c_char, c_int};

pub unsafe extern "C" {
    pub fn print_address(memaddr: c_ulong);
}

pub type c_ulong = core::ffi::c_ulong;

/* CONFIG_XMON_DISASSEMBLY is a build-time condition from the original header. */
#[cfg(feature = "CONFIG_XMON_DISASSEMBLY")]
pub unsafe extern "C" {
    pub fn print_insn_powerpc(insn: c_ulong, memaddr: c_ulong) -> c_int;
    pub fn print_insn_spu(insn: c_ulong, memaddr: c_ulong) -> c_int;
}

#[cfg(not(feature = "CONFIG_XMON_DISASSEMBLY"))]
unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}

#[cfg(not(feature = "CONFIG_XMON_DISASSEMBLY"))]
#[inline]
pub unsafe fn print_insn_powerpc(insn: c_ulong, _memaddr: c_ulong) -> c_int {
    static FORMAT: &[u8] = b"%.8lx\0";
    unsafe {
        printf(FORMAT.as_ptr() as *const c_char, insn);
    }
    0
}

#[cfg(not(feature = "CONFIG_XMON_DISASSEMBLY"))]
#[inline]
pub unsafe fn print_insn_spu(insn: c_ulong, _memaddr: c_ulong) -> c_int {
    static FORMAT: &[u8] = b"%.8lx\0";
    unsafe {
        printf(FORMAT.as_ptr() as *const c_char, insn);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
