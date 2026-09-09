/*
 * i.MX27 Power Management Routines
 *
 * Based on Freescale's BSP
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License.
 */

// C dependencies: <linux/of_address.h>, <linux/kernel.h>,
// <linux/suspend.h>, <linux/io.h>, "common.h", and "hardware.h".

use core::ffi::{c_char, c_int, c_void};

// These types and symbols are supplied by the surrounding kernel bindings.
type SuspendState = u32;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PlatformSuspendOps {
    pub enter: Option<unsafe extern "C" fn(state: SuspendState) -> c_int>,
    pub valid: Option<unsafe extern "C" fn(state: SuspendState) -> bool>,
}

extern "C" {
    fn of_find_compatible_node(
        from: *mut DeviceNode,
        ty: *const c_char,
        compatible: *const c_char,
    ) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: c_int) -> *mut c_void;
    fn imx_readl(addr: *mut c_void) -> u32;
    fn imx_writel(value: u32, addr: *mut c_void);
    fn cpu_do_idle();
    fn suspend_valid_only_mem(state: SuspendState) -> bool;
    fn suspend_set_ops(ops: *const PlatformSuspendOps);
}

const PM_SUSPEND_MEM: SuspendState = 3;
const EINVAL: c_int = 22;

unsafe fn mx27_suspend_enter(state: SuspendState) -> c_int {
    let ccm_base: *mut c_void;
    let np: *mut DeviceNode;
    let mut cscr: u32;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"fsl,imx27-ccm\0".as_ptr() as *const c_char,
    );
    ccm_base = of_iomap(np, 0);
    // BUG_ON(!ccm_base);
    if ccm_base.is_null() {
        panic!("BUG_ON(!ccm_base)");
    }

    match state {
        PM_SUSPEND_MEM => {
            /* Clear MPEN and SPEN to disable MPLL/SPLL */
            cscr = imx_readl(ccm_base);
            cscr &= 0xFFFFFFFC;
            imx_writel(cscr, ccm_base);
            /* Executes WFI */
            cpu_do_idle();
        }

        _ => {
            return -EINVAL;
        }
    }
    0
}

static MX27_SUSPEND_OPS: PlatformSuspendOps = PlatformSuspendOps {
    enter: Some(mx27_suspend_enter),
    valid: Some(suspend_valid_only_mem),
};

pub unsafe extern "C" fn imx27_pm_init() {
    suspend_set_ops(&MX27_SUSPEND_OPS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
