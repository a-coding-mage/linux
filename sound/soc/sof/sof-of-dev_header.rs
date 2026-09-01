/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright 2021 NXP
 */

#[repr(C)]
pub struct snd_sof_of_mach {
    pub compatible: *const core::ffi::c_char,
    pub drv_name: *const core::ffi::c_char,
    pub fw_filename: *const core::ffi::c_char,
    pub sof_tplg_filename: *const core::ffi::c_char,
}

unsafe extern "C" {
    pub static sof_of_pm: dev_pm_ops;

    pub fn sof_of_probe(pdev: *mut platform_device) -> core::ffi::c_int;
    pub fn sof_of_remove(pdev: *mut platform_device);
    pub fn sof_of_shutdown(pdev: *mut platform_device);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
