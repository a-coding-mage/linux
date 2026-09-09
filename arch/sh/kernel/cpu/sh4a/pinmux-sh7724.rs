// SPDX-License-Identifier: GPL-2.0
/*
 * SH7724 Pinmux
 *
 * Copyright (C) 2009 Renesas Solutions Corp.
 *
 * Kuninori Morimoto <morimoto.kuninori@renesas.com>
 *
 * Based on SH7723 Pinmux
 *  Copyright (C) 2008  Magnus Damm
 */

// Dependencies supplied by the surrounding kernel environment:
// linux/bug.h, linux/init.h, linux/kernel.h, linux/ioport.h, cpu/pfc.h

static mut sh7724_pfc_resources: [resource; 1] = [resource {
    start: 0xa4050100,
    end: 0xa405016f,
    flags: IORESOURCE_MEM,
}];

unsafe extern "C" {
    fn sh_pfc_register(
        name: *const core::ffi::c_char,
        resources: *mut resource,
        num_resources: usize,
    ) -> core::ffi::c_int;
}

#[allow(non_snake_case)]
unsafe fn plat_pinmux_setup() -> core::ffi::c_int {
    sh_pfc_register(
        b"pfc-sh7724\0".as_ptr() as *const core::ffi::c_char,
        sh7724_pfc_resources.as_mut_ptr(),
        sh7724_pfc_resources.len(),
    )
}

// Equivalent to: arch_initcall(plat_pinmux_setup)
arch_initcall!(plat_pinmux_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
