/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Translated from adf_transport_internal.h.
// Linux header dependencies and symbols supplied by other translation units
// remain external dependencies of this header translation.

#[repr(C)]
pub struct adf_etr_ring_debug_entry {
    pub ring_name: [core::ffi::c_char; ADF_CFG_MAX_KEY_LEN_IN_BYTES as usize],
    pub debug: *mut dentry,
}

#[repr(C)]
pub struct adf_etr_ring_data {
    pub base_addr: *mut core::ffi::c_void,
    pub inflights: *mut atomic_t,
    pub callback: adf_callback_fn,
    pub bank: *mut adf_etr_bank_data,
    pub dma_addr: dma_addr_t,
    pub ring_debug: *mut adf_etr_ring_debug_entry,
    pub lock: spinlock_t, /* protects ring data struct */
    pub head: u16,
    pub tail: u16,
    pub threshold: u32,
    pub ring_number: u8,
    pub ring_size: u8,
    pub msg_size: u8,
}

#[repr(C)]
pub struct adf_etr_bank_data {
    pub rings: *mut adf_etr_ring_data,
    pub resp_handler: tasklet_struct,
    pub csr_addr: *mut core::ffi::c_void,
    pub irq_coalesc_timer: u32,
    pub bank_number: u32,
    pub ring_mask: u16,
    pub irq_mask: u16,
    pub lock: spinlock_t, /* protects bank data struct */
    pub accel_dev: *mut adf_accel_dev,
    pub bank_debug_dir: *mut dentry,
    pub bank_debug_cfg: *mut dentry,
}

#[repr(C)]
pub struct adf_etr_data {
    pub banks: *mut adf_etr_bank_data,
    pub debug: *mut dentry,
}

extern "C" {
    pub fn adf_response_handler(bank_addr: usize);
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn adf_bank_debugfs_add(bank: *mut adf_etr_bank_data) -> i32;
    pub fn adf_bank_debugfs_rm(bank: *mut adf_etr_bank_data);
    pub fn adf_ring_debugfs_add(
        ring: *mut adf_etr_ring_data,
        name: *const core::ffi::c_char,
    ) -> i32;
    pub fn adf_ring_debugfs_rm(ring: *mut adf_etr_ring_data);
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn adf_bank_debugfs_add(_bank: *mut adf_etr_bank_data) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn adf_bank_debugfs_rm(_bank: *mut adf_etr_bank_data) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn adf_ring_debugfs_add(
    _ring: *mut adf_etr_ring_data,
    _name: *const core::ffi::c_char,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn adf_ring_debugfs_rm(_ring: *mut adf_etr_ring_data) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
