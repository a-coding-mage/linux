/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Enclosure Services
 *
 * Copyright (C) 2008 James Bottomley <James.Bottomley@HansenPartnership.com>
 *
**-----------------------------------------------------------------------------
**
**
**-----------------------------------------------------------------------------
*/

// Dependencies supplied by the surrounding kernel translation:
// linux/device.h, linux/list.h

/* A few generic types ... taken from ses-2 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enclosure_component_type {
    ENCLOSURE_COMPONENT_DEVICE = 0x01,
    ENCLOSURE_COMPONENT_CONTROLLER_ELECTRONICS = 0x07,
    ENCLOSURE_COMPONENT_SCSI_TARGET_PORT = 0x14,
    ENCLOSURE_COMPONENT_SCSI_INITIATOR_PORT = 0x15,
    ENCLOSURE_COMPONENT_ARRAY_DEVICE = 0x17,
    ENCLOSURE_COMPONENT_SAS_EXPANDER = 0x18,
}

/* ses-2 common element status */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enclosure_status {
    ENCLOSURE_STATUS_UNSUPPORTED = 0,
    ENCLOSURE_STATUS_OK,
    ENCLOSURE_STATUS_CRITICAL,
    ENCLOSURE_STATUS_NON_CRITICAL,
    ENCLOSURE_STATUS_UNRECOVERABLE,
    ENCLOSURE_STATUS_NOT_INSTALLED,
    ENCLOSURE_STATUS_UNKNOWN,
    ENCLOSURE_STATUS_UNAVAILABLE,
    /* last element for counting purposes */
    ENCLOSURE_STATUS_MAX,
}

/* SFF-8485 activity light settings */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enclosure_component_setting {
    ENCLOSURE_SETTING_DISABLED = 0,
    ENCLOSURE_SETTING_ENABLED = 1,
    ENCLOSURE_SETTING_BLINK_A_ON_OFF = 2,
    ENCLOSURE_SETTING_BLINK_A_OFF_ON = 3,
    ENCLOSURE_SETTING_BLINK_B_ON_OFF = 6,
    ENCLOSURE_SETTING_BLINK_B_OFF_ON = 7,
}

#[repr(C)]
pub struct enclosure_component_callbacks {
    pub get_status: Option<unsafe extern "C" fn(*mut enclosure_device, *mut enclosure_component)>,
    pub set_status: Option<unsafe extern "C" fn(*mut enclosure_device, *mut enclosure_component, enclosure_status) -> ::core::ffi::c_int>,
    pub get_fault: Option<unsafe extern "C" fn(*mut enclosure_device, *mut enclosure_component)>,
    pub set_fault: Option<unsafe extern "C" fn(*mut enclosure_device, *mut enclosure_component, enclosure_component_setting) -> ::core::ffi::c_int>,
    pub get_active: Option<unsafe extern "C" fn(*mut enclosure_device, *mut enclosure_component)>,
    pub set_active: Option<unsafe extern "C" fn(*mut enclosure_device, *mut enclosure_component, enclosure_component_setting) -> ::core::ffi::c_int>,
    pub get_locate: Option<unsafe extern "C" fn(*mut enclosure_device, *mut enclosure_component)>,
    pub set_locate: Option<unsafe extern "C" fn(*mut enclosure_device, *mut enclosure_component, enclosure_component_setting) -> ::core::ffi::c_int>,
    pub get_power_status: Option<unsafe extern "C" fn(*mut enclosure_device, *mut enclosure_component)>,
    pub set_power_status: Option<unsafe extern "C" fn(*mut enclosure_device, *mut enclosure_component, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub show_id: Option<unsafe extern "C" fn(*mut enclosure_device, *mut ::core::ffi::c_char) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct enclosure_component {
    pub scratch: *mut ::core::ffi::c_void,
    pub cdev: device,
    pub dev: *mut device,
    pub type_: enclosure_component_type,
    pub number: ::core::ffi::c_int,
    pub fault: ::core::ffi::c_int,
    pub active: ::core::ffi::c_int,
    pub locate: ::core::ffi::c_int,
    pub slot: ::core::ffi::c_int,
    pub status: enclosure_status,
    pub power_status: ::core::ffi::c_int,
}

#[repr(C)]
pub struct enclosure_device {
    pub scratch: *mut ::core::ffi::c_void,
    pub node: list_head,
    pub edev: device,
    pub cb: *mut enclosure_component_callbacks,
    pub components: ::core::ffi::c_int,
    pub component: [enclosure_component; 0],
}

pub unsafe fn to_enclosure_device(dev: *mut device) -> *mut enclosure_device {
    crate::container_of!(dev, enclosure_device, edev)
}

pub unsafe fn to_enclosure_component(dev: *mut device) -> *mut enclosure_component {
    crate::container_of!(dev, enclosure_component, cdev)
}

extern "C" {
    pub fn enclosure_register(
        dev: *mut device,
        name: *const ::core::ffi::c_char,
        components: ::core::ffi::c_int,
        cb: *mut enclosure_component_callbacks,
    ) -> *mut enclosure_device;
    pub fn enclosure_unregister(ed: *mut enclosure_device);
    pub fn enclosure_component_alloc(
        ed: *mut enclosure_device,
        number: ::core::ffi::c_uint,
        type_: enclosure_component_type,
        name: *const ::core::ffi::c_char,
    ) -> *mut enclosure_component;
    pub fn enclosure_component_register(component: *mut enclosure_component) -> ::core::ffi::c_int;
    pub fn enclosure_add_device(ed: *mut enclosure_device, component: ::core::ffi::c_int, dev: *mut device) -> ::core::ffi::c_int;
    pub fn enclosure_remove_device(ed: *mut enclosure_device, dev: *mut device) -> ::core::ffi::c_int;
    pub fn enclosure_find(dev: *mut device, start: *mut enclosure_device) -> *mut enclosure_device;
    pub fn enclosure_for_each_device(fn_: Option<unsafe extern "C" fn(*mut enclosure_device, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
