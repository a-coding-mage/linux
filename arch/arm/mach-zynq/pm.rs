// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Zynq power management
 *
 *  Copyright (C) 2012 - 2014 Xilinx
 *
 *  Sören Brinkmann <soren.brinkmann@xilinx.com>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

/* register offsets */
const DDRC_CTRL_REG1_OFFS: usize = 0x60;
const DDRC_DRAM_PARAM_REG3_OFFS: usize = 0x20;

/* bitfields */
const DDRC_CLOCKSTOP_MASK: u32 = 1u32 << 23;
const DDRC_SELFREFRESH_MASK: u32 = 1u32 << 12;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: c_int) -> *mut c_void;
    fn of_node_put(node: *mut device_node);
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn pr_warn(format: *const c_char, ...);
}

static mut ddrc_base: *mut c_void = core::ptr::null_mut();

/**
 * zynq_pm_ioremap() - Create IO mappings
 * @comp: DT compatible string
 * Return: Pointer to the mapped memory or NULL.
 *
 * Remap the memory region for a compatible DT node.
 */
unsafe fn zynq_pm_ioremap(comp: *const c_char) -> *mut c_void {
    let np: *mut device_node;
    let mut base: *mut c_void = core::ptr::null_mut();

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), comp);
    if !np.is_null() {
        base = of_iomap(np, 0);
        of_node_put(np);
    } else {
        // __func__ is represented by the function name in the format argument.
        pr_warn(b"zynq_pm_ioremap: no compatible node found for '%s'\n\0".as_ptr() as *const c_char, comp);
    }

    base
}

/**
 * zynq_pm_late_init() - Power management init
 *
 * Initialization of power management related features and infrastructure.
 */
pub unsafe fn zynq_pm_late_init() {
    let mut reg: u32;

    ddrc_base = zynq_pm_ioremap(b"xlnx,zynq-ddrc-a05\0".as_ptr() as *const c_char);
    if ddrc_base.is_null() {
        pr_warn(b"zynq_pm_late_init: Unable to map DDRC IO memory.\n\0".as_ptr() as *const c_char);
    } else {
        /*
         * Enable DDRC clock stop feature. The HW takes care of
         * entering/exiting the correct mode depending
         * on activity state.
         */
        reg = readl(ddrc_base.add(DDRC_DRAM_PARAM_REG3_OFFS));
        reg |= DDRC_CLOCKSTOP_MASK;
        writel(reg, ddrc_base.add(DDRC_DRAM_PARAM_REG3_OFFS));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
