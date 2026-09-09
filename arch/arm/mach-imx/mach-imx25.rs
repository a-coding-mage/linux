// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Sascha Hauer, Pengutronix
 */

use core::ffi::c_char;

extern "C" {
    fn mxc_set_cpu_type(cpu_type: i32);
    fn imx_aips_allow_unprivileged_access(compat: *const c_char);
    fn imx25_pm_init();
}

const MXC_CPU_MX25: i32 = 25;

unsafe fn imx25_init_early() {
    mxc_set_cpu_type(MXC_CPU_MX25);
}

unsafe fn imx25_dt_init() {
    imx_aips_allow_unprivileged_access(b"fsl,imx25-aips\0".as_ptr() as *const c_char);
}

static IMX25_DT_BOARD_COMPAT: [*const c_char; 2] = [
    b"fsl,imx25\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

#[repr(C)]
struct Imx25DtMachine {
    name: *const c_char,
    init_early: unsafe fn(),
    init_machine: unsafe fn(),
    init_late: unsafe extern "C" fn(),
    dt_compat: *const *const c_char,
}

#[used]
static IMX25_DT: Imx25DtMachine = Imx25DtMachine {
    name: b"Freescale i.MX25 (Device Tree Support)\0".as_ptr() as *const c_char,
    init_early: imx25_init_early,
    init_machine: imx25_dt_init,
    init_late: imx25_pm_init,
    dt_compat: IMX25_DT_BOARD_COMPAT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
