// SPDX-License-Identifier: GPL-2.0
/*
 * r7s9210 processor support
 *
 * Copyright (C) 2018  Renesas Electronics Corporation
 * Copyright (C) 2018  Chris Brandt
 *
 */

use core::ffi::c_char;

// C dependencies supplied by the kernel build.
unsafe extern "C" {
    fn shmobile_init_delay();
    fn shmobile_init_late();
}

static R7S9210_BOARDS_COMPAT_DT: [*const c_char; 2] = [
    c"renesas,r7s9210".as_ptr(),
    core::ptr::null(),
];

#[repr(C)]
pub struct MachineDesc {
    pub l2c_aux_val: u32,
    pub l2c_aux_mask: u32,
    pub init_early: Option<unsafe extern "C" fn()>,
    pub init_late: Option<unsafe extern "C" fn()>,
    pub dt_compat: *const *const c_char,
}

#[used]
#[no_mangle]
pub static R7S72100_DT: MachineDesc = MachineDesc {
    l2c_aux_val: 0,
    l2c_aux_mask: u32::MAX,
    init_early: Some(shmobile_init_delay),
    init_late: Some(shmobile_init_late),
    dt_compat: R7S9210_BOARDS_COMPAT_DT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
