// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MX35 CPU type detection
 *
 * Copyright (c) 2009 Daniel Mack <daniel@caiaq.de>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/module.h, linux/of_address.h, linux/io.h, hardware.h, and iim.h.

use core::ffi::c_void;

extern "C" {
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const i8,
        compatible: *const i8,
    ) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut c_void;
    fn of_node_put(np: *mut device_node);
    fn imx_readl(addr: *mut c_void) -> u32;
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    static MXC_IIMSREV: usize;
    static IMX_CHIP_REVISION_1_0: i32;
    static IMX_CHIP_REVISION_2_0: i32;
    static IMX_CHIP_REVISION_2_1: i32;
    static IMX_CHIP_REVISION_UNKNOWN: i32;
}

static mut mx35_cpu_rev: i32 = -1;

unsafe fn mx35_read_cpu_rev() -> i32 {
    let iim_base: *mut c_void;
    let np: *mut device_node;
    let rev: u32;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"fsl,imx35-iim\0".as_ptr() as *const i8,
    );
    iim_base = of_iomap(np, 0);
    of_node_put(np);
    if iim_base.is_null() {
        panic!("BUG_ON(!iim_base)");
    }

    rev = imx_readl((iim_base as *mut u8).add(MXC_IIMSREV) as *mut c_void);
    match rev {
        0x00 => IMX_CHIP_REVISION_1_0,
        0x10 => IMX_CHIP_REVISION_2_0,
        0x11 => IMX_CHIP_REVISION_2_1,
        _ => IMX_CHIP_REVISION_UNKNOWN,
    }
}

pub unsafe fn mx35_revision() -> i32 {
    if mx35_cpu_rev == -1 {
        mx35_cpu_rev = mx35_read_cpu_rev();
    }

    mx35_cpu_rev
}

// EXPORT_SYMBOL(mx35_revision);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
