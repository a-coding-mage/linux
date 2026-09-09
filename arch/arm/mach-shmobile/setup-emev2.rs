// SPDX-License-Identifier: GPL-2.0
/*
 * Emma Mobile EV2 processor support
 *
 * Copyright (C) 2012  Magnus Damm
 */

// Dependencies supplied by the surrounding kernel translation unit.
use crate::common::*;
use crate::emev2::*;

// Corresponds to __initconst storage in the C source.
pub static EMEV2_BOARDS_COMPAT_DT: [&'static core::ffi::c_char; 2] = [
    b"renesas,emev2\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(EMEV2_DT, "Generic Emma Mobile EV2 (Flattened Device Tree)")
// The machine descriptor type and smp_ops helper are provided by the architecture
// support code; the following declaration preserves the descriptor's C layout and
// initialization intent.
extern "C" {
    pub static emev2_smp_ops: core::ffi::c_void;
    pub fn smp_ops(ops: *const core::ffi::c_void) -> *const core::ffi::c_void;
    pub fn shmobile_init_delay();
    pub fn shmobile_init_late();
}

#[repr(C)]
pub struct MachineDesc {
    pub smp: *const core::ffi::c_void,
    pub init_early: Option<unsafe extern "C" fn()>,
    pub init_late: Option<unsafe extern "C" fn()>,
    pub dt_compat: *const *const core::ffi::c_char,
}

pub static EMEV2_DT: MachineDesc = MachineDesc {
    smp: unsafe { smp_ops(core::ptr::addr_of!(emev2_smp_ops)) },
    init_early: Some(shmobile_init_delay),
    init_late: Some(shmobile_init_late),
    dt_compat: EMEV2_BOARDS_COMPAT_DT.as_ptr(),
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
