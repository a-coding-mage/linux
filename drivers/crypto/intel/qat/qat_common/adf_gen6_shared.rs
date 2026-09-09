// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2025 Intel Corporation */

// Dependencies supplied by the corresponding C headers:
// adf_gen4_config.h, adf_gen4_hw_csr_data.h, adf_gen4_pfvf.h,
// adf_gen4_vf_mig.h, and adf_gen6_shared.h.

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_pfvf_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_hw_csr_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qat_migdev_ops {
    _private: [u8; 0],
}

extern "C" {
    fn adf_gen4_init_pfvf_ops(pfvf_ops: *mut adf_pfvf_ops);
    fn adf_gen4_init_hw_csr_ops(csr_ops: *mut adf_hw_csr_ops);
    fn adf_comp_dev_config(accel_dev: *mut adf_accel_dev) -> i32;
    fn adf_no_dev_config(accel_dev: *mut adf_accel_dev) -> i32;
    fn adf_gen4_init_vf_mig_ops(vfmig_ops: *mut qat_migdev_ops);
}

/*
 * QAT GEN4 and GEN6 devices often differ in terms of supported features,
 * options and internal logic. However, some of the mechanisms and register
 * layout are shared between those two GENs. This file serves as an abstraction
 * layer that allows to use existing GEN4 implementation that is also
 * applicable to GEN6 without additional overhead and complexity.
 */
#[no_mangle]
pub unsafe extern "C" fn adf_gen6_init_pf_pfvf_ops(pfvf_ops: *mut adf_pfvf_ops) {
    adf_gen4_init_pfvf_ops(pfvf_ops);
}

#[no_mangle]
pub unsafe extern "C" fn adf_gen6_init_hw_csr_ops(csr_ops: *mut adf_hw_csr_ops) {
    adf_gen4_init_hw_csr_ops(csr_ops);
}

#[no_mangle]
pub unsafe extern "C" fn adf_gen6_comp_dev_config(accel_dev: *mut adf_accel_dev) -> i32 {
    adf_comp_dev_config(accel_dev)
}

#[no_mangle]
pub unsafe extern "C" fn adf_gen6_no_dev_config(accel_dev: *mut adf_accel_dev) -> i32 {
    adf_no_dev_config(accel_dev)
}

#[no_mangle]
pub unsafe extern "C" fn adf_gen6_init_vf_mig_ops(vfmig_ops: *mut qat_migdev_ops) {
    adf_gen4_init_vf_mig_ops(vfmig_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
