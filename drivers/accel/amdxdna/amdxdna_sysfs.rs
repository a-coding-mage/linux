// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/driver translation unit.
use core::ffi::{c_char, c_int, c_void};

type ssize_t = isize;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    pub attr: attribute,
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    pub attrs: *mut *mut attribute,
}

#[repr(C)]
pub struct drm_device {
    pub dev: *mut device,
}

#[repr(C)]
pub struct amdxdna_dev_info {
    pub device_type: c_int,
}

#[repr(C)]
pub struct amdxdna_fw_ver {
    pub major: c_int,
    pub minor: c_int,
    pub sub: c_int,
    pub build: c_int,
}

#[repr(C)]
pub struct amdxdna_dev {
    pub ddev: drm_device,
    pub vbnv: *const c_char,
    pub dev_info: *mut amdxdna_dev_info,
    pub fw_ver: amdxdna_fw_ver,
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
    fn sysfs_create_group(kobj: *mut kobject, group: *const attribute_group) -> c_int;
    fn sysfs_remove_group(kobj: *mut kobject, group: *const attribute_group);
    fn XDNA_ERR(xdna: *mut amdxdna_dev, format: *const c_char, ...);
}

// DEVICE_ATTR_RO(vbnv)
static mut dev_attr_vbnv: device_attribute = device_attribute {
    attr: attribute { _private: [] },
};

// DEVICE_ATTR_RO(device_type)
static mut dev_attr_device_type: device_attribute = device_attribute {
    attr: attribute { _private: [] },
};

// DEVICE_ATTR_RO(fw_version)
static mut dev_attr_fw_version: device_attribute = device_attribute {
    attr: attribute { _private: [] },
};

unsafe extern "C" fn vbnv_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let xdna = dev_get_drvdata(dev) as *mut amdxdna_dev;

    if (*xdna).vbnv.is_null() {
        return sprintf(buf, b"\n\0".as_ptr() as *const c_char) as ssize_t;
    }

    sprintf(
        buf,
        b"%s\n\0".as_ptr() as *const c_char,
        (*xdna).vbnv,
    ) as ssize_t
}

unsafe extern "C" fn device_type_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let xdna = dev_get_drvdata(dev) as *mut amdxdna_dev;

    sprintf(
        buf,
        b"%d\n\0".as_ptr() as *const c_char,
        (*(*xdna).dev_info).device_type,
    ) as ssize_t
}

unsafe extern "C" fn fw_version_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let xdna = dev_get_drvdata(dev) as *mut amdxdna_dev;

    sprintf(
        buf,
        b"%d.%d.%d.%d\n\0".as_ptr() as *const c_char,
        (*xdna).fw_ver.major,
        (*xdna).fw_ver.minor,
        (*xdna).fw_ver.sub,
        (*xdna).fw_ver.build,
    ) as ssize_t
}

static mut amdxdna_attrs: [*mut attribute; 4] = [
    unsafe { &raw mut dev_attr_device_type.attr },
    unsafe { &raw mut dev_attr_vbnv.attr },
    unsafe { &raw mut dev_attr_fw_version.attr },
    core::ptr::null_mut(),
];

static mut amdxdna_attr_group: attribute_group = attribute_group {
    attrs: unsafe { &raw mut amdxdna_attrs[0] },
};

pub unsafe extern "C" fn amdxdna_sysfs_init(xdna: *mut amdxdna_dev) -> c_int {
    let ret = sysfs_create_group(
        // The kernel's embedded device exposes its kobject through this path.
        (*(*xdna).ddev.dev as *mut device) as *mut kobject,
        &raw const amdxdna_attr_group,
    );
    if ret != 0 {
        XDNA_ERR(xdna, b"Create attr group failed\0".as_ptr() as *const c_char);
    }

    ret
}

pub unsafe extern "C" fn amdxdna_sysfs_fini(xdna: *mut amdxdna_dev) {
    sysfs_remove_group(
        (*(*xdna).ddev.dev as *mut device) as *mut kobject,
        &raw const amdxdna_attr_group,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
