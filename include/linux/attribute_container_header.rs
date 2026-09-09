/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * attribute_container.h - a generic container for all classes
 *
 * Copyright (c) 2005 - James Bottomley <James.Bottomley@steeleye.com>
 */

/* C dependencies: linux/list.h and linux/klist.h. */

use core::ffi::c_int;

pub const ATTRIBUTE_CONTAINER_NO_CLASSDEVS: c_int = 0x01;

#[repr(C)]
pub struct attribute_container {
    pub node: list_head,
    pub containers: klist,
    pub class: *mut class,
    pub grp: *const attribute_group,
    pub attrs: *mut *mut device_attribute,
    pub r#match: Option<unsafe extern "C" fn(*mut attribute_container, *mut device) -> c_int>,
    pub flags: c_ulong,
}

#[inline]
pub unsafe fn attribute_container_no_classdevs(atc: *mut attribute_container) -> c_int {
    ((*atc).flags & ATTRIBUTE_CONTAINER_NO_CLASSDEVS as c_ulong) as c_int
}

#[inline]
pub unsafe fn attribute_container_set_no_classdevs(atc: *mut attribute_container) {
    (*atc).flags |= ATTRIBUTE_CONTAINER_NO_CLASSDEVS as c_ulong;
}

unsafe extern "C" {
    pub fn attribute_container_register(cont: *mut attribute_container);
    pub fn attribute_container_unregister(cont: *mut attribute_container) -> c_int;
    pub fn attribute_container_create_device(
        dev: *mut device,
        f: Option<unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device) -> c_int>,
    );
    pub fn attribute_container_add_device(
        dev: *mut device,
        f: Option<unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device) -> c_int>,
    );
    pub fn attribute_container_remove_device(
        dev: *mut device,
        f: Option<unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device)>,
    );
    pub fn attribute_container_device_trigger(
        dev: *mut device,
        f: Option<unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device) -> c_int>,
    );
    pub fn attribute_container_device_trigger_safe(
        dev: *mut device,
        f: Option<unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device) -> c_int>,
        undo: Option<unsafe extern "C" fn(*mut attribute_container, *mut device, *mut device) -> c_int>,
    ) -> c_int;
    pub fn attribute_container_add_attrs(classdev: *mut device) -> c_int;
    pub fn attribute_container_add_class_device(classdev: *mut device) -> c_int;
    pub fn attribute_container_remove_attrs(classdev: *mut device);
    pub fn attribute_container_class_device_del(classdev: *mut device);
    pub fn attribute_container_classdev_to_container(dev: *mut device) -> *mut attribute_container;
    pub fn attribute_container_find_class_device(
        cont: *mut attribute_container,
        dev: *mut device,
    ) -> *mut device;
    pub fn attribute_container_classdev_to_attrs(
        classdev: *const device,
    ) -> *mut *mut device_attribute;
}

/* External types supplied by the included Linux headers. */
#[allow(non_camel_case_types)]
pub type c_ulong = usize;
pub struct list_head;
pub struct klist;
pub struct class;
pub struct attribute_group;
pub struct device_attribute;
pub struct device;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
