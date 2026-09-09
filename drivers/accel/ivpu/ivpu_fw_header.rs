/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2025 Intel Corporation
 */

// Translated from ivpu_fw.h. Dependencies supplied by the surrounding
// translation unit: vpu_boot_api.h and vpu_jsm_api.h.

pub const FW_VERSION_HEADER_SIZE: usize = SZ_4K;
pub const FW_VERSION_STR_SIZE: usize = SZ_256;

pub enum ivpu_device {}
pub enum ivpu_bo {}
pub enum vpu_boot_params {}
pub enum firmware {}
pub enum ivpu_addr_range {}

#[repr(C)]
pub struct ivpu_fw_info {
    pub file: *const firmware,
    pub name: *const core::ffi::c_char,
    pub version: [core::ffi::c_char; FW_VERSION_STR_SIZE],
    pub mem_bp: *mut ivpu_bo,
    pub mem_fw_ver: *mut ivpu_bo,
    pub mem: *mut ivpu_bo,
    pub mem_shave_nn: *mut ivpu_bo,
    pub mem_log_crit: *mut ivpu_bo,
    pub mem_log_verb: *mut ivpu_bo,
    pub boot_params_addr: u64,
    pub boot_params_size: u64,
    pub fw_version_addr: u64,
    pub fw_version_size: u64,
    pub runtime_addr: u64,
    pub runtime_size: u32,
    pub image_load_offset: u64,
    pub image_size: u32,
    pub shave_nn_size: u32,
    pub warm_boot_entry_point: u64,
    pub cold_boot_entry_point: u64,
    pub last_boot_mode: u8,
    pub next_boot_mode: u8,
    pub trace_level: u32,
    pub trace_destination_mask: u32,
    pub trace_hw_component_mask: u64,
    pub dvfs_mode: u32,
    pub primary_preempt_buf_size: u32,
    pub secondary_preempt_buf_size: u32,
    pub read_only_addr: u64,
    pub read_only_size: u32,
    pub sched_mode: u32,
    pub last_heartbeat: u64,
}

unsafe extern "C" {
    pub fn ivpu_is_within_range(
        addr: u64,
        size: usize,
        range: *mut ivpu_addr_range,
    ) -> bool;
    pub fn ivpu_fw_init(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_fw_fini(vdev: *mut ivpu_device);
    pub fn ivpu_fw_load(vdev: *mut ivpu_device);
    pub fn ivpu_fw_boot_params_setup(
        vdev: *mut ivpu_device,
        boot_params: *mut vpu_boot_params,
    );
}

// The complete definition of ivpu_device is provided by its owning
// translation unit. These views preserve the header's direct field access.
#[repr(C)]
struct ivpu_device_fw_view {
    fw: *mut ivpu_fw_info,
}

#[inline]
pub unsafe fn ivpu_fw_is_warm_boot(vdev: *mut ivpu_device) -> bool {
    (*((vdev as *mut ivpu_device_fw_view))).fw.as_ref().unwrap().next_boot_mode
        == VPU_BOOT_TYPE_WARMBOOT
}

#[inline]
pub unsafe fn ivpu_fw_preempt_buf_size(vdev: *mut ivpu_device) -> u32 {
    let fw = (*((vdev as *mut ivpu_device_fw_view))).fw.as_ref().unwrap();
    fw.primary_preempt_buf_size
        .wrapping_add(fw.secondary_preempt_buf_size)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
