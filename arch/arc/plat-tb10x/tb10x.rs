// SPDX-License-Identifier: GPL-2.0-only
/*
 * Abilis Systems TB10x platform initialisation
 *
 * Copyright (C) Abilis Systems 2012
 *
 * Author: Christian Ruppert <christian.ruppert@abilis.com>
 */

// C dependencies: linux/init.h and asm/mach_desc.h.
use core::ffi::c_char;

static mut TB10X_COMPAT: [*const c_char; 2] = [
    b"abilis,arc-tb10x\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// MACHINE_START(TB10x, "tb10x")
//     .dt_compat = tb10x_compat,
// MACHINE_END
// The machine-description registration is supplied by asm/mach_desc.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
