/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Copyright (c) 2010-2012 Broadcom. All rights reserved. */

use core::ffi::c_void;
use core::os::raw::{c_char, c_int, c_short, c_uint};

/* The C macro is preserved as a Rust macro for source-level compatibility. */
macro_rules! VCHIQ_MAKE_FOURCC {
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr) => {
        (($x0 << 24) | ($x1 << 16) | ($x2 << 8) | $x3)
    };
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vchiq_reason {
    VCHIQ_SERVICE_OPENED,
    VCHIQ_SERVICE_CLOSED,
    VCHIQ_MESSAGE_AVAILABLE,
    VCHIQ_BULK_TRANSMIT_DONE,
    VCHIQ_BULK_RECEIVE_DONE,
    VCHIQ_BULK_TRANSMIT_ABORTED,
    VCHIQ_BULK_RECEIVE_ABORTED,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vchiq_bulk_mode {
    VCHIQ_BULK_MODE_CALLBACK,
    VCHIQ_BULK_MODE_BLOCKING,
    VCHIQ_BULK_MODE_NOCALLBACK,
    VCHIQ_BULK_MODE_WAITING,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vchiq_service_option {
    VCHIQ_SERVICE_OPTION_AUTOCLOSE,
    VCHIQ_SERVICE_OPTION_SLOT_QUOTA,
    VCHIQ_SERVICE_OPTION_MESSAGE_QUOTA,
    VCHIQ_SERVICE_OPTION_SYNCHRONOUS,
    VCHIQ_SERVICE_OPTION_TRACE,
}

#[repr(C)]
pub struct vchiq_header {
    /* The message identifier - opaque to applications. */
    pub msgid: c_int,
    /* Size of message data. */
    pub size: c_uint,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct vchiq_element {
    pub data: *const c_void,
    pub size: c_uint,
}

#[repr(C)]
pub struct vchiq_instance {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vchiq_state {
    _private: [u8; 0],
}

pub type vchiq_callback = unsafe extern "C" fn(
    instance: *mut vchiq_instance,
    reason: vchiq_reason,
    header: *mut vchiq_header,
    handle: c_uint,
    cb_data: *mut c_void,
    cb_userdata: *mut c_void,
) -> c_int;

#[repr(C)]
pub struct vchiq_service_base {
    pub fourcc: c_int,
    pub callback: Option<vchiq_callback>,
    pub userdata: *mut c_void,
}

#[repr(C)]
pub struct vchiq_completion_data_kernel {
    pub reason: vchiq_reason,
    pub header: *mut vchiq_header,
    pub service_userdata: *mut c_void,
    pub cb_data: *mut c_void,
    pub cb_userdata: *mut c_void,
}

#[repr(C)]
pub struct vchiq_service_params_kernel {
    pub fourcc: c_int,
    pub callback: Option<vchiq_callback>,
    pub userdata: *mut c_void,
    pub version: c_short,
    pub version_min: c_short,
}

extern "C" {
    pub fn vchiq_initialise(
        state: *mut vchiq_state,
        pinstance: *mut *mut vchiq_instance,
    ) -> c_int;
    pub fn vchiq_shutdown(instance: *mut vchiq_instance) -> c_int;
    pub fn vchiq_connect(instance: *mut vchiq_instance) -> c_int;
    pub fn vchiq_open_service(
        instance: *mut vchiq_instance,
        params: *const vchiq_service_params_kernel,
        pservice: *mut c_uint,
    ) -> c_int;
    pub fn vchiq_close_service(instance: *mut vchiq_instance, service: c_uint) -> c_int;
    pub fn vchiq_use_service(instance: *mut vchiq_instance, service: c_uint) -> c_int;
    pub fn vchiq_release_service(instance: *mut vchiq_instance, service: c_uint) -> c_int;
    pub fn vchiq_msg_queue_push(
        instance: *mut vchiq_instance,
        handle: c_uint,
        header: *mut vchiq_header,
    );
    pub fn vchiq_release_message(
        instance: *mut vchiq_instance,
        service: c_uint,
        header: *mut vchiq_header,
    );
    pub fn vchiq_queue_kernel_message(
        instance: *mut vchiq_instance,
        handle: c_uint,
        data: *mut c_void,
        size: c_uint,
    ) -> c_int;
    pub fn vchiq_bulk_transmit(
        instance: *mut vchiq_instance,
        service: c_uint,
        data: *const c_void,
        size: c_uint,
        userdata: *mut c_void,
        mode: vchiq_bulk_mode,
    ) -> c_int;
    pub fn vchiq_bulk_receive(
        instance: *mut vchiq_instance,
        service: c_uint,
        data: *mut c_void,
        size: c_uint,
        userdata: *mut c_void,
        mode: vchiq_bulk_mode,
    ) -> c_int;
    pub fn vchiq_get_service_userdata(
        instance: *mut vchiq_instance,
        service: c_uint,
    ) -> *mut c_void;
    pub fn vchiq_get_peer_version(
        instance: *mut vchiq_instance,
        handle: c_uint,
        peer_version: *mut c_short,
    ) -> c_int;
    pub fn vchiq_msg_hold(
        instance: *mut vchiq_instance,
        handle: c_uint,
    ) -> *mut vchiq_header;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
