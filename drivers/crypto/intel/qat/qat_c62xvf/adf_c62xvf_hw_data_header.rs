/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2015 - 2020 Intel Corporation */

pub const ADF_C62XIOV_PMISC_BAR: u32 = 1;
pub const ADF_C62XIOV_ACCELERATORS_MASK: u32 = 0x1;
pub const ADF_C62XIOV_ACCELENGINES_MASK: u32 = 0x1;
pub const ADF_C62XIOV_MAX_ACCELERATORS: u32 = 1;
pub const ADF_C62XIOV_MAX_ACCELENGINES: u32 = 1;
pub const ADF_C62XIOV_RX_RINGS_OFFSET: u32 = 8;
pub const ADF_C62XIOV_TX_RINGS_MASK: u32 = 0xFF;
pub const ADF_C62XIOV_ETR_BAR: u32 = 0;
pub const ADF_C62XIOV_ETR_MAX_BANKS: u32 = 1;

pub enum adf_hw_device_data {}

unsafe extern "C" {
    pub fn adf_init_hw_data_c62xiov(hw_data: *mut adf_hw_device_data);
    pub fn adf_clean_hw_data_c62xiov(hw_data: *mut adf_hw_device_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
