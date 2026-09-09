/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2021-2022, NVIDIA CORPORATION. All rights reserved
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Supplied by the corresponding kernel headers.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tegra_cbb_error {
    pub code: *const c_char,
    pub source: *const c_char,
    pub desc: *const c_char,
}

#[repr(C)]
pub struct tegra_cbb {
    pub dev: *mut device,
    pub ops: *const tegra_cbb_ops,
    pub node: list_head,
}

#[repr(C)]
pub struct tegra_cbb_ops {
    pub debugfs_show:
        Option<unsafe extern "C" fn(cbb: *mut tegra_cbb, s: *mut seq_file, v: *mut c_void) -> c_int>,
    pub interrupt_enable: Option<unsafe extern "C" fn(cbb: *mut tegra_cbb) -> c_int>,
    pub error_enable: Option<unsafe extern "C" fn(cbb: *mut tegra_cbb)>,
    pub fault_enable: Option<unsafe extern "C" fn(cbb: *mut tegra_cbb)>,
    pub stall_enable: Option<unsafe extern "C" fn(cbb: *mut tegra_cbb)>,
    pub error_clear: Option<unsafe extern "C" fn(cbb: *mut tegra_cbb)>,
    pub get_status: Option<unsafe extern "C" fn(cbb: *mut tegra_cbb) -> u32>,
}

extern "C" {
    pub fn tegra_cbb_get_irq(
        pdev: *mut platform_device,
        nonsec_irq: *mut c_uint,
        sec_irq: *mut c_uint,
    ) -> c_int;

    pub fn tegra_cbb_print_err(file: *mut seq_file, fmt: *const c_char, ...);

    pub fn tegra_cbb_print_cache(file: *mut seq_file, cache: u32);
    pub fn tegra_cbb_print_prot(file: *mut seq_file, prot: u32);
    pub fn tegra_cbb_register(cbb: *mut tegra_cbb) -> c_int;

    pub fn tegra_cbb_fault_enable(cbb: *mut tegra_cbb);
    pub fn tegra_cbb_stall_enable(cbb: *mut tegra_cbb);
    pub fn tegra_cbb_error_clear(cbb: *mut tegra_cbb);
    pub fn tegra_cbb_get_status(cbb: *mut tegra_cbb) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
