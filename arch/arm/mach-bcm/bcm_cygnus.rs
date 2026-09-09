// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2014 Broadcom Corporation

// Dependency supplied by the architecture support code:
// use asm::mach::arch::*;

/// Device-tree compatibility strings for the Broadcom Cygnus machine.
#[used]
pub static BCM_CYGNUS_DT_COMPAT: [Option<&'static core::ffi::CStr>; 2] = [
    Some(unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"brcm,cygnus\0") }),
    None,
];

// Translation of:
// DT_MACHINE_START(BCM_CYGNUS_DT, "Broadcom Cygnus SoC")
//     .l2c_aux_val  = 0,
//     .l2c_aux_mask = ~0,
//     .dt_compat    = bcm_cygnus_dt_compat,
// MACHINE_END
//
// The DT_MACHINE_START/MACHINE_END declarations are architecture-provided
// linker/macro infrastructure and remain an external dependency here.
#[allow(dead_code)]
pub const BCM_CYGNUS_DT_NAME: &str = "Broadcom Cygnus SoC";
#[allow(dead_code)]
pub const BCM_CYGNUS_DT_L2C_AUX_VAL: u32 = 0;
#[allow(dead_code)]
pub const BCM_CYGNUS_DT_L2C_AUX_MASK: u32 = !0u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
