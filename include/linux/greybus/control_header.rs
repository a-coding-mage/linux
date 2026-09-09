/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Greybus CPort control protocol
 *
 * Copyright 2015 Google Inc.
 * Copyright 2015 Linaro Ltd.
 */

/* C dependencies: linux/types.h and linux/device.h. */

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct gb_interface;

#[repr(C)]
pub struct gb_connection;

#[repr(C)]
pub struct gb_control {
    pub dev: device,
    pub intf: *mut gb_interface,

    pub connection: *mut gb_connection,

    pub protocol_major: u8,
    pub protocol_minor: u8,

    pub has_bundle_activate: bool,
    pub has_bundle_version: bool,

    pub vendor_string: *mut core::ffi::c_char,
    pub product_string: *mut core::ffi::c_char,
}

/* Equivalent to container_of(d, struct gb_control, dev); dev is the first field. */
#[inline]
pub unsafe fn to_gb_control(d: *mut device) -> *mut gb_control {
    d as *mut gb_control
}

unsafe extern "C" {
    pub fn gb_control_create(intf: *mut gb_interface) -> *mut gb_control;
    pub fn gb_control_enable(control: *mut gb_control) -> core::ffi::c_int;
    pub fn gb_control_disable(control: *mut gb_control);
    pub fn gb_control_suspend(control: *mut gb_control) -> core::ffi::c_int;
    pub fn gb_control_resume(control: *mut gb_control) -> core::ffi::c_int;
    pub fn gb_control_add(control: *mut gb_control) -> core::ffi::c_int;
    pub fn gb_control_del(control: *mut gb_control);
    pub fn gb_control_get(control: *mut gb_control) -> *mut gb_control;
    pub fn gb_control_put(control: *mut gb_control);

    pub fn gb_control_get_bundle_versions(control: *mut gb_control) -> core::ffi::c_int;
    pub fn gb_control_connected_operation(
        control: *mut gb_control,
        cport_id: u16,
    ) -> core::ffi::c_int;
    pub fn gb_control_disconnected_operation(
        control: *mut gb_control,
        cport_id: u16,
    ) -> core::ffi::c_int;
    pub fn gb_control_disconnecting_operation(
        control: *mut gb_control,
        cport_id: u16,
    ) -> core::ffi::c_int;
    pub fn gb_control_mode_switch_operation(control: *mut gb_control) -> core::ffi::c_int;
    pub fn gb_control_mode_switch_prepare(control: *mut gb_control);
    pub fn gb_control_mode_switch_complete(control: *mut gb_control);
    pub fn gb_control_get_manifest_size_operation(intf: *mut gb_interface) -> core::ffi::c_int;
    pub fn gb_control_get_manifest_operation(
        intf: *mut gb_interface,
        manifest: *mut core::ffi::c_void,
        size: usize,
    ) -> core::ffi::c_int;
    pub fn gb_control_bundle_suspend(
        control: *mut gb_control,
        bundle_id: u8,
    ) -> core::ffi::c_int;
    pub fn gb_control_bundle_resume(
        control: *mut gb_control,
        bundle_id: u8,
    ) -> core::ffi::c_int;
    pub fn gb_control_bundle_deactivate(
        control: *mut gb_control,
        bundle_id: u8,
    ) -> core::ffi::c_int;
    pub fn gb_control_bundle_activate(
        control: *mut gb_control,
        bundle_id: u8,
    ) -> core::ffi::c_int;
    pub fn gb_control_interface_suspend_prepare(control: *mut gb_control) -> core::ffi::c_int;
    pub fn gb_control_interface_deactivate_prepare(control: *mut gb_control) -> core::ffi::c_int;
    pub fn gb_control_interface_hibernate_abort(control: *mut gb_control) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
