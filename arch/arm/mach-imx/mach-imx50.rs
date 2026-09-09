// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013 Greg Ungerer <gerg@uclinux.org>
 * Copyright 2011 Freescale Semiconductor, Inc. All Rights Reserved.
 * Copyright 2011 Linaro Ltd.
 */

// C dependencies: <asm/mach/arch.h>, "common.h", and "hardware.h".

extern "C" {
    fn mxc_set_cpu_type(cpu_type: ::core::ffi::c_int);
    static MXC_CPU_MX50: ::core::ffi::c_int;
}

unsafe extern "C" fn imx50_init_early() {
    mxc_set_cpu_type(MXC_CPU_MX50);
}

static IMX50_DT_BOARD_COMPAT: [*const ::core::ffi::c_char; 2] = [
    b"fsl,imx50\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null(),
];

// Translation of DT_MACHINE_START(IMX50_DT, "Freescale i.MX50 (Device Tree Support)").
#[repr(C)]
pub struct Imx50DtMachine {
    pub init_early: unsafe extern "C" fn(),
    pub dt_compat: *const *const ::core::ffi::c_char,
    pub name: *const ::core::ffi::c_char,
}

#[no_mangle]
pub static IMX50_DT: Imx50DtMachine = Imx50DtMachine {
    init_early: imx50_init_early,
    dt_compat: IMX50_DT_BOARD_COMPAT.as_ptr(),
    name: b"Freescale i.MX50 (Device Tree Support)\0".as_ptr()
        as *const ::core::ffi::c_char,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
