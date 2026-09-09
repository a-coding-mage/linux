// SPDX-License-Identifier: GPL-2.0
/*
 * SH7757 (B0 step) Pinmux
 *
 *  Copyright (C) 2009-2010  Renesas Solutions Corp.
 *
 *  Author : Yoshihiro Shimoda <shimoda.yoshihiro@renesas.com>
 *
 * Based on SH7723 Pinmux
 *  Copyright (C) 2008  Magnus Damm
 */

// The following names are supplied by the platform headers and other
// translation units in the surrounding kernel.
use crate::{resource, sh_pfc_register, IORESOURCE_MEM};

static mut sh7757_pfc_resources: [resource; 1] = [resource {
    start: 0xffec0000,
    end: 0xffec008f,
    flags: IORESOURCE_MEM,
}];

unsafe extern "C" fn plat_pinmux_setup() -> i32 {
    sh_pfc_register(
        b"pfc-sh7757\0".as_ptr() as *const core::ffi::c_char,
        sh7757_pfc_resources.as_mut_ptr(),
        sh7757_pfc_resources.len(),
    )
}

// arch_initcall(plat_pinmux_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
