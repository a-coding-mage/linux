// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Sascha Hauer, Pengutronix
 */

// C dependencies: <asm/mach/arch.h> and "common.h".

use core::ffi::c_char;

extern "C" {
    fn mx31_map_io();
    fn imx31_init_early();
}

static IMX31_DT_BOARD_COMPAT: [*const c_char; 2] = [
    b"fsl,imx31\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// Corresponds to DT_MACHINE_START(IMX31_DT, "Freescale i.MX31 (Device Tree Support)")
// and MACHINE_END. The machine descriptor type and field layout are supplied by
// the architecture dependencies.
extern "C" {
    static mut IMX31_DT: MachineDesc;
}

#[repr(C)]
pub struct MachineDesc {
    pub name: *const c_char,
    pub map_io: Option<unsafe extern "C" fn()>,
    pub init_early: Option<unsafe extern "C" fn()>,
    pub dt_compat: *const *const c_char,
}

#[used]
#[no_mangle]
pub static mut IMX31_DT_MACHINE: MachineDesc = MachineDesc {
    name: b"Freescale i.MX31 (Device Tree Support)\0".as_ptr() as *const c_char,
    map_io: Some(mx31_map_io),
    init_early: Some(imx31_init_early),
    dt_compat: IMX31_DT_BOARD_COMPAT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
