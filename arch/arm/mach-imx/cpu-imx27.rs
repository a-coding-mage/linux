// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2007 Freescale Semiconductor, Inc. All Rights Reserved.
 * Copyright 2008 Juergen Beisert, kernel@pengutronix.de
 */

/*
 * i.MX27 specific CPU detection code
 */

// Dependencies supplied by the Linux I/O, device-tree, module, and hardware
// interfaces are intentionally left external to this translation unit.

use core::ffi::c_void;

extern "C" {
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const u8,
        compatible: *const u8,
    ) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut c_void;
    fn of_node_put(np: *mut device_node);
    fn imx_readl(addr: *const c_void) -> u32;
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

static mut mx27_cpu_rev: i32 = -1;
static mut mx27_cpu_partnumber: i32 = 0;

const SYS_CHIP_ID: usize = 0x00; // The offset of CHIP ID register
const SYSCTRL_OFFSET: usize = 0x800; // Offset from CCM base address

unsafe fn mx27_read_cpu_rev() -> i32 {
    let ccm_base: *mut c_void;
    let np: *mut device_node;
    let val: u32;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,imx27-ccm\0".as_ptr());
    ccm_base = of_iomap(np, 0);
    of_node_put(np);
    if ccm_base.is_null() {
        panic!("BUG_ON(!ccm_base)");
    }
    /*
     * now we have access to the IO registers. As we need
     * the silicon revision very early we read it here to
     * avoid any further hooks
     */
    val = imx_readl(
        (ccm_base as *mut u8)
            .add(SYSCTRL_OFFSET + SYS_CHIP_ID)
            .cast::<c_void>(),
    );

    mx27_cpu_partnumber = ((val >> 12) & 0xFFFF) as i32;

    match val >> 28 {
        0 => IMX_CHIP_REVISION_1_0,
        1 => IMX_CHIP_REVISION_2_0,
        2 => IMX_CHIP_REVISION_2_1,
        _ => IMX_CHIP_REVISION_UNKNOWN,
    }
}

/*
 * Returns:
 *\tthe silicon revision of the cpu
 *\t-EINVAL - not a mx27
 */
pub unsafe fn mx27_revision() -> i32 {
    if mx27_cpu_rev == -1 {
        mx27_cpu_rev = mx27_read_cpu_rev();
    }

    if mx27_cpu_partnumber != 0x8821 {
        return -EINVAL;
    }

    mx27_cpu_rev
}

// EXPORT_SYMBOL(mx27_revision);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
