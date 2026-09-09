/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2025 Intel Corporation */

// Forward declarations corresponding to the C structs.
#[repr(C)]
pub struct adf_hw_csr_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qat_migdev_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_pfvf_ops {
    _private: [u8; 0],
}

extern "C" {
    pub fn adf_gen6_init_pf_pfvf_ops(pfvf_ops: *mut adf_pfvf_ops);
    pub fn adf_gen6_init_hw_csr_ops(csr_ops: *mut adf_hw_csr_ops);
    pub fn adf_gen6_comp_dev_config(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_gen6_no_dev_config(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_gen6_init_vf_mig_ops(vfmig_ops: *mut qat_migdev_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
