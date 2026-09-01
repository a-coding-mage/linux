// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2024 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    pub attr: attribute,
}

#[repr(C)]
pub struct attribute_group {
    pub name: *const c_char,
    pub attrs: *mut *mut attribute,
}

#[repr(C)]
pub struct avs_fw_version {
    pub major: c_int,
    pub minor: c_int,
    pub hotfix: c_int,
    pub build: c_int,
}

#[repr(C)]
pub struct avs_fw_cfg {
    pub fw_version: avs_fw_version,
}

#[repr(C)]
pub struct avs_dev {
    pub fw_cfg: avs_fw_cfg,
}

pub type ssize_t = isize;

extern "C" {
    fn to_avs_dev(dev: *mut device) -> *mut avs_dev;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
}

unsafe extern "C" fn fw_version_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let adev: *mut avs_dev = to_avs_dev(dev);
    let fw_version: *mut avs_fw_version = &mut (*adev).fw_cfg.fw_version;

    sysfs_emit(
        buf,
        b"%d.%d.%d.%d\n\0".as_ptr() as *const c_char,
        (*fw_version).major,
        (*fw_version).minor,
        (*fw_version).hotfix,
        (*fw_version).build,
    )
}

// static DEVICE_ATTR_RO(fw_version);
extern "C" {
    static mut dev_attr_fw_version: device_attribute;
}

static mut avs_fw_attrs: [*mut attribute; 2] = unsafe {
    [
        &mut dev_attr_fw_version.attr as *mut attribute,
        ptr::null_mut(),
    ]
};

static avs_attr_group_name: [c_char; 4] = [b'a' as c_char, b'v' as c_char, b's' as c_char, 0];

static avs_attr_group: attribute_group = attribute_group {
    name: avs_attr_group_name.as_ptr(),
    attrs: unsafe { avs_fw_attrs.as_mut_ptr() },
};

#[no_mangle]
pub static avs_attr_groups: [*const attribute_group; 2] = [
    &avs_attr_group as *const attribute_group,
    ptr::null(),
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
