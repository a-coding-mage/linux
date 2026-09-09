/* SPDX-License-Identifier: GPL-2.0-only OR MIT */
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

use core::ffi::c_void;

// Dependency supplied by the Linux list implementation.
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

pub struct drm_device;
pub struct drm_file;
pub struct ivpu_bo;
pub struct ivpu_device;
pub struct ivpu_file_priv;

#[repr(C)]
pub struct ivpu_ms_instance {
    pub bo: *mut ivpu_bo,
    pub ms_instance_node: list_head,
    pub mask: u64,
    pub buff_size: u64,
    pub active_buff_vpu_addr: u64,
    pub inactive_buff_vpu_addr: u64,
    pub active_buff_ptr: *mut c_void,
    pub inactive_buff_ptr: *mut c_void,
    pub leftover_bytes: u64,
    pub leftover_addr: *mut c_void,
}

unsafe extern "C" {
    pub fn ivpu_ms_start_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file: *mut drm_file,
    ) -> i32;
    pub fn ivpu_ms_stop_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file: *mut drm_file,
    ) -> i32;
    pub fn ivpu_ms_get_data_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file: *mut drm_file,
    ) -> i32;
    pub fn ivpu_ms_get_info_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file: *mut drm_file,
    ) -> i32;
    pub fn ivpu_ms_cleanup(file_priv: *mut ivpu_file_priv);
    pub fn ivpu_ms_cleanup_all(vdev: *mut ivpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
