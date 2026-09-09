/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2025 Intel Corporation */

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum adf_dc_algo {
    QAT_DEFLATE,
    QAT_LZ4,
    QAT_LZ4S,
    QAT_ZSTD,
}

unsafe extern "C" {
    pub fn qat_comp_build_ctx(
        accel_dev: *mut adf_accel_dev,
        ctx: *mut core::ffi::c_void,
        algo: adf_dc_algo,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
