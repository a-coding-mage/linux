/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2025 Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the corresponding C/Rust translation units:
// #include "dc.h"
// #include "mod_hdcp.h"

extern "C" {
    pub fn dm_atomic_write_poll_read_i2c(
        link: *mut dc_link,
        write: *const mod_hdcp_atomic_op_i2c,
        poll: *const mod_hdcp_atomic_op_i2c,
        read: *mut mod_hdcp_atomic_op_i2c,
        poll_timeout_us: u32,
        poll_mask_msb: u8,
    ) -> bool;

    pub fn dm_atomic_write_poll_read_aux(
        link: *mut dc_link,
        write: *const mod_hdcp_atomic_op_aux,
        poll: *const mod_hdcp_atomic_op_aux,
        read: *mut mod_hdcp_atomic_op_aux,
        poll_timeout_us: u32,
        poll_mask_msb: u8,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
