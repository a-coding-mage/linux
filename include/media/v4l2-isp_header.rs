/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Video4Linux2 generic ISP parameters and statistics support
 *
 * Copyright (C) 2025 Ideas On Board Oy
 * Author: Jacopo Mondi <jacopo.mondi@ideasonboard.com>
 */

// Dependency intent: declarations supplied by <linux/media/v4l2-isp.h>.

use core::ffi::c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vb2_buffer {
    _private: [u8; 0],
}

// v4l2_isp_buffer_size - Calculate size of v4l2_isp_buffer
// @max_size: The total size of the ISP configuration or statistics blocks
//
// Users of v4l2-isp will have differing sized data arrays for parameters and
// statistics, depending on their specific blocks. Drivers need to be able to
// calculate the appropriate size of the buffer to accommodate all ISP blocks
// supported by the platform. This macro provides a convenient tool for the
// calculation.
//
// The intended users of this function are drivers initializing the size
// of their metadata (parameters and statistics) buffers.
#[macro_export]
macro_rules! v4l2_isp_buffer_size {
    ($max_size:expr) => {
        core::mem::offset_of!(v4l2_isp_buffer, data) + ($max_size)
    };
}

extern "C" {
    pub fn v4l2_isp_params_validate_buffer_size(
        dev: *mut device,
        vb: *mut vb2_buffer,
        max_size: usize,
    ) -> c_int;

    // v4l2_isp_params_validate_buffer - Validate a V4L2 ISP parameters buffer
    pub fn v4l2_isp_params_validate_buffer(
        dev: *mut device,
        vb: *mut vb2_buffer,
        buffer: *const v4l2_isp_params_buffer,
        type_info: *const v4l2_isp_params_block_type_info,
        num_block_types: usize,
    ) -> c_int;

    pub fn v4l2_isp_stats_init_buffer(
        buf: *mut v4l2_isp_buffer,
        version: v4l2_isp_version,
    );

    pub fn v4l2_isp_stats_init_block(
        dev: *mut device,
        buf: *mut v4l2_isp_buffer,
        type_info: *const v4l2_isp_stats_block_type_info,
        num_block_types: usize,
        block_type: u32,
        max_size: usize,
    ) -> *mut v4l2_isp_block_header;
}

#[repr(C)]
pub struct v4l2_isp_params_block_type_info {
    pub size: usize,
    pub block_validate: Option<
        unsafe extern "C" fn(
            dev: *mut device,
            block: *const v4l2_isp_block_header,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct v4l2_isp_stats_block_type_info {
    pub size: usize,
}

// v4l2_isp_buffer, v4l2_isp_block_header, v4l2_isp_params_buffer, and
// v4l2_isp_version are supplied by <linux/media/v4l2-isp.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
