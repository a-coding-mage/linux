// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
 */

use core::ffi::c_char;

// Supplied by the architecture and i.MX support dependencies.
use crate::{mxc_restart, mxc_set_cpu_type, MachineDesc, MXC_CPU_MX1};

unsafe fn imx1_init_early() {
    mxc_set_cpu_type(MXC_CPU_MX1);
}

static IMX1_DT_BOARD_COMPAT: [*const c_char; 2] = [
    b"fsl,imx1\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// Equivalent of DT_MACHINE_START(IMX1_DT, "Freescale i.MX1 (Device Tree Support)").
#[no_mangle]
pub static IMX1_DT: MachineDesc = MachineDesc {
    name: b"Freescale i.MX1 (Device Tree Support)\0".as_ptr() as *const c_char,
    init_early: Some(imx1_init_early),
    dt_compat: IMX1_DT_BOARD_COMPAT.as_ptr(),
    restart: Some(mxc_restart),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
