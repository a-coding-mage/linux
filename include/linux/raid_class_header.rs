/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * raid_class.h - a generic raid visualisation class
 *
 * Copyright (c) 2005 - James Bottomley <James.Bottomley@steeleye.com>
 */

/* Dependency supplied by linux/transport_class.h. */
#[repr(C)]
pub struct attribute_container {
    _private: [u8; 0],
}

#[repr(C)]
pub struct transport_container {
    pub ac: attribute_container,
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct raid_template {
    pub raid_attrs: transport_container,
}

#[repr(C)]
pub struct raid_function_template {
    pub cookie: *const core::ffi::c_void,
    pub is_raid: Option<unsafe extern "C" fn(*mut device) -> core::ffi::c_int>,
    pub get_resync: Option<unsafe extern "C" fn(*mut device)>,
    pub get_state: Option<unsafe extern "C" fn(*mut device)>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum raid_state {
    RAID_STATE_UNKNOWN = 0,
    RAID_STATE_ACTIVE,
    RAID_STATE_DEGRADED,
    RAID_STATE_RESYNCING,
    RAID_STATE_OFFLINE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum raid_level {
    RAID_LEVEL_UNKNOWN = 0,
    RAID_LEVEL_LINEAR,
    RAID_LEVEL_0,
    RAID_LEVEL_1,
    RAID_LEVEL_10,
    RAID_LEVEL_1E,
    RAID_LEVEL_3,
    RAID_LEVEL_4,
    RAID_LEVEL_5,
    RAID_LEVEL_50,
    RAID_LEVEL_6,
    RAID_LEVEL_JBOD,
}

#[repr(C)]
pub struct raid_data {
    pub component_list: list_head,
    pub component_count: core::ffi::c_int,
    pub level: raid_level,
    pub state: raid_state,
    pub resync: core::ffi::c_int,
}

/* resync complete goes from 0 to this */
pub const RAID_MAX_RESYNC: core::ffi::c_int = 10000;

unsafe extern "C" {
    pub fn attribute_container_find_class_device(
        ac: *mut attribute_container,
        dev: *mut device,
    ) -> *mut device;
    pub fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    pub fn raid_class_attach(
        ft: *mut raid_function_template,
    ) -> *mut raid_template;
    pub fn raid_class_release(rt: *mut raid_template);
}

#[inline]
pub unsafe fn raid_set_level(r: *mut raid_template, dev: *mut device, value: raid_level) {
    let device = attribute_container_find_class_device(&mut (*r).raid_attrs.ac, dev);
    assert!(!device.is_null());
    let rd = dev_get_drvdata(device) as *mut raid_data;
    (*rd).level = value;
}

#[inline]
pub unsafe fn raid_get_level(r: *mut raid_template, dev: *mut device) -> raid_level {
    let device = attribute_container_find_class_device(&mut (*r).raid_attrs.ac, dev);
    assert!(!device.is_null());
    let rd = dev_get_drvdata(device) as *mut raid_data;
    (*rd).level
}

#[inline]
pub unsafe fn raid_set_resync(r: *mut raid_template, dev: *mut device, value: core::ffi::c_int) {
    let device = attribute_container_find_class_device(&mut (*r).raid_attrs.ac, dev);
    assert!(!device.is_null());
    let rd = dev_get_drvdata(device) as *mut raid_data;
    (*rd).resync = value;
}

#[inline]
pub unsafe fn raid_get_resync(r: *mut raid_template, dev: *mut device) -> core::ffi::c_int {
    let device = attribute_container_find_class_device(&mut (*r).raid_attrs.ac, dev);
    assert!(!device.is_null());
    let rd = dev_get_drvdata(device) as *mut raid_data;
    (*rd).resync
}

#[inline]
pub unsafe fn raid_set_state(r: *mut raid_template, dev: *mut device, value: raid_state) {
    let device = attribute_container_find_class_device(&mut (*r).raid_attrs.ac, dev);
    assert!(!device.is_null());
    let rd = dev_get_drvdata(device) as *mut raid_data;
    (*rd).state = value;
}

#[inline]
pub unsafe fn raid_get_state(r: *mut raid_template, dev: *mut device) -> raid_state {
    let device = attribute_container_find_class_device(&mut (*r).raid_attrs.ac, dev);
    assert!(!device.is_null());
    let rd = dev_get_drvdata(device) as *mut raid_data;
    (*rd).state
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
