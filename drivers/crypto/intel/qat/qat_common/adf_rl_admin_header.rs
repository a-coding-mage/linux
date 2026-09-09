/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

// Dependencies supplied by linux/types.h and adf_rl.h are intentionally not
// implemented here.

/// Opaque declaration supplied by the surrounding dependency set.
#[allow(non_camel_case_types)]
pub enum adf_accel_dev {}

/// Opaque declaration supplied by the surrounding dependency set.
#[allow(non_camel_case_types)]
pub enum rl_slice_cnt {}

/// Opaque declaration supplied by the surrounding dependency set.
#[allow(non_camel_case_types)]
pub enum rl_sla {}

/// Send the rate-limiting administration initialization message.
unsafe extern "C" {
    pub fn adf_rl_send_admin_init_msg(
        accel_dev: *mut adf_accel_dev,
        slices_int: *mut rl_slice_cnt,
    ) -> ::core::ffi::c_int;

    /// Send the rate-limiting administration add/update message.
    pub fn adf_rl_send_admin_add_update_msg(
        accel_dev: *mut adf_accel_dev,
        sla: *mut rl_sla,
        is_update: bool,
    ) -> ::core::ffi::c_int;

    /// Send the rate-limiting administration delete message.
    pub fn adf_rl_send_admin_delete_msg(
        accel_dev: *mut adf_accel_dev,
        node_id: u16,
        node_type: u8,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
