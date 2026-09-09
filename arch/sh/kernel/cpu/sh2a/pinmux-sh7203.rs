// SPDX-License-Identifier: GPL-2.0
/*
 * SH7203 Pinmux
 *
 *  Copyright (C) 2008  Magnus Damm
 */

// C dependencies:
// #include <linux/bug.h>
// #include <linux/init.h>
// #include <linux/kernel.h>
// #include <linux/ioport.h>
// #include <cpu/pfc.h>

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

extern "C" {
    pub fn sh_pfc_register(
        name: *const core::ffi::c_char,
        resources: *mut resource,
        num_resources: usize,
    ) -> core::ffi::c_int;
}

// Supplied by the Linux I/O resource definitions.
extern "C" {
    pub static IORESOURCE_MEM: usize;
}

static mut sh7203_pfc_resources: [resource; 1] = [resource {
    start: 0xfffe3800,
    end: 0xfffe3a9f,
    flags: unsafe { IORESOURCE_MEM },
}];

// __init
unsafe fn plat_pinmux_setup() -> core::ffi::c_int {
    sh_pfc_register(
        b"pfc-sh7203\0".as_ptr() as *const core::ffi::c_char,
        sh7203_pfc_resources.as_mut_ptr(),
        sh7203_pfc_resources.len(),
    )
}

// arch_initcall(plat_pinmux_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
