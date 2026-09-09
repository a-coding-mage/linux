// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019
 * Author(s): Giulio Benetti <giulio.benetti@benettiengineering.com>
 */

// Dependencies supplied by the kernel architecture headers:
// linux/kernel.h, asm/mach/arch.h, and asm/v7m.h.

use core::ffi::c_char;

// `__initconst` places this data in the kernel's init-const section.
#[used]
#[link_section = ".init.rodata"]
static IMXRT_COMPAT: [*const c_char; 2] = [
    b"fsl,imxrt1050\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(IMXRTDT, "IMXRT (Device Tree Support)")
//
// The following machine descriptor fields are supplied by the kernel's
// architecture-specific machine descriptor definitions:
//     .dt_compat = imxrt_compat,
//     .restart = armv7m_restart,
// MACHINE_END
//
// Preserve the macro-generated registration as an external kernel item.
extern "C" {
    static IMXRTDT: core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
