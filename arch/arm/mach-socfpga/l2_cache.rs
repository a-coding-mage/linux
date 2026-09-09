// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright Altera Corporation (C) 2016. All rights reserved.
 */

// Linux I/O, device-tree, and core declarations are supplied by the surrounding build.

use core::ffi::c_void;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const i8,
        compatible: *const i8,
    ) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut c_void;
    fn of_node_put(np: *mut device_node);
    fn iounmap(addr: *mut c_void);
    fn writel(value: u32, addr: *mut c_void);
    fn pr_err(format: *const i8);
    static mut sys_manager_base_addr: *mut c_void;
}

/* A10 System Manager L2 ECC Control register */
const A10_MPU_CTRL_L2_ECC_OFST: usize = 0x0;
const A10_MPU_CTRL_L2_ECC_EN: u32 = 1 << 0;

/* A10 System Manager Global IRQ Mask register */
const A10_SYSMGR_ECC_INTMASK_CLR_OFST: usize = 0x98;
const A10_SYSMGR_ECC_INTMASK_CLR_L2: u32 = 1 << 0;

/* A10 System Manager L2 ECC IRQ Clear register */
const A10_SYSMGR_MPU_CLEAR_L2_ECC_OFST: usize = 0xA8;
const A10_SYSMGR_MPU_CLEAR_L2_ECC: u32 = (1 << 31) | (1 << 15);

pub unsafe fn socfpga_init_l2_ecc() {
    let np: *mut device_node;
    let mapped_l2_edac_addr: *mut c_void;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"altr,socfpga-l2-ecc\0".as_ptr() as *const i8,
    );
    if np.is_null() {
        pr_err(b"Unable to find socfpga-l2-ecc in dtb\n\0".as_ptr() as *const i8);
        return;
    }

    mapped_l2_edac_addr = of_iomap(np, 0);
    of_node_put(np);
    if mapped_l2_edac_addr.is_null() {
        pr_err(b"Unable to find L2 ECC mapping in dtb\n\0".as_ptr() as *const i8);
        return;
    }

    /* Enable ECC */
    writel(0x01, mapped_l2_edac_addr);
    iounmap(mapped_l2_edac_addr);
}

pub unsafe fn socfpga_init_arria10_l2_ecc() {
    let np: *mut device_node;
    let mapped_l2_edac_addr: *mut c_void;

    /* Find the L2 EDAC device tree node */
    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"altr,socfpga-a10-l2-ecc\0".as_ptr() as *const i8,
    );
    if np.is_null() {
        pr_err(b"Unable to find socfpga-a10-l2-ecc in dtb\n\0".as_ptr() as *const i8);
        return;
    }

    mapped_l2_edac_addr = of_iomap(np, 0);
    of_node_put(np);
    if mapped_l2_edac_addr.is_null() {
        pr_err(b"Unable to find L2 ECC mapping in dtb\n\0".as_ptr() as *const i8);
        return;
    }

    if sys_manager_base_addr.is_null() {
        pr_err(b"System Manager not mapped for L2 ECC\n\0".as_ptr() as *const i8);
        iounmap(mapped_l2_edac_addr);
        return;
    }
    /* Clear any pending IRQs */
    writel(
        A10_SYSMGR_MPU_CLEAR_L2_ECC,
        (sys_manager_base_addr as *mut u8).add(A10_SYSMGR_MPU_CLEAR_L2_ECC_OFST)
            as *mut c_void,
    );
    /* Enable ECC */
    writel(
        A10_SYSMGR_ECC_INTMASK_CLR_L2,
        (sys_manager_base_addr as *mut u8).add(A10_SYSMGR_ECC_INTMASK_CLR_OFST) as *mut c_void,
    );
    writel(
        A10_MPU_CTRL_L2_ECC_EN,
        (mapped_l2_edac_addr as *mut u8).add(A10_MPU_CTRL_L2_ECC_OFST) as *mut c_void,
    );
    iounmap(mapped_l2_edac_addr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
