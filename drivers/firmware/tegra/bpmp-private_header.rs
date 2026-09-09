/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018, NVIDIA CORPORATION.
 */

// Dependency equivalent of <soc/tegra/bpmp.h>.
use crate::soc::tegra::bpmp::{tegra_bpmp, tegra_bpmp_channel};

#[repr(C)]
pub struct tegra_bpmp_ops {
    pub init: Option<unsafe extern "C" fn(bpmp: *mut tegra_bpmp) -> core::ffi::c_int>,
    pub deinit: Option<unsafe extern "C" fn(bpmp: *mut tegra_bpmp)>,
    pub is_response_ready:
        Option<unsafe extern "C" fn(channel: *mut tegra_bpmp_channel) -> bool>,
    pub is_request_ready:
        Option<unsafe extern "C" fn(channel: *mut tegra_bpmp_channel) -> bool>,
    pub ack_response:
        Option<unsafe extern "C" fn(channel: *mut tegra_bpmp_channel) -> core::ffi::c_int>,
    pub ack_request:
        Option<unsafe extern "C" fn(channel: *mut tegra_bpmp_channel) -> core::ffi::c_int>,
    pub is_response_channel_free:
        Option<unsafe extern "C" fn(channel: *mut tegra_bpmp_channel) -> bool>,
    pub is_request_channel_free:
        Option<unsafe extern "C" fn(channel: *mut tegra_bpmp_channel) -> bool>,
    pub post_response:
        Option<unsafe extern "C" fn(channel: *mut tegra_bpmp_channel) -> core::ffi::c_int>,
    pub post_request:
        Option<unsafe extern "C" fn(channel: *mut tegra_bpmp_channel) -> core::ffi::c_int>,
    pub ring_doorbell:
        Option<unsafe extern "C" fn(bpmp: *mut tegra_bpmp) -> core::ffi::c_int>,
    pub resume: Option<unsafe extern "C" fn(bpmp: *mut tegra_bpmp) -> core::ffi::c_int>,
}

unsafe extern "C" {
    pub static tegra186_bpmp_ops: tegra_bpmp_ops;
    pub static tegra210_bpmp_ops: tegra_bpmp_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
