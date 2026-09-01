// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2020 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

// C includes translated as external dependency intent:
// #include <linux/pm_runtime.h>
// #include "core.h"

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
pub struct attribute_group {
    pub attrs: *mut *mut attribute,
}

#[repr(C)]
pub struct catpt_dev {
    pub dev: *mut device,
    pub ipc: catpt_ipc,
}

#[repr(C)]
pub struct catpt_ipc {
    pub config: catpt_ipc_config,
}

#[repr(C)]
pub struct catpt_ipc_config {
    pub fw_info: *const c_char,
}

#[repr(C)]
pub struct catpt_fw_version {
    pub type_: c_int,
    pub major: c_int,
    pub minor: c_int,
    pub build: c_int,
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn catpt_ipc_get_fw_version(cdev: *mut catpt_dev, version: *mut catpt_fw_version) -> c_int;
    fn CATPT_IPC_RET(ret: c_int) -> ssize_t;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
}

unsafe extern "C" fn fw_version_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let cdev: *mut catpt_dev = dev_get_drvdata(dev) as *mut catpt_dev;
    let mut version: catpt_fw_version = core::mem::zeroed();
    let mut ret: c_int;

    ret = pm_runtime_resume_and_get((*cdev).dev);
    if ret != 0 {
        return ret as ssize_t;
    }

    ret = catpt_ipc_get_fw_version(cdev, &mut version);

    pm_runtime_put_autosuspend((*cdev).dev);

    if ret != 0 {
        return CATPT_IPC_RET(ret);
    }

    sysfs_emit(
        buf,
        b"%d.%d.%d.%d\n\0".as_ptr() as *const c_char,
        version.type_,
        version.major,
        version.minor,
        version.build,
    )
}

// static DEVICE_ATTR_RO(fw_version);
extern "C" {
    static mut dev_attr_fw_version: device_attribute;
}

unsafe extern "C" fn fw_info_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let cdev: *mut catpt_dev = dev_get_drvdata(dev) as *mut catpt_dev;

    sysfs_emit(
        buf,
        b"%s\n\0".as_ptr() as *const c_char,
        (*cdev).ipc.config.fw_info,
    )
}

// static DEVICE_ATTR_RO(fw_info);
extern "C" {
    static mut dev_attr_fw_info: device_attribute;
}

static mut catpt_attrs: [*mut attribute; 3] = [
    unsafe { &mut dev_attr_fw_version.attr as *mut attribute },
    unsafe { &mut dev_attr_fw_info.attr as *mut attribute },
    core::ptr::null_mut(),
];

static catpt_attr_group: attribute_group = attribute_group {
    attrs: unsafe { catpt_attrs.as_mut_ptr() },
};

pub static mut catpt_attr_groups: [*const attribute_group; 2] = [
    &catpt_attr_group as *const attribute_group,
    core::ptr::null(),
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
