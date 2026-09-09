/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2021 Intel Corporation */

// External declarations supplied by the surrounding translation unit.
#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pfvf_message {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn adf_send_pf2vf_msg(
        accel_dev: *mut adf_accel_dev,
        vf_nr: u8,
        msg: pfvf_message,
    ) -> i32;

    pub fn adf_enable_pf2vf_comms(accel_dev: *mut adf_accel_dev) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
