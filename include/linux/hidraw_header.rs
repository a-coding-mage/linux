/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (c) 2007 Jiri Kosina
 */

// Dependency: <uapi/linux/hidraw.h>

#[repr(C)]
pub struct hidraw {
    pub minor: ::core::ffi::c_uint,
    pub exist: ::core::ffi::c_int,
    pub open: ::core::ffi::c_int,
    pub wait: wait_queue_head_t,
    pub hid: *mut hid_device,
    pub dev: *mut device,
    pub list_lock: spinlock_t,
    pub list: list_head,
}

#[repr(C)]
pub struct hidraw_report {
    pub value: *mut u8,
    pub len: ::core::ffi::c_int,
}

#[repr(C)]
pub struct hidraw_list {
    pub buffer: [hidraw_report; HIDRAW_BUFFER_SIZE],
    pub head: ::core::ffi::c_int,
    pub tail: ::core::ffi::c_int,
    pub fasync: *mut fasync_struct,
    pub hidraw: *mut hidraw,
    pub node: list_head,
    pub read_mutex: mutex,
    pub revoked: bool,
}

#[cfg(feature = "CONFIG_HIDRAW")]
extern "C" {
    pub fn hidraw_init() -> ::core::ffi::c_int;
    pub fn hidraw_exit();
    pub fn hidraw_report_event(hid: *mut hid_device, data: *mut u8, len: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn hidraw_connect(hid: *mut hid_device) -> ::core::ffi::c_int;
    pub fn hidraw_disconnect(hid: *mut hid_device);
}

#[cfg(not(feature = "CONFIG_HIDRAW"))]
#[inline]
pub unsafe fn hidraw_init() -> ::core::ffi::c_int { 0 }

#[cfg(not(feature = "CONFIG_HIDRAW"))]
#[inline]
pub unsafe fn hidraw_exit() {}

#[cfg(not(feature = "CONFIG_HIDRAW"))]
#[inline]
pub unsafe fn hidraw_report_event(_hid: *mut hid_device, _data: *mut u8, _len: ::core::ffi::c_int) -> ::core::ffi::c_int { 0 }

#[cfg(not(feature = "CONFIG_HIDRAW"))]
#[inline]
pub unsafe fn hidraw_connect(_hid: *mut hid_device) -> ::core::ffi::c_int { -1 }

#[cfg(not(feature = "CONFIG_HIDRAW"))]
#[inline]
pub unsafe fn hidraw_disconnect(_hid: *mut hid_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
