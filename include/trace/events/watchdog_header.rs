/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of trace/events/watchdog.h. */

/* Supplied by the Linux watchdog interface. */
#[repr(C)]
pub struct watchdog_device {
    pub id: ::core::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct watchdog_template_entry {
    pub id: ::core::ffi::c_int,
    pub err: ::core::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct watchdog_set_timeout_entry {
    pub id: ::core::ffi::c_int,
    pub timeout: ::core::ffi::c_uint,
    pub err: ::core::ffi::c_int,
}

#[inline]
pub unsafe fn watchdog_template_assign(
    entry: *mut watchdog_template_entry,
    wdd: *mut watchdog_device,
    err: ::core::ffi::c_int,
) {
    (*entry).id = (*wdd).id;
    (*entry).err = err;
}

#[inline]
pub unsafe fn watchdog_set_timeout_assign(
    entry: *mut watchdog_set_timeout_entry,
    wdd: *mut watchdog_device,
    timeout: ::core::ffi::c_uint,
    err: ::core::ffi::c_int,
) {
    (*entry).id = (*wdd).id;
    (*entry).timeout = timeout;
    (*entry).err = err;
}

/* TP_printk formats: "watchdog%d err=%d" and
 * "watchdog%d timeout=%u err=%d". */

extern "C" {
    pub fn watchdog_start(wdd: *mut watchdog_device, err: ::core::ffi::c_int);
    pub fn watchdog_ping(wdd: *mut watchdog_device, err: ::core::ffi::c_int);
    pub fn watchdog_stop(wdd: *mut watchdog_device, err: ::core::ffi::c_int);
    pub fn watchdog_set_timeout(
        wdd: *mut watchdog_device,
        timeout: ::core::ffi::c_uint,
        err: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
