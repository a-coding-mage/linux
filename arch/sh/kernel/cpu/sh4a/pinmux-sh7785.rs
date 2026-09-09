// SPDX-License-Identifier: GPL-2.0
/*
 * SH7785 Pinmux
 *
 *  Copyright (C) 2008  Magnus Damm
 */

use core::ffi::{c_char, c_int, c_ulong};

// Translated from the Linux resource declaration used by the source file.
#[repr(C)]
pub struct Resource {
    pub start: u64,
    pub end: u64,
    pub flags: c_ulong,
}

pub const IORESOURCE_MEM: c_ulong = 0x0000_0200;

unsafe extern "C" {
    fn sh_pfc_register(
        name: *const c_char,
        res: *const Resource,
        num_res: usize,
    ) -> c_int;
}

static SH7785_PFC_RESOURCES: [Resource; 1] = [Resource {
    start: 0xffe70000,
    end: 0xffe7008f,
    flags: IORESOURCE_MEM,
}];

// __init
pub unsafe extern "C" fn plat_pinmux_setup() -> c_int {
    static NAME: &[u8] = b"pfc-sh7785\0";

    unsafe {
        sh_pfc_register(
            NAME.as_ptr() as *const c_char,
            SH7785_PFC_RESOURCES.as_ptr(),
            SH7785_PFC_RESOURCES.len(),
        )
    }
}

// arch_initcall(plat_pinmux_setup)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
