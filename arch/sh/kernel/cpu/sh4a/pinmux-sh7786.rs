// SPDX-License-Identifier: GPL-2.0
/*
 * SH7786 Pinmux
 *
 * Copyright (C) 2008, 2009  Renesas Solutions Corp.
 * Kuninori Morimoto <morimoto.kuninori@renesas.com>
 *
 *  Based on SH7785 pinmux
 *
 *  Copyright (C) 2008  Magnus Damm
 */

use core::ffi::{c_char, c_int};

// Declarations supplied by the Linux platform headers.
#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

unsafe extern "C" {
    fn sh_pfc_register(
        name: *const c_char,
        resources: *mut resource,
        count: usize,
    ) -> c_int;
}

static mut sh7786_pfc_resources: [resource; 1] = [resource {
    start: 0xffcc0000,
    end: 0xffcc008f,
    flags: 0x00000200, // IORESOURCE_MEM
}];

pub unsafe extern "C" fn plat_pinmux_setup() -> c_int {
    sh_pfc_register(
        b"pfc-sh7786\0".as_ptr() as *const c_char,
        sh7786_pfc_resources.as_mut_ptr(),
        sh7786_pfc_resources.len(),
    )
}

// arch_initcall(plat_pinmux_setup);
// The initcall registration is supplied by the target kernel build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
