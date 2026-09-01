/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2024-2025 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

/* Translated from C header: includes linux/cleanup.h, messages.h, registers.h. */

pub type s32 = i32;
pub type u32 = u32;
pub type u8 = u8;

#[repr(C)]
pub struct avs_dev {
    _unused: [u8; 0],
}

#[repr(C)]
pub union avs_notify_msg {
    _unused: [u8; 0],
}

pub const AVS_DEBUG_WINDOW: u32 = 0;

unsafe extern "C" {
    pub fn avs_sram_addr(adev: *mut avs_dev, window: u32) -> *mut u8;
    pub fn avs_dsp_op_log_buffer_offset(adev: *mut avs_dev, core: u32) -> s32;
    pub fn avs_dsp_op_log_buffer_status(adev: *mut avs_dev, msg: *mut avs_notify_msg) -> i32;
}

/*
 * C macro:
 * avs_log_buffer_size(adev) =
 *     (adev)->fw_cfg.trace_log_bytes / (adev)->hw_cfg.dsp_cores
 *
 * The avs_dev field layout is supplied by other headers.
 */

#[inline]
pub unsafe fn avs_log_buffer_addr(adev: *mut avs_dev, core: u32) -> *mut u8 {
    let __offset: s32 = unsafe { avs_dsp_op_log_buffer_offset(adev, core) };

    if __offset < 0 {
        core::ptr::null_mut()
    } else {
        unsafe { avs_sram_addr(adev, AVS_DEBUG_WINDOW).add(__offset as usize) }
    }
}

#[inline]
pub unsafe fn avs_log_buffer_status_locked(adev: *mut avs_dev, msg: *mut avs_notify_msg) -> i32 {
    /*
     * C used guard(spinlock_irqsave)(&adev->trace_lock) to hold adev->trace_lock
     * for this call. The lock field and guard helper are supplied by other headers.
     */
    unsafe { avs_dsp_op_log_buffer_status(adev, msg) }
}

#[repr(C, packed)]
pub struct avs_apl_log_buffer_layout {
    pub read_ptr: u32,
    pub write_ptr: u32,
    pub buffer: [u8; 0],
}

const _: [(); 8] = [(); core::mem::size_of::<avs_apl_log_buffer_layout>()];

/*
 * C macro:
 * avs_apl_log_payload_size(adev) =
 *     avs_log_buffer_size(adev) - sizeof(struct avs_apl_log_buffer_layout)
 *
 * The avs_log_buffer_size expression depends on avs_dev fields declared in
 * other headers.
 */

#[inline]
pub unsafe fn avs_apl_log_payload_addr(addr: *mut u8) -> *mut u8 {
    unsafe { addr.add(core::mem::size_of::<avs_apl_log_buffer_layout>()) }
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe extern "C" {
    pub fn avs_register_probe_component(adev: *mut avs_dev, name: *const core::ffi::c_char) -> i32;
    pub fn avs_logging_fw(adev: *mut avs_dev) -> bool;
    pub fn avs_dump_fw_log(adev: *mut avs_dev, src: *const core::ffi::c_void, len: u32);
    pub fn avs_dump_fw_log_wakeup(adev: *mut avs_dev, src: *const core::ffi::c_void, len: u32);
    pub fn avs_debugfs_init(adev: *mut avs_dev);
    pub fn avs_debugfs_exit(adev: *mut avs_dev);
}

/*
 * C macro under CONFIG_DEBUG_FS:
 * AVS_SET_ENABLE_LOGS_OP(name) expands to:
 *     .enable_logs = avs_##name##_enable_logs
 */

#[cfg(not(CONFIG_DEBUG_FS))]
pub const EOPNOTSUPP: i32 = 95;

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub unsafe fn avs_register_probe_component(
    _adev: *mut avs_dev,
    _name: *const core::ffi::c_char,
) -> i32 {
    -EOPNOTSUPP
}

/*
 * C macro when CONFIG_DEBUG_FS is not set:
 * AVS_SET_ENABLE_LOGS_OP(name) expands to nothing.
 */

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub unsafe fn avs_logging_fw(_adev: *mut avs_dev) -> bool {
    false
}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub unsafe fn avs_dump_fw_log(_adev: *mut avs_dev, _src: *const core::ffi::c_void, _len: u32) {}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub unsafe fn avs_dump_fw_log_wakeup(
    _adev: *mut avs_dev,
    _src: *const core::ffi::c_void,
    _len: u32,
) {
}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub unsafe fn avs_debugfs_init(_adev: *mut avs_dev) {}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub unsafe fn avs_debugfs_exit(_adev: *mut avs_dev) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
