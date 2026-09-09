/* SPDX-License-Identifier: GPL-2.0 */
/*
 * traps.h:  Format of entries for the Sparc trap table.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependency translated from: #include <uapi/asm/traps.h>

// This is for V8 compliant Sparc CPUS
#[repr(C)]
pub struct tt_entry {
    pub inst_one: core::ffi::c_ulong,
    pub inst_two: core::ffi::c_ulong,
    pub inst_three: core::ffi::c_ulong,
    pub inst_four: core::ffi::c_ulong,
}

// We set this to _start in system setup.
extern "C" {
    pub static mut sparc_ttable: *mut tt_entry;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
