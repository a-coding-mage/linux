/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2015 - 2020 Intel Corporation */

// C header guard: ADF_DH895XVF_HW_DATA_H_

pub const ADF_DH895XCCIOV_PMISC_BAR: i32 = 1;
pub const ADF_DH895XCCIOV_ACCELERATORS_MASK: i32 = 0x1;
pub const ADF_DH895XCCIOV_ACCELENGINES_MASK: i32 = 0x1;
pub const ADF_DH895XCCIOV_MAX_ACCELERATORS: i32 = 1;
pub const ADF_DH895XCCIOV_MAX_ACCELENGINES: i32 = 1;
pub const ADF_DH895XCCIOV_RX_RINGS_OFFSET: i32 = 8;
pub const ADF_DH895XCCIOV_TX_RINGS_MASK: i32 = 0xFF;
pub const ADF_DH895XCCIOV_ETR_BAR: i32 = 0;
pub const ADF_DH895XCCIOV_ETR_MAX_BANKS: i32 = 1;

unsafe extern "C" {
    pub fn adf_init_hw_data_dh895xcciov(hw_data: *mut adf_hw_device_data);
    pub fn adf_clean_hw_data_dh895xcciov(hw_data: *mut adf_hw_device_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
