// SPDX-License-Identifier: GPL-2.0
// The C source includes ../cpuflags.c; its declarations and definitions are
// supplied by the surrounding translation unit.

use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct Cpu {
    pub flags: [c_ulong; 1],
}

unsafe extern "C" {
    pub fn get_cpuflags();
    pub fn test_bit(flag: c_int, addr: *const c_ulong) -> bool;
    pub static mut cpu: Cpu;
}

pub unsafe fn has_cpuflag(flag: c_int) -> bool {
    get_cpuflags();

    test_bit(flag, cpu.flags.as_ptr())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
