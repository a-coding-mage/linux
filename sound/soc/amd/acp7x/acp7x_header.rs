/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Common ACP header file for ACP7.X variants(ACP7.D/7.E/7.F)
 *
 * Copyright (C) 2026 Advanced Micro Devices, Inc. All rights reserved.
 */

use core::ffi::{c_int, c_void};

// Dependencies from the original C includes:
// linux/device.h, linux/errno.h, linux/io.h, linux/pci.h, linux/types.h,
// and sound/acp7x_chip_offset_byte.h.

pub const ACP_DEVICE_ID: u32 = 0x15E2;
pub const ACP7X_REG_START: u32 = 0x1240000;
pub const ACP7X_REG_END: u32 = 0x125C000;

pub const ACP7D_PCI_REV: u32 = 0x7D;
pub const ACP7E_PCI_REV: u32 = 0x7E;
pub const ACP7F_PCI_REV: u32 = 0x7F;

/* Common register helper bits used by acp7x-common.c */
pub const ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK: u32 = 0x00010001;

pub const DELAY_US: u32 = 5;
pub const ACP7X_TIMEOUT: u32 = 5000;

pub const ACP7X_PGFSM_CNTL_POWER_ON_MASK: u32 = 7;
pub const ACP7X_PGFSM_STATUS_MASK: u32 = 0x3F;

/* time in ms for runtime suspend delay */
pub const ACP_SUSPEND_DELAY_MS: u32 = 2000;

#[repr(C)]
pub struct acp_hw_ops {
    pub acp_init: Option<unsafe extern "C" fn(acp_base: *mut c_void, dev: *mut device) -> c_int>,
    pub acp_deinit: Option<unsafe extern "C" fn(acp_base: *mut c_void, dev: *mut device) -> c_int>,
    pub acp_suspend: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub acp_resume: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub acp_suspend_runtime: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub acp_resume_runtime: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
}

#[repr(C)]
pub struct acp7x_dev_data {
    pub acp7x_base: *mut c_void,
    pub hw_ops: *mut acp_hw_ops,
    pub addr: u32,
    pub reg_range: u32,
    pub acp_rev: u32,
}

unsafe extern "C" {
    pub fn acp7x_hw_init_ops(hw_ops: *mut acp_hw_ops);
    pub fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_int;
}

pub unsafe fn acp_hw_init(adata: *mut acp7x_dev_data, dev: *mut device) -> c_int {
    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_init) = (*(*adata).hw_ops).acp_init {
            return unsafe { acp_init((*adata).acp7x_base, dev) };
        }
    }
    -EOPNOTSUPP
}

pub unsafe fn acp_hw_deinit(adata: *mut acp7x_dev_data, dev: *mut device) -> c_int {
    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_deinit) = (*(*adata).hw_ops).acp_deinit {
            return unsafe { acp_deinit((*adata).acp7x_base, dev) };
        }
    }
    -EOPNOTSUPP
}

pub unsafe fn acp_hw_suspend(dev: *mut device) -> c_int {
    let adata: *mut acp7x_dev_data = unsafe { dev_get_drvdata(dev) as *mut acp7x_dev_data };

    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_suspend) = (*(*adata).hw_ops).acp_suspend {
            return unsafe { acp_suspend(dev) };
        }
    }
    -EOPNOTSUPP
}

pub unsafe fn acp_hw_resume(dev: *mut device) -> c_int {
    let adata: *mut acp7x_dev_data = unsafe { dev_get_drvdata(dev) as *mut acp7x_dev_data };

    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_resume) = (*(*adata).hw_ops).acp_resume {
            return unsafe { acp_resume(dev) };
        }
    }
    -EOPNOTSUPP
}

pub unsafe fn acp_hw_suspend_runtime(dev: *mut device) -> c_int {
    let adata: *mut acp7x_dev_data = unsafe { dev_get_drvdata(dev) as *mut acp7x_dev_data };

    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_suspend_runtime) = (*(*adata).hw_ops).acp_suspend_runtime {
            return unsafe { acp_suspend_runtime(dev) };
        }
    }
    -EOPNOTSUPP
}

pub unsafe fn acp_hw_runtime_resume(dev: *mut device) -> c_int {
    let adata: *mut acp7x_dev_data = unsafe { dev_get_drvdata(dev) as *mut acp7x_dev_data };

    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_resume_runtime) = (*(*adata).hw_ops).acp_resume_runtime {
            return unsafe { acp_resume_runtime(dev) };
        }
    }
    -EOPNOTSUPP
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
