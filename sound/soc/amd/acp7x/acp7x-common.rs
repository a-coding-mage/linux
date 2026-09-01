// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD ACP PCI driver callback routines for ACP7.x
 * platforms.
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

use core::ffi::{c_int, c_void};

// C dependencies:
// #include <linux/device.h>
// #include <linux/io.h>
// #include <linux/iopoll.h>
// #include <linux/types.h>
// #include "acp7x.h"

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acp7x_dev_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acp_hw_ops {
    pub acp_init: Option<unsafe extern "C" fn(*mut c_void, *mut device) -> c_int>,
    pub acp_deinit: Option<unsafe extern "C" fn(*mut c_void, *mut device) -> c_int>,
    pub acp_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub acp_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub acp_suspend_runtime: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub acp_resume_runtime: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn acp_hw_deinit(adata: *mut acp7x_dev_data, dev: *mut device) -> c_int;
    fn acp_hw_init(adata: *mut acp7x_dev_data, dev: *mut device) -> c_int;
}

unsafe extern "C" fn acp7x_power_on(acp_base: *mut c_void) -> c_int {
    let mut val: u32 = 0;

    val = readl((acp_base as *mut u8).add(ACP_PGFSM_STATUS as usize) as *const c_void);
    if (val & ACP7X_PGFSM_STATUS_MASK) == 0 {
        return 0;
    }

    writel(
        ACP7X_PGFSM_CNTL_POWER_ON_MASK,
        (acp_base as *mut u8).add(ACP_PGFSM_CONTROL as usize) as *mut c_void,
    );
    val = readl((acp_base as *mut u8).add(ACP_PGFSM_CONTROL as usize) as *const c_void);
    return readl_poll_timeout!(
        (acp_base as *mut u8).add(ACP_PGFSM_STATUS as usize) as *const c_void,
        val,
        ((val & ACP7X_PGFSM_STATUS_MASK) == 0),
        DELAY_US,
        ACP7X_TIMEOUT
    );
}

unsafe extern "C" fn acp7x_reset(acp_base: *mut c_void) -> c_int {
    let mut val: u32;
    let ret: c_int;

    writel(1, (acp_base as *mut u8).add(ACP_SOFT_RESET as usize) as *mut c_void);
    ret = readl_poll_timeout!(
        (acp_base as *mut u8).add(ACP_SOFT_RESET as usize) as *const c_void,
        val,
        val & ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK != 0,
        DELAY_US,
        ACP7X_TIMEOUT
    );
    if ret != 0 {
        return ret;
    }

    writel(0, (acp_base as *mut u8).add(ACP_SOFT_RESET as usize) as *mut c_void);
    return readl_poll_timeout!(
        (acp_base as *mut u8).add(ACP_SOFT_RESET as usize) as *const c_void,
        val,
        val == 0,
        DELAY_US,
        ACP7X_TIMEOUT
    );
}

unsafe extern "C" fn acp7x_init(acp_base: *mut c_void, dev: *mut device) -> c_int {
    let mut ret: c_int;

    ret = acp7x_power_on(acp_base);
    if ret != 0 {
        dev_err(dev, c"ACP power on failed\n".as_ptr() as *const u8);
        return ret;
    }
    writel(0x01, (acp_base as *mut u8).add(ACP_CONTROL as usize) as *mut c_void);
    ret = acp7x_reset(acp_base);
    if ret != 0 {
        dev_err(dev, c"ACP reset failed\n".as_ptr() as *const u8);
        return ret;
    }
    writel(0, (acp_base as *mut u8).add(ACP_ZSC_DSP_CTRL as usize) as *mut c_void);
    return 0;
}

unsafe extern "C" fn acp7x_deinit(acp_base: *mut c_void, dev: *mut device) -> c_int {
    let ret: c_int;

    ret = acp7x_reset(acp_base);
    if ret != 0 {
        dev_err(dev, c"ACP reset failed\n".as_ptr() as *const u8);
        return ret;
    }
    writel(0x01, (acp_base as *mut u8).add(ACP_ZSC_DSP_CTRL as usize) as *mut c_void);
    return 0;
}

// __maybe_unused
unsafe extern "C" fn snd_acp7x_suspend(dev: *mut device) -> c_int {
    let adata: *mut acp7x_dev_data;
    let ret: c_int;

    adata = dev_get_drvdata(dev) as *mut acp7x_dev_data;
    ret = acp_hw_deinit(adata, dev);
    if ret != 0 {
        dev_err(dev, c"ACP de-init failed\n".as_ptr() as *const u8);
    }
    return ret;
}

// __maybe_unused
unsafe extern "C" fn snd_acp7x_runtime_resume(dev: *mut device) -> c_int {
    let adata: *mut acp7x_dev_data;
    let ret: c_int;

    adata = dev_get_drvdata(dev) as *mut acp7x_dev_data;
    ret = acp_hw_init(adata, dev);
    if ret != 0 {
        dev_err(dev, c"ACP init failed\n".as_ptr() as *const u8);
        return ret;
    }
    return 0;
}

// __maybe_unused
unsafe extern "C" fn snd_acp7x_resume(dev: *mut device) -> c_int {
    let adata: *mut acp7x_dev_data;
    let ret: c_int;

    adata = dev_get_drvdata(dev) as *mut acp7x_dev_data;
    ret = acp_hw_init(adata, dev);
    if ret != 0 {
        dev_err(dev, c"ACP init failed\n".as_ptr() as *const u8);
    }

    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn acp7x_hw_init_ops(hw_ops: *mut acp_hw_ops) {
    (*hw_ops).acp_init = Some(acp7x_init);
    (*hw_ops).acp_deinit = Some(acp7x_deinit);
    (*hw_ops).acp_suspend = Some(snd_acp7x_suspend);
    (*hw_ops).acp_resume = Some(snd_acp7x_resume);
    (*hw_ops).acp_suspend_runtime = Some(snd_acp7x_suspend);
    (*hw_ops).acp_resume_runtime = Some(snd_acp7x_runtime_resume);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
