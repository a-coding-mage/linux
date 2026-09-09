// SPDX-License-Identifier: GPL-2.0
/*
 * SH7269 Pinmux
 *
 * Copyright (C) 2012  Renesas Electronics Europe Ltd
 * Copyright (C) 2012  Phil Edworthy
 */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int};

extern "C" {
    fn sh_pfc_register(
        name: *const c_char,
        resources: *mut resource,
        count: usize,
    ) -> c_int;
}

static mut sh7269_pfc_resources: [resource; 1] = [resource {
    start: 0xfffe3800,
    end: 0xfffe391f,
    flags: IORESOURCE_MEM,
}];

unsafe fn plat_pinmux_setup() -> c_int {
    sh_pfc_register(
        b"pfc-sh7269\0".as_ptr() as *const c_char,
        sh7269_pfc_resources.as_mut_ptr(),
        sh7269_pfc_resources.len(),
    )
}

// C equivalent: arch_initcall(plat_pinmux_setup).
// The architecture initcall registration is supplied by the surrounding kernel.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
