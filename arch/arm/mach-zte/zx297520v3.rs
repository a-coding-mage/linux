// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2026 Stefan Dösinger
 */

// Dependencies supplied by the kernel headers:
// <asm/mach/arch.h>
// <linux/init.h>

use core::ffi::c_char;

/// Device-tree compatibility strings for the ZTE zx297520v3 machine.
#[allow(non_upper_case_globals)]
pub static zx297520v3_dt_compat: [*const c_char; 2] = [
    b"zte,zx297520v3\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// C macro translation:
// DT_MACHINE_START(ZX, "ZTE zx297520v3 (Device Tree)")
//     .dt_compat = zx297520v3_dt_compat,
// MACHINE_END
//
// The DT_MACHINE_START/MACHINE_END macros define the architecture-specific
// kernel machine descriptor and its registration; their generated type and
// linker registration are supplied by the external kernel environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
