// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 Pengutronix, Oleksij Rempel <o.rempel@pengutronix.de>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, asm/v7m.h, and asm/mach/arch.h.

use core::ffi::c_char;

// Corresponds to the external ARMv7-M restart routine.
extern "C" {
    fn armv7m_restart(mode: u32, cmd: *const c_char);
}

// __initconst: initialized data intended for the kernel init section.
static IMX7D_CM4_DT_COMPAT: [*const c_char; 2] = [
    b"fsl,imx7d-cm4\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(IMX7D, "Freescale i.MX7 Dual Cortex-M4 (Device Tree)")
// MACHINE_END
// The macro-generated machine descriptor is represented directly here.
#[repr(C)]
struct MachineDesc {
    name: *const c_char,
    dt_compat: *const *const c_char,
    restart: unsafe extern "C" fn(mode: u32, cmd: *const c_char),
}

#[no_mangle]
static IMX7D: MachineDesc = MachineDesc {
    name: b"Freescale i.MX7 Dual Cortex-M4 (Device Tree)\0".as_ptr() as *const c_char,
    dt_compat: IMX7D_CM4_DT_COMPAT.as_ptr(),
    restart: armv7m_restart,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
