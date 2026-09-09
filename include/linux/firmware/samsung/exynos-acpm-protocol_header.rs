/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2020 Samsung Electronics Co., Ltd.
 * Copyright 2020 Google LLC.
 * Copyright 2024 Linaro Ltd.
 */

// Dependency intent from <linux/types.h> is represented by Rust primitive types.

pub struct device_node;

#[repr(C)]
pub struct acpm_dvfs_ops {
    pub set_rate: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        clk_id: u32,
        rate: usize,
    ) -> i32>,
    pub get_rate: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        clk_id: u32,
    ) -> usize>,
}

#[repr(C)]
pub struct acpm_pmic_ops {
    pub read_reg: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        type_: u8,
        reg: u8,
        chan: u8,
        buf: *mut u8,
    ) -> i32>,
    pub bulk_read: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        type_: u8,
        reg: u8,
        chan: u8,
        count: u8,
        buf: *mut u8,
    ) -> i32>,
    pub write_reg: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        type_: u8,
        reg: u8,
        chan: u8,
        value: u8,
    ) -> i32>,
    pub bulk_write: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        type_: u8,
        reg: u8,
        chan: u8,
        count: u8,
        buf: *const u8,
    ) -> i32>,
    pub update_reg: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        type_: u8,
        reg: u8,
        chan: u8,
        value: u8,
        mask: u8,
    ) -> i32>,
}

#[repr(C)]
pub struct acpm_tmu_ops {
    pub init: Option<unsafe extern "C" fn(handle: *mut acpm_handle, acpm_chan_id: u32) -> i32>,
    pub read_temp: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        tz: u8,
        temp: *mut i32,
    ) -> i32>,
    pub set_threshold: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        tz: u8,
        temperature: *const u8,
        tlen: usize,
    ) -> i32>,
    pub set_interrupt_enable: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        tz: u8,
        inten: u8,
    ) -> i32>,
    pub tz_control: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        tz: u8,
        enable: bool,
    ) -> i32>,
    pub clear_tz_irq: Option<unsafe extern "C" fn(
        handle: *mut acpm_handle,
        acpm_chan_id: u32,
        tz: u8,
    ) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(handle: *mut acpm_handle, acpm_chan_id: u32) -> i32>,
    pub resume: Option<unsafe extern "C" fn(handle: *mut acpm_handle, acpm_chan_id: u32) -> i32>,
}

#[repr(C)]
pub struct acpm_ops {
    pub dvfs: acpm_dvfs_ops,
    pub pmic: acpm_pmic_ops,
    pub tmu: acpm_tmu_ops,
}

/**
 * struct acpm_handle - Reference to an initialized protocol instance
 * @ops: pointer to the constant ACPM protocol operations.
 */
#[repr(C)]
pub struct acpm_handle {
    pub ops: *const acpm_ops,
}

pub struct device;

unsafe extern "C" {
    pub fn devm_acpm_get_by_node(
        dev: *mut device,
        np: *mut device_node,
    ) -> *mut acpm_handle;
    pub fn devm_acpm_get_by_phandle(dev: *mut device) -> *mut acpm_handle;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
