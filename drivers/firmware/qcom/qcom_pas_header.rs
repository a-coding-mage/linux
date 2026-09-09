/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// C header guard: __QCOM_PAS_INT_H

use core::ffi::c_void;

// Supplied by the surrounding kernel interfaces.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

// Supplied by the surrounding Qualcomm PAS interfaces.
#[repr(C)]
pub struct qcom_pas_context {
    _private: [u8; 0],
}

// C phys_addr_t; the concrete platform definition is supplied externally.
pub type phys_addr_t = usize;

/**
 * struct qcom_pas_ops - Qcom Peripheral Authentication Service (PAS) ops
 * @drv_name:                  PAS driver name.
 * @dev:                       PAS device pointer.
 * @supported:                 Peripheral supported callback.
 * @init_image:                Peripheral image initialization callback.
 * @mem_setup:                 Peripheral memory setup callback.
 * @get_rsc_table:             Peripheral get resource table callback.
 * @prepare_and_auth_reset:    Peripheral prepare firmware authentication and
 *                             reset callback.
 * @auth_and_reset:            Peripheral firmware authentication and reset
 *                             callback.
 * @set_remote_state:          Peripheral set remote state callback.
 * @shutdown:                  Peripheral shutdown callback.
 * @metadata_release:          Image metadata release callback.
 */
#[repr(C)]
pub struct qcom_pas_ops {
    pub drv_name: *const i8,
    pub dev: *mut device,
    pub supported:
        Option<unsafe extern "C" fn(dev: *mut device, pas_id: u32) -> bool>,
    pub init_image: Option<unsafe extern "C" fn(
        dev: *mut device,
        pas_id: u32,
        metadata: *const c_void,
        size: usize,
        ctx: *mut qcom_pas_context,
    ) -> i32>,
    pub mem_setup: Option<unsafe extern "C" fn(
        dev: *mut device,
        pas_id: u32,
        addr: phys_addr_t,
        size: phys_addr_t,
    ) -> i32>,
    pub get_rsc_table: Option<unsafe extern "C" fn(
        dev: *mut device,
        ctx: *mut qcom_pas_context,
        input_rt: *mut c_void,
        input_rt_size: usize,
        output_rt_size: *mut usize,
    ) -> *mut c_void>,
    pub prepare_and_auth_reset: Option<unsafe extern "C" fn(
        dev: *mut device,
        ctx: *mut qcom_pas_context,
    ) -> i32>,
    pub auth_and_reset:
        Option<unsafe extern "C" fn(dev: *mut device, pas_id: u32) -> i32>,
    pub set_remote_state: Option<unsafe extern "C" fn(
        dev: *mut device,
        state: u32,
        pas_id: u32,
    ) -> i32>,
    pub shutdown:
        Option<unsafe extern "C" fn(dev: *mut device, pas_id: u32) -> i32>,
    pub metadata_release: Option<unsafe extern "C" fn(
        dev: *mut device,
        ctx: *mut qcom_pas_context,
    )>,
}

unsafe extern "C" {
    pub fn qcom_pas_ops_register(ops: *mut qcom_pas_ops);
    pub fn qcom_pas_ops_unregister();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
