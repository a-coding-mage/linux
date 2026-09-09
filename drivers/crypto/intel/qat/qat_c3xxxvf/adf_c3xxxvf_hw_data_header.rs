/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2015 - 2020 Intel Corporation */

// Translated from adf_c3xxxvf_hw_data.h.
// The original header guard is omitted because Rust modules provide equivalent
// single-definition protection.

pub const ADF_C3XXXIOV_PMISC_BAR: i32 = 1;
pub const ADF_C3XXXIOV_ACCELERATORS_MASK: i32 = 0x1;
pub const ADF_C3XXXIOV_ACCELENGINES_MASK: i32 = 0x1;
pub const ADF_C3XXXIOV_MAX_ACCELERATORS: i32 = 1;
pub const ADF_C3XXXIOV_MAX_ACCELENGINES: i32 = 1;
pub const ADF_C3XXXIOV_RX_RINGS_OFFSET: i32 = 8;
pub const ADF_C3XXXIOV_TX_RINGS_MASK: i32 = 0xFF;
pub const ADF_C3XXXIOV_ETR_BAR: i32 = 0;
pub const ADF_C3XXXIOV_ETR_MAX_BANKS: i32 = 1;

// External dependency supplied by the surrounding translation unit/project.
pub struct adf_hw_device_data;

unsafe extern "C" {
    pub fn adf_init_hw_data_c3xxxiov(hw_data: *mut adf_hw_device_data);
    pub fn adf_clean_hw_data_c3xxxiov(hw_data: *mut adf_hw_device_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
