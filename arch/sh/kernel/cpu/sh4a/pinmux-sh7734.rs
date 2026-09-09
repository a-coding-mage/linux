// SPDX-License-Identifier: GPL-2.0
/*
 * SH7734 processor support - PFC hardware block
 *
 * Copyright (C) 2012  Renesas Solutions Corp.
 * Copyright (C) 2012  Nobuhiro Iwamatsu <nobuhiro.iwamatsu.yj@renesas.com>
 */

// Supplied by the kernel's resource and PFC dependencies.
use crate::resource;

const IORESOURCE_MEM: u64 = 0x0000_0200;

extern "C" {
    fn sh_pfc_register(
        name: *const core::ffi::c_char,
        resources: *mut resource,
        count: usize,
    ) -> i32;
}

static mut SH7734_PFC_RESOURCES: [resource; 2] = [
    resource {
        start: 0xFFFC0000,
        end: 0xFFFC011C,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: 0xFFC40000,
        end: 0xFFC4502B,
        flags: IORESOURCE_MEM,
    },
];

unsafe fn plat_pinmux_setup() -> i32 {
    let name = b"pfc-sh7734\0";
    sh_pfc_register(
        name.as_ptr() as *const core::ffi::c_char,
        SH7734_PFC_RESOURCES.as_mut_ptr(),
        SH7734_PFC_RESOURCES.len(),
    )
}

// arch_initcall(plat_pinmux_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
