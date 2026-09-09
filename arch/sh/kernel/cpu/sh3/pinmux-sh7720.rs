// SPDX-License-Identifier: GPL-2.0
/*
 * SH7720 Pinmux
 *
 *  Copyright (C) 2008  Magnus Damm
 */

// C dependencies: <linux/bug.h>, <linux/init.h>, <linux/kernel.h>,
// <linux/ioport.h>, and <cpu/pfc.h>.

extern "C" {
    fn sh_pfc_register(
        name: *const core::ffi::c_char,
        resources: *mut crate::resource,
        resource_count: usize,
    ) -> core::ffi::c_int;
}

static mut sh7720_pfc_resources: [crate::resource; 1] = [crate::resource {
    start: 0xa4050100,
    end: 0xa405016f,
    flags: crate::IORESOURCE_MEM,
}];

#[no_mangle]
pub unsafe extern "C" fn plat_pinmux_setup() -> core::ffi::c_int {
    sh_pfc_register(
        b"pfc-sh7720\0".as_ptr() as *const core::ffi::c_char,
        sh7720_pfc_resources.as_mut_ptr(),
        sh7720_pfc_resources.len(),
    )
}

// C registration: arch_initcall(plat_pinmux_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
