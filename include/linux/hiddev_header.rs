/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (c) 1999-2000 Vojtech Pavlik
 *
 *  Sponsored by SuSE
 */
/*
 *
 * Should you need to contact me, the author, you can do so either by
 * e-mail - mail your message to <vojtech@suse.cz>, or by paper mail:
 * Vojtech Pavlik, Ucitelska 1576, Prague 8, 182 00 Czech Republic
 */

// Dependency supplied by the corresponding kernel interfaces:
// #include <uapi/linux/hiddev.h>

/*
 * In-kernel definitions.
 */

#[repr(C)]
pub struct hiddev {
    pub minor: ::core::ffi::c_int,
    pub exist: ::core::ffi::c_int,
    pub open: ::core::ffi::c_int,
    pub existancelock: mutex,
    pub wait: wait_queue_head_t,
    pub hid: *mut hid_device,
    pub list: list_head,
    pub list_lock: spinlock_t,
    pub initialized: bool,
}

#[repr(C)]
pub struct hid_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hid_usage {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hid_field {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hid_report {
    _private: [u8; 0],
}

// These types are supplied by the kernel synchronization and list interfaces.
// struct mutex;
// typedef wait_queue_head_t;
// struct list_head;
// typedef spinlock_t;

#[cfg(feature = "CONFIG_USB_HIDDEV")]
unsafe extern "C" {
    pub fn hiddev_connect(hid: *mut hid_device, force: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn hiddev_disconnect(hid: *mut hid_device);
    pub fn hiddev_hid_event(
        hid: *mut hid_device,
        field: *mut hid_field,
        usage: *mut hid_usage,
        value: i32,
    );
    pub fn hiddev_report_event(hid: *mut hid_device, report: *mut hid_report);
}

#[cfg(not(feature = "CONFIG_USB_HIDDEV"))]
#[inline]
pub unsafe fn hiddev_connect(_hid: *mut hid_device, _force: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    -1
}

#[cfg(not(feature = "CONFIG_USB_HIDDEV"))]
#[inline]
pub unsafe fn hiddev_disconnect(_hid: *mut hid_device) {}

#[cfg(not(feature = "CONFIG_USB_HIDDEV"))]
#[inline]
pub unsafe fn hiddev_hid_event(
    _hid: *mut hid_device,
    _field: *mut hid_field,
    _usage: *mut hid_usage,
    _value: i32,
) {
}

#[cfg(not(feature = "CONFIG_USB_HIDDEV"))]
#[inline]
pub unsafe fn hiddev_report_event(_hid: *mut hid_device, _report: *mut hid_report) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
