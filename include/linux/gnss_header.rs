/* SPDX-License-Identifier: GPL-2.0 */
/*
 * GNSS receiver support
 *
 * Copyright (C) 2018 Johan Hovold <johan@kernel.org>
 */

/* Dependencies corresponding to the C header includes are supplied externally. */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gnss_type {
    GNSS_TYPE_NMEA = 0,
    GNSS_TYPE_SIRF,
    GNSS_TYPE_UBX,
    GNSS_TYPE_MTK,

    GNSS_TYPE_COUNT,
}

#[repr(C)]
pub struct gnss_operations {
    pub open: Option<unsafe extern "C" fn(gdev: *mut gnss_device) -> ::core::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(gdev: *mut gnss_device)>,
    pub write_raw: Option<unsafe extern "C" fn(
        gdev: *mut gnss_device,
        buf: *const ::core::ffi::c_uchar,
        count: usize,
    ) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct gnss_device {
    pub dev: device,
    pub cdev: cdev,
    pub id: ::core::ffi::c_int,

    pub r#type: gnss_type,
    pub flags: ::core::ffi::c_ulong,

    pub rwsem: rw_semaphore,
    pub ops: *const gnss_operations,
    pub count: ::core::ffi::c_uint,
    pub disconnected: ::core::ffi::c_uint,

    pub read_mutex: mutex,
    pub read_fifo: kfifo,
    pub read_queue: wait_queue_head_t,

    pub write_mutex: mutex,
    pub write_buf: *mut ::core::ffi::c_char,
}

extern "C" {
    pub fn gnss_allocate_device(parent: *mut device) -> *mut gnss_device;
    pub fn gnss_put_device(gdev: *mut gnss_device);
    pub fn gnss_register_device(gdev: *mut gnss_device) -> ::core::ffi::c_int;
    pub fn gnss_deregister_device(gdev: *mut gnss_device);

    pub fn gnss_insert_raw(
        gdev: *mut gnss_device,
        buf: *const ::core::ffi::c_uchar,
        count: usize,
    ) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn gnss_set_drvdata(gdev: *mut gnss_device, data: *mut ::core::ffi::c_void) {
    dev_set_drvdata(&mut (*gdev).dev, data);
}

#[inline]
pub unsafe fn gnss_get_drvdata(gdev: *mut gnss_device) -> *mut ::core::ffi::c_void {
    dev_get_drvdata(&(*gdev).dev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
