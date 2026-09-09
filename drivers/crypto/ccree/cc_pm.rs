// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

// Dependencies supplied by the surrounding kernel driver and other translation units.

use core::ffi::c_void;

pub const POWER_DOWN_ENABLE: u32 = 0x01;
pub const POWER_DOWN_DISABLE: u32 = 0x00;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cc_drvdata {
    _private: [u8; 0],
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut cc_drvdata;
    fn dev_dbg(dev: *mut device, fmt: *const i8, ...);
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn fini_cc_regs(drvdata: *mut cc_drvdata);
    fn cc_iowrite(drvdata: *mut cc_drvdata, reg: u32, value: u32);
    fn cc_wait_for_reset_completion(drvdata: *mut cc_drvdata) -> bool;
    fn clk_disable_unprepare(clk: *mut c_void);
    fn clk_prepare_enable(clk: *mut c_void) -> i32;
    fn init_cc_regs(drvdata: *mut cc_drvdata) -> i32;
    fn cc_tee_handle_fips_error(drvdata: *mut cc_drvdata);
    fn cc_init_hash_sram(drvdata: *mut cc_drvdata);
    fn pm_runtime_get_sync(dev: *mut device) -> i32;
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn CC_REG(reg: u32) -> u32;
    fn cc_drvdata_clk(drvdata: *mut cc_drvdata) -> *mut c_void;
}

extern "C" {
    static HOST_POWER_DOWN_EN: u32;
}

const EBUSY: i32 = 16;

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> i32>,
}

unsafe extern "C" fn cc_pm_suspend(dev: *mut device) -> i32 {
    let drvdata = dev_get_drvdata(dev);

    dev_dbg(dev, b"set HOST_POWER_DOWN_EN\0".as_ptr() as *const i8);
    fini_cc_regs(drvdata);
    cc_iowrite(drvdata, CC_REG(HOST_POWER_DOWN_EN), POWER_DOWN_ENABLE);
    clk_disable_unprepare(cc_drvdata_clk(drvdata));
    0
}

unsafe extern "C" fn cc_pm_resume(dev: *mut device) -> i32 {
    let mut rc: i32;
    let drvdata = dev_get_drvdata(dev);

    dev_dbg(dev, b"unset HOST_POWER_DOWN_EN\0".as_ptr() as *const i8);
    /* Enables the device source clk */
    rc = clk_prepare_enable(cc_drvdata_clk(drvdata));
    if rc != 0 {
        dev_err(
            dev,
            b"failed getting clock back on. We're toast.\n\0".as_ptr() as *const i8,
        );
        return rc;
    }
    /* wait for Cryptocell reset completion */
    if !cc_wait_for_reset_completion(drvdata) {
        dev_err(dev, b"Cryptocell reset not completed\0".as_ptr() as *const i8);
        clk_disable_unprepare(cc_drvdata_clk(drvdata));
        return -EBUSY;
    }

    cc_iowrite(drvdata, CC_REG(HOST_POWER_DOWN_EN), POWER_DOWN_DISABLE);
    rc = init_cc_regs(drvdata);
    if rc != 0 {
        dev_err(dev, b"init_cc_regs (%x)\n\0".as_ptr() as *const i8, rc);
        clk_disable_unprepare(cc_drvdata_clk(drvdata));
        return rc;
    }
    /* check if tee fips error occurred during power down */
    cc_tee_handle_fips_error(drvdata);

    cc_init_hash_sram(drvdata);

    0
}

pub static ccree_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(cc_pm_suspend),
    resume: Some(cc_pm_resume),
    runtime_suspend: None,
};

pub unsafe extern "C" fn cc_pm_get(dev: *mut device) -> i32 {
    let rc = pm_runtime_get_sync(dev);
    if rc < 0 {
        pm_runtime_put_noidle(dev);
        return rc;
    }

    0
}

pub unsafe extern "C" fn cc_pm_put_suspend(dev: *mut device) {
    pm_runtime_put_autosuspend(dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
