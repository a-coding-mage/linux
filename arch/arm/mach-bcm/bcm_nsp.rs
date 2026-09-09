// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2015 Broadcom Corporation

use core::ffi::c_char;

// Equivalent of the C __initconst compatibility table.
static BCM_NSP_DT_COMPAT: [*const c_char; 2] = [
    b"brcm,nsp\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(NSP_DT, "Broadcom Northstar Plus SoC") ... MACHINE_END
// The machine-description type and registration mechanism are supplied by
// asm/mach/arch.h and the surrounding kernel build.
#[repr(C)]
struct MachineDesc {
    l2c_aux_val: u32,
    l2c_aux_mask: u32,
    dt_compat: *const *const c_char,
}

static NSP_DT: MachineDesc = MachineDesc {
    l2c_aux_val: 0,
    l2c_aux_mask: !0u32,
    dt_compat: BCM_NSP_DT_COMPAT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
