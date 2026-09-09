/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

// Dependency intent from <linux/types.h> and "ivpu_drv.h" is preserved here.

#[allow(non_camel_case_types)]
pub type __u32 = u32;

#[repr(C)]
pub struct ivpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_printer {
    _private: [u8; 0],
}

pub const IVPU_FW_LOG_DEFAULT: i32 = 0;
pub const IVPU_FW_LOG_DEBUG: i32 = 1;
pub const IVPU_FW_LOG_INFO: i32 = 2;
pub const IVPU_FW_LOG_WARN: i32 = 3;
pub const IVPU_FW_LOG_ERROR: i32 = 4;
pub const IVPU_FW_LOG_FATAL: i32 = 5;

// These values correspond to SZ_1M, SZ_8M, and SZ_512K from the kernel headers.
pub const IVPU_FW_VERBOSE_BUFFER_SMALL_SIZE: usize = SZ_1M;
pub const IVPU_FW_VERBOSE_BUFFER_LARGE_SIZE: usize = SZ_8M;
pub const IVPU_FW_CRITICAL_BUFFER_SIZE: usize = SZ_512K;

extern "C" {
    pub static mut ivpu_fw_log_level: ::core::ffi::c_uint;

    pub fn ivpu_fw_log_print(
        vdev: *mut ivpu_device,
        only_new_msgs: bool,
        p: *mut drm_printer,
    );
    pub fn ivpu_fw_log_mark_read(vdev: *mut ivpu_device);
    pub fn ivpu_fw_log_reset(vdev: *mut ivpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
