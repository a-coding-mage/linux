/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

// C header guard: ADF_CNV_DBG_H

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

extern "C" {
    pub fn adf_cnv_dbgfs_add(accel_dev: *mut adf_accel_dev);
    pub fn adf_cnv_dbgfs_rm(accel_dev: *mut adf_accel_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
