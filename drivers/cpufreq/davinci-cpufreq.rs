// SPDX-License-Identifier: GPL-2.0-only
/*
 * CPU frequency scaling for DaVinci
 *
 * Copyright (C) 2009 Texas Instruments Incorporated - https://www.ti.com/
 *
 * Based on linux/arch/arm/plat-omap/cpu-omap.c. Original Copyright follows:
 *
 *  Copyright (C) 2005 Nokia Corporation
 *  Written by Tony Lindgren <tony@atomide.com>
 *
 *  Based on cpu-sa1110.c, Copyright (C) 2001 Russell King
 *
 * Copyright (C) 2007-2008 Texas Instruments, Inc.
 * Updated to support OMAP3
 * Rajendra Nayak <rnayak@ti.com>
 */

// External kernel types and functions supplied by other translated units.
#[repr(C)]
pub struct device {
    pub platform_data: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct clk;
#[repr(C)]
pub struct platform_device { pub dev: device }
#[repr(C)]
pub struct cpufreq_policy {
    pub cpu: u32,
    pub cur: u32,
    pub clk: *mut clk,
}
#[repr(C)]
pub struct cpufreq_frequency_table { pub frequency: u32 }
#[repr(C)]
pub struct davinci_cpufreq_config {
    pub freq_table: *mut cpufreq_frequency_table,
    pub set_voltage: Option<unsafe extern "C" fn(u32) -> i32>,
    pub init: Option<unsafe extern "C" fn() -> i32>,
}
#[repr(C)]
pub struct cpufreq_driver {
    pub flags: u32,
    pub verify: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    pub target_index: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> u32>,
    pub init: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    pub name: *const core::ffi::c_char,
}

extern "C" {
    fn clk_set_rate(clk: *mut clk, rate: u32) -> i32;
    fn clk_get(dev: *mut device, id: *const core::ffi::c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn clk_put(clk: *mut clk);
    fn cpufreq_generic_init(policy: *mut cpufreq_policy, table: *mut cpufreq_frequency_table, latency: u32);
    fn cpufreq_generic_frequency_table_verify(policy: *mut cpufreq_policy) -> i32;
    fn cpufreq_generic_get(policy: *mut cpufreq_policy) -> u32;
    fn cpufreq_register_driver(driver: *mut cpufreq_driver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut cpufreq_driver);
    fn platform_driver_probe(driver: *mut platform_driver, probe: unsafe extern "C" fn(*mut platform_device) -> i32) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char);
    fn is_err(ptr: *mut clk) -> bool;
    fn ptr_err(ptr: *mut clk) -> i32;
}

pub const CPUFREQ_NEED_INITIAL_FREQ_CHECK: u32 = 1;

#[repr(C)]
pub struct platform_driver_driver { pub name: *const core::ffi::c_char }
#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
struct davinci_cpufreq {
    dev: *mut device,
    armclk: *mut clk,
    asyncclk: *mut clk,
    asyncrate: u32,
}
static mut CPUFREQ: davinci_cpufreq = davinci_cpufreq {
    dev: core::ptr::null_mut(), armclk: core::ptr::null_mut(),
    asyncclk: core::ptr::null_mut(), asyncrate: 0,
};

unsafe extern "C" fn davinci_target(policy: *mut cpufreq_policy, idx: u32) -> i32 {
    let pdata = &mut *((*CPUFREQ.dev).platform_data as *mut davinci_cpufreq_config);
    let armclk = CPUFREQ.armclk;
    let old_freq = (*policy).cur;
    let new_freq = (*pdata.freq_table.add(idx as usize)).frequency;
    if let Some(set_voltage) = pdata.set_voltage {
        if new_freq > old_freq {
            let ret = set_voltage(idx);
            if ret != 0 { return ret; }
        }
    }
    let ret = clk_set_rate(armclk, new_freq.wrapping_mul(1000));
    if ret != 0 { return ret; }
    if !CPUFREQ.asyncclk.is_null() {
        let ret = clk_set_rate(CPUFREQ.asyncclk, CPUFREQ.asyncrate);
        if ret != 0 { return ret; }
    }
    if let Some(set_voltage) = pdata.set_voltage {
        if new_freq < old_freq { set_voltage(idx); }
    }
    0
}

unsafe extern "C" fn davinci_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    let pdata = &mut *((*CPUFREQ.dev).platform_data as *mut davinci_cpufreq_config);
    if (*policy).cpu != 0 { return -22; }
    if let Some(init) = pdata.init {
        let result = init();
        if result != 0 { return result; }
    }
    (*policy).clk = CPUFREQ.armclk;
    cpufreq_generic_init(policy, pdata.freq_table, 2000u32.wrapping_mul(1000));
    0
}

static mut DAVINCI_DRIVER: cpufreq_driver = cpufreq_driver {
    flags: CPUFREQ_NEED_INITIAL_FREQ_CHECK,
    verify: Some(cpufreq_generic_frequency_table_verify), target_index: Some(davinci_target),
    get: Some(cpufreq_generic_get), init: Some(davinci_cpu_init), name: b"davinci\0".as_ptr() as *const _
};

unsafe extern "C" fn davinci_cpufreq_probe(pdev: *mut platform_device) -> i32 {
    let pdata = (*pdev).dev.platform_data as *mut davinci_cpufreq_config;
    if pdata.is_null() || (*pdata).freq_table.is_null() { return -22; }
    CPUFREQ.dev = &mut (*pdev).dev;
    CPUFREQ.armclk = clk_get(core::ptr::null_mut(), b"arm\0".as_ptr() as *const _);
    if is_err(CPUFREQ.armclk) {
        dev_err(CPUFREQ.dev, b"Unable to get ARM clock\n\0".as_ptr() as *const _);
        return ptr_err(CPUFREQ.armclk);
    }
    let asyncclk = clk_get(CPUFREQ.dev, b"async\0".as_ptr() as *const _);
    if !is_err(asyncclk) { CPUFREQ.asyncclk = asyncclk; CPUFREQ.asyncrate = clk_get_rate(asyncclk); }
    cpufreq_register_driver(&mut DAVINCI_DRIVER)
}

unsafe extern "C" fn davinci_cpufreq_remove(_pdev: *mut platform_device) {
    cpufreq_unregister_driver(&mut DAVINCI_DRIVER);
    clk_put(CPUFREQ.armclk);
    if !CPUFREQ.asyncclk.is_null() { clk_put(CPUFREQ.asyncclk); }
}

static mut DAVINCI_CPUFREQ_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_driver { name: b"cpufreq-davinci\0".as_ptr() as *const _ },
    remove: Some(davinci_cpufreq_remove),
};

pub unsafe extern "C" fn davinci_cpufreq_init() -> i32 {
    platform_driver_probe(&mut DAVINCI_CPUFREQ_DRIVER, davinci_cpufreq_probe)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
