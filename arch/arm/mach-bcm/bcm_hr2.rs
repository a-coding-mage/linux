// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2017 Broadcom

use core::ffi::c_char;

// Corresponds to the C `__initconst` annotation.
#[cfg_attr(any(), link_section = ".init.rodata")]
static BCM_HR2_DT_COMPAT: [*const c_char; 2] = [
    b"brcm,hr2\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(BCM_HR2_DT, "Broadcom Hurricane 2 SoC")
//	.l2c_aux_val	= 0,
//	.l2c_aux_mask	= ~0,
//	.dt_compat = bcm_hr2_dt_compat,
// MACHINE_END
//
// The machine-registration macros and their descriptor type are supplied by
// the architecture dependencies.
extern "C" {
    static BCM_HR2_DT: core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
