// SPDX-License-Identifier: GPL-2.0
/*
 * SH7264 Pinmux
 *
 *  Copyright (C) 2012  Renesas Electronics Europe Ltd
 */

// C dependencies:
// linux/bug.h, linux/init.h, linux/kernel.h, linux/ioport.h, and cpu/pfc.h

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
    pub end: c_ulong,
    pub flags: c_ulong,
}

// Supplied by cpu/pfc.h.
unsafe extern "C" {
    fn sh_pfc_register(
        name: *const c_char,
        resources: *mut resource,
        count: usize,
    ) -> c_int;
}

// Supplied by linux/ioport.h.
unsafe extern "C" {
    static IORESOURCE_MEM: c_ulong;
}

static mut SH7264_PFC_RESOURCES: [resource; 1] = [resource {
    start: 0xfffe3800,
    end: 0xfffe393f,
    flags: 0,
}];

#[inline(never)]
pub unsafe extern "C" fn plat_pinmux_setup() -> c_int {
    SH7264_PFC_RESOURCES[0].flags = IORESOURCE_MEM;
    sh_pfc_register(
        b"pfc-sh7264\0".as_ptr() as *const c_char,
        SH7264_PFC_RESOURCES.as_mut_ptr(),
        SH7264_PFC_RESOURCES.len(),
    )
}

// arch_initcall(plat_pinmux_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
