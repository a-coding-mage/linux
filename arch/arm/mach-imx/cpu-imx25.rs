// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MX25 CPU type detection
 *
 * Copyright (c) 2009 Daniel Mack <daniel@caiaq.de>
 * Copyright (C) 2011 Freescale Semiconductor, Inc. All Rights Reserved
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn of_find_compatible_node(
        from: *mut device_node,
        typ: *const u8,
        compatible: *const u8,
    ) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut c_void;
    fn of_node_put(node: *mut device_node);
    fn readl(addr: *const c_void) -> u32;
    fn iounmap(addr: *mut c_void);
    fn EXPORT_SYMBOL(symbol: *const c_void);
}

// External constants supplied by iim.h and hardware.h.
extern "C" {
    static MXC_IIMSREV: usize;
    static IMX_CHIP_REVISION_1_0: i32;
    static IMX_CHIP_REVISION_1_1: i32;
    static IMX_CHIP_REVISION_1_2: i32;
    static IMX_CHIP_REVISION_UNKNOWN: i32;
}

static mut mx25_cpu_rev: i32 = -1;

unsafe fn mx25_read_cpu_rev() -> i32 {
    let rev: u32;
    let iim_base: *mut c_void;
    let np: *mut device_node;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"fsl,imx25-iim\0".as_ptr(),
    );
    iim_base = of_iomap(np, 0);
    of_node_put(np);
    if iim_base.is_null() {
        panic!("BUG_ON(!iim_base)");
    }
    rev = readl((iim_base as *mut u8).add(MXC_IIMSREV) as *const c_void);
    iounmap(iim_base);

    match rev {
        0x00 => IMX_CHIP_REVISION_1_0,
        0x01 => IMX_CHIP_REVISION_1_1,
        0x02 => IMX_CHIP_REVISION_1_2,
        _ => IMX_CHIP_REVISION_UNKNOWN,
    }
}

pub unsafe fn mx25_revision() -> i32 {
    if mx25_cpu_rev == -1 {
        mx25_cpu_rev = mx25_read_cpu_rev();
    }

    mx25_cpu_rev
}

// EXPORT_SYMBOL(mx25_revision);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
