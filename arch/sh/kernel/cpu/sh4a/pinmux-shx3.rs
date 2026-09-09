// SPDX-License-Identifier: GPL-2.0
/*
 * SH-X3 prototype CPU pinmux
 *
 * Copyright (C) 2010  Paul Mundt
 */

// Translated from the Linux kernel headers:
// <linux/bug.h>, <linux/init.h>, <linux/kernel.h>, <linux/ioport.h>, <cpu/pfc.h>

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
    pub end: c_ulong,
    pub flags: c_ulong,
}

// IORESOURCE_MEM
pub const IORESOURCE_MEM: c_ulong = 0x0000_0200;

unsafe extern "C" {
    fn sh_pfc_register(
        name: *const c_char,
        resources: *mut resource,
        num_resources: usize,
    ) -> c_int;
}

static mut shx3_pfc_resources: [resource; 1] = [resource {
    start: 0xffc7_0000,
    end: 0xffc7_001f,
    flags: IORESOURCE_MEM,
}];

#[inline]
unsafe fn plat_pinmux_setup() -> c_int {
    // ARRAY_SIZE(shx3_pfc_resources)
    sh_pfc_register(
        b"pfc-shx3\0".as_ptr() as *const c_char,
        shx3_pfc_resources.as_mut_ptr(),
        shx3_pfc_resources.len(),
    )
}

// arch_initcall(plat_pinmux_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
