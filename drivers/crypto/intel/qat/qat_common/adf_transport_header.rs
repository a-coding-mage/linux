/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// C dependency: adf_accel_devices.h

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_etr_ring_data {
    _private: [u8; 0],
}

pub type adf_callback_fn = Option<unsafe extern "C" fn(resp_msg: *mut core::ffi::c_void)>;

extern "C" {
    pub fn adf_create_ring(
        accel_dev: *mut adf_accel_dev,
        section: *const core::ffi::c_char,
        bank_num: u32,
        num_mgs: u32,
        msg_size: u32,
        ring_name: *const core::ffi::c_char,
        callback: adf_callback_fn,
        poll_mode: core::ffi::c_int,
        ring_ptr: *mut *mut adf_etr_ring_data,
    ) -> core::ffi::c_int;

    pub fn adf_ring_nearly_full(ring: *mut adf_etr_ring_data) -> bool;

    pub fn adf_send_message(
        ring: *mut adf_etr_ring_data,
        msg: *mut u32,
    ) -> core::ffi::c_int;

    pub fn adf_remove_ring(ring: *mut adf_etr_ring_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
