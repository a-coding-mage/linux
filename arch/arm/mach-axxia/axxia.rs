// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Support for the LSI Axxia SoC devices based on ARM cores.
 *
 * Copyright (C) 2012 LSI
 */

// C dependencies: <linux/init.h>, <asm/mach/arch.h>

// DT_MACHINE_START(AXXIA_DT, "LSI Axxia AXM55XX") / MACHINE_END
// are architecture-provided linker/macro declarations.  Their local Rust
// equivalent is represented by the compatibility table below; the machine
// registration is retained as dependency-provided build-time intent.

#[allow(non_upper_case_globals)]
pub static axxia_dt_match: [*const core::ffi::c_char; 4] = [
    b"lsi,axm5516\0".as_ptr() as *const core::ffi::c_char,
    b"lsi,axm5516-sim\0".as_ptr() as *const core::ffi::c_char,
    b"lsi,axm5516-emu\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
