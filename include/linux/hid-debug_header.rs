/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Copyright (c) 2007-2009	Jiri Kosina
 */

/* The declarations below are enabled when CONFIG_DEBUG_FS is enabled. */
#[cfg(CONFIG_DEBUG_FS)]
pub const HID_DEBUG_BUFSIZE: usize = 512;
#[cfg(CONFIG_DEBUG_FS)]
pub const HID_DEBUG_FIFOSIZE: usize = 512;

#[cfg(CONFIG_DEBUG_FS)]
pub type __s32 = i32;

#[cfg(CONFIG_DEBUG_FS)]
pub enum hid_device {}
#[cfg(CONFIG_DEBUG_FS)]
pub enum hid_usage {}
#[cfg(CONFIG_DEBUG_FS)]
pub enum seq_file {}
#[cfg(CONFIG_DEBUG_FS)]
pub enum hid_field {}
#[cfg(CONFIG_DEBUG_FS)]
pub enum fasync_struct {}
#[cfg(CONFIG_DEBUG_FS)]
pub enum list_head {}
#[cfg(CONFIG_DEBUG_FS)]
pub enum mutex {}

#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub fn hid_dump_input(hdev: *mut hid_device, usage: *mut hid_usage, value: __s32);
    pub fn hid_dump_report(hdev: *mut hid_device, report_type: i32, data: *mut u8, size: i32);
    pub fn hid_dump_device(hdev: *mut hid_device, f: *mut seq_file);
    pub fn hid_dump_field(field: *mut hid_field, n: i32, f: *mut seq_file);
    pub fn hid_resolv_usage(usage: u32, f: *mut seq_file) -> *mut core::ffi::c_char;
    pub fn hid_debug_register(hdev: *mut hid_device, name: *const core::ffi::c_char);
    pub fn hid_debug_unregister(hdev: *mut hid_device);
    pub fn hid_debug_init();
    pub fn hid_debug_exit();
    pub fn hid_debug_event(hdev: *mut hid_device, buf: *mut core::ffi::c_char);
}

#[cfg(CONFIG_DEBUG_FS)]
#[repr(C)]
pub struct hid_debug_list {
    /* DECLARE_KFIFO_PTR(hid_debug_fifo, char); */
    pub hid_debug_fifo: *mut core::ffi::c_void,
    pub fasync: *mut fasync_struct,
    pub hdev: *mut hid_device,
    pub node: *mut list_head,
    pub read_mutex: *mut mutex,
}

/* CONFIG_DEBUG_FS disabled: the C macros expand to no-op statements. */
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline(always)]
pub unsafe fn hid_dump_input(_a: *mut core::ffi::c_void, _b: *mut core::ffi::c_void, _c: i32) {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline(always)]
pub unsafe fn hid_dump_report(_a: *mut core::ffi::c_void, _b: i32, _c: *mut u8, _d: i32) {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline(always)]
pub unsafe fn hid_dump_device(_a: *mut core::ffi::c_void, _b: *mut core::ffi::c_void) {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline(always)]
pub unsafe fn hid_dump_field(_a: *mut core::ffi::c_void, _b: i32, _c: *mut core::ffi::c_void) {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline(always)]
pub unsafe fn hid_resolv_usage(_a: u32, _b: *mut core::ffi::c_void) {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline(always)]
pub unsafe fn hid_debug_register(_a: *mut core::ffi::c_void, _b: *const core::ffi::c_char) {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline(always)]
pub unsafe fn hid_debug_unregister(_a: *mut core::ffi::c_void) {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline(always)]
pub unsafe fn hid_debug_init() {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline(always)]
pub unsafe fn hid_debug_exit() {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline(always)]
pub unsafe fn hid_debug_event(_a: *mut core::ffi::c_void, _b: *mut core::ffi::c_char) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
