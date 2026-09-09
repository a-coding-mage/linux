// SPDX-License-Identifier: GPL-2.0-only
/*
 *  arch/arm/mach-socfpga/pm.c
 *
 * Copyright (C) 2014-2015 Altera Corporation. All rights reserved.
 *
 * with code from pm-imx6.c
 * Copyright 2011-2014 Freescale Semiconductor, Inc.
 * Copyright 2011 Linaro Ltd.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gen_pool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type phys_addr_t = usize;
pub type suspend_state_t = c_int;

extern "C" {
    static mut socfpga_sdram_self_refresh_sz: usize;
    static mut sdr_ctl_base_addr: *mut c_void;
    static mut socfpga_sdram_self_refresh: unsafe extern "C" fn(u32) -> u32;

    fn of_find_compatible_node(
        from: *mut device_node,
        ty: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;
    fn gen_pool_get(dev: *mut device, name: *const c_char) -> *mut gen_pool;
    fn gen_pool_alloc(pool: *mut gen_pool, size: usize) -> c_ulong;
    fn gen_pool_virt_to_phys(pool: *mut gen_pool, addr: c_ulong) -> phys_addr_t;
    fn __arm_ioremap_exec(addr: phys_addr_t, size: usize, cached: bool) -> *mut c_void;
    fn fncpy(dest: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
    fn put_device(dev: *mut device);
    fn of_node_put(np: *mut device_node);
    fn outer_disable();
    fn outer_resume();
    fn cpu_suspend(arg: c_ulong, fn_: unsafe extern "C" fn(c_ulong) -> c_int) -> c_int;
    fn suspend_valid_only_mem(state: suspend_state_t) -> bool;
    fn suspend_set_ops(ops: *const platform_suspend_ops);
}

#[repr(C)]
pub struct platform_suspend_ops {
    pub valid: Option<unsafe extern "C" fn(suspend_state_t) -> bool>,
    pub begin: Option<unsafe extern "C" fn(suspend_state_t) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn() -> c_int>,
    pub prepare_late: Option<unsafe extern "C" fn() -> c_int>,
    pub enter: Option<unsafe extern "C" fn(suspend_state_t) -> c_int>,
    pub wake: Option<unsafe extern "C" fn()>,
    pub finish: Option<unsafe extern "C" fn()>,
    pub end: Option<unsafe extern "C" fn()>,
    pub recover: Option<unsafe extern "C" fn()>,
};

const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const PM_SUSPEND_MEM: suspend_state_t = 3;

/* Pointer to function copied to ocram */
static mut socfpga_sdram_self_refresh_in_ocram: Option<unsafe extern "C" fn(u32) -> u32> = None;

unsafe fn socfpga_setup_ocram_self_refresh() -> c_int {
    let mut pdev: *mut platform_device;
    let ocram_pbase: phys_addr_t;
    let np: *mut device_node;
    let ocram_pool: *mut gen_pool;
    let ocram_base: c_ulong;
    let suspend_ocram_base: *mut c_void;
    let mut ret: c_int = 0;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"mmio-sram\0".as_ptr() as *const c_char);
    if np.is_null() {
        return -ENODEV;
    }

    pdev = of_find_device_by_node(np);
    if pdev.is_null() {
        ret = -ENODEV;
        of_node_put(np);
        return ret;
    }

    ocram_pool = gen_pool_get(pdev as *mut device, core::ptr::null());
    if ocram_pool.is_null() {
        ret = -ENODEV;
        put_device(pdev as *mut device);
        of_node_put(np);
        return ret;
    }

    ocram_base = gen_pool_alloc(ocram_pool, socfpga_sdram_self_refresh_sz);
    if ocram_base == 0 {
        ret = -ENOMEM;
        put_device(pdev as *mut device);
        of_node_put(np);
        return ret;
    }

    ocram_pbase = gen_pool_virt_to_phys(ocram_pool, ocram_base);
    suspend_ocram_base = __arm_ioremap_exec(ocram_pbase, socfpga_sdram_self_refresh_sz, false);
    if suspend_ocram_base.is_null() {
        ret = -ENOMEM;
        put_device(pdev as *mut device);
        of_node_put(np);
        return ret;
    }

    /* Copy the code that puts DDR in self refresh to ocram */
    socfpga_sdram_self_refresh_in_ocram = Some(core::mem::transmute(fncpy(
        suspend_ocram_base,
        socfpga_sdram_self_refresh as *const c_void,
        socfpga_sdram_self_refresh_sz,
    )));

    if socfpga_sdram_self_refresh_in_ocram.is_none() {
        ret = -EFAULT;
    }

    put_device(pdev as *mut device);
    of_node_put(np);
    ret
}

unsafe extern "C" fn socfpga_pm_suspend(arg: c_ulong) -> c_int {
    let ret: u32;

    if sdr_ctl_base_addr.is_null() {
        return -EFAULT;
    }

    ret = socfpga_sdram_self_refresh_in_ocram.unwrap()(sdr_ctl_base_addr as u32);
    let _ = (ret & 0xffff, (ret >> 16) & 0xffff);
    let _ = arg;
    0
}

unsafe extern "C" fn socfpga_pm_enter(state: suspend_state_t) -> c_int {
    match state {
        PM_SUSPEND_MEM => {
            outer_disable();
            cpu_suspend(0, socfpga_pm_suspend);
            outer_resume();
        }
        _ => return -EINVAL,
    }
    0
}

static socfpga_pm_ops: platform_suspend_ops = platform_suspend_ops {
    valid: Some(suspend_valid_only_mem),
    begin: None,
    prepare: None,
    prepare_late: None,
    enter: Some(socfpga_pm_enter),
    wake: None,
    finish: None,
    end: None,
    recover: None,
};

unsafe fn socfpga_pm_init() -> c_int {
    let ret = socfpga_setup_ocram_self_refresh();
    if ret != 0 {
        return ret;
    }

    suspend_set_ops(&socfpga_pm_ops);
    0
}

// arch_initcall(socfpga_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
