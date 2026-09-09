// SPDX-License-Identifier: GPL-2.0
/*
 * r7s72100 processor support
 *
 * Copyright (C) 2013  Renesas Solutions Corp.
 * Copyright (C) 2013  Magnus Damm
 */

// Dependency intent from the C source:
//   <linux/kernel.h>, <asm/mach/arch.h>, and "common.h"

unsafe extern "C" {
    fn shmobile_init_delay();
    fn shmobile_init_late();
}

#[used]
#[link_section = ".init.rodata"]
static R7S72100_BOARDS_COMPAT_DT: [*const core::ffi::c_char; 2] = [
    c"renesas,r7s72100".as_ptr(),
    core::ptr::null(),
];

// Translation of DT_MACHINE_START(R7S72100_DT,
// "Generic R7S72100 (Flattened Device Tree)") ... MACHINE_END.
#[repr(C)]
pub struct R7s72100DtMachine {
    pub name: &'static str,
    pub l2c_aux_val: u32,
    pub l2c_aux_mask: u32,
    pub init_early: Option<unsafe extern "C" fn()>,
    pub init_late: Option<unsafe extern "C" fn()>,
    pub dt_compat: *const *const core::ffi::c_char,
}

#[used]
#[link_section = ".init.data"]
pub static R7S72100_DT: R7s72100DtMachine = R7s72100DtMachine {
    name: "Generic R7S72100 (Flattened Device Tree)",
    l2c_aux_val: 0,
    l2c_aux_mask: !0u32,
    init_early: Some(shmobile_init_delay),
    init_late: Some(shmobile_init_late),
    dt_compat: R7S72100_BOARDS_COMPAT_DT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
