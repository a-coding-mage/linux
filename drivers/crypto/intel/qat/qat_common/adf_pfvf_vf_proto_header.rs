/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2021 Intel Corporation */

// Declarations supplied by the included kernel and device headers.
pub struct adf_accel_dev;
pub struct pfvf_message;

extern "C" {
    pub fn adf_send_vf2pf_msg(
        accel_dev: *mut adf_accel_dev,
        msg: pfvf_message,
    ) -> ::core::ffi::c_int;

    pub fn adf_send_vf2pf_req(
        accel_dev: *mut adf_accel_dev,
        msg: pfvf_message,
        resp: *mut pfvf_message,
    ) -> ::core::ffi::c_int;

    pub fn adf_send_vf2pf_blkmsg_req(
        accel_dev: *mut adf_accel_dev,
        type_: u8,
        buffer: *mut u8,
        buffer_len: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn adf_enable_vf2pf_comms(
        accel_dev: *mut adf_accel_dev,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
