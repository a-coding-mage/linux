// SPDX-License-Identifier: GPL-2.0
/*
 * SH7723 Pinmux
 *
 *  Copyright (C) 2008  Magnus Damm
 */

use core::ffi::{c_char, c_int, c_ulong};

// Supplied by the Linux kernel headers: <linux/ioport.h> and <cpu/pfc.h>.
#[repr(C)]
pub struct resource {
    pub start: c_ulong,
    pub end: c_ulong,
    pub flags: c_ulong,
}

unsafe extern "C" {
    fn sh_pfc_register(
        name: *const c_char,
        resources: *mut resource,
        num_resources: usize,
    ) -> c_int;
}

// IORESOURCE_MEM from <linux/ioport.h>.
const IORESOURCE_MEM: c_ulong = 0x0000_0200;

static mut sh7723_pfc_resources: [resource; 1] = [resource {
    start: 0xa4050100,
    end: 0xa405016f,
    flags: IORESOURCE_MEM,
}];

// __init
unsafe fn plat_pinmux_setup() -> c_int {
    static NAME: &[u8] = b"pfc-sh7723\0";

    sh_pfc_register(
        NAME.as_ptr() as *const c_char,
        sh7723_pfc_resources.as_mut_ptr(),
        sh7723_pfc_resources.len(),
    )
}

// arch_initcall(plat_pinmux_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
