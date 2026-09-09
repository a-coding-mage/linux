// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Authors:
 * Alexander Aring <aar@pengutronix.de>
 *
 * Based on: net/wireless/sysfs.c
 */

// Translated from the Linux kernel implementation.  The declarations below
// are supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device {
    pub _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    pub attr: attribute,
}

#[repr(C)]
pub struct attribute {
    pub _private: [u8; 0],
}

#[repr(C)]
pub struct wpan_phy {
    pub dev: device,
}

#[repr(C)]
pub struct cfg802154_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut wpan_phy) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut wpan_phy) -> c_int>,
}

#[repr(C)]
pub struct cfg802154_registered_device {
    pub wpan_phy: wpan_phy,
    pub ops: *mut cfg802154_ops,
    pub wpan_phy_idx: c_int,
}

#[repr(C)]
pub struct attribute_group {
    pub attrs: *mut *mut attribute,
}

#[repr(C)]
pub struct class {
    pub name: *const c_char,
    pub dev_release: Option<unsafe extern "C" fn(*mut device)>,
    pub dev_groups: *const *const attribute_group,
    pub pm: *const c_void,
}

extern "C" {
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn dev_name(dev: *const device) -> *const c_char;
    fn cfg802154_dev_free(rdev: *mut cfg802154_registered_device);
    fn rtnl_lock();
    fn rtnl_unlock();
    fn rdev_suspend(rdev: *mut cfg802154_registered_device) -> c_int;
    fn rdev_resume(rdev: *mut cfg802154_registered_device) -> c_int;
    fn class_register(cls: *const class) -> c_int;
    fn class_unregister(cls: *const class);
}

unsafe fn dev_to_rdev(dev: *mut device) -> *mut cfg802154_registered_device {
    // container_of(dev, struct cfg802154_registered_device, wpan_phy.dev)
    dev as *mut cfg802154_registered_device
}

unsafe extern "C" fn index_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    sprintf(buf, b"%d\0".as_ptr() as *const c_char, (*dev_to_rdev(dev)).wpan_phy_idx)
}

pub static mut dev_attr_index: device_attribute = device_attribute {
    attr: attribute { _private: [] },
};

unsafe extern "C" fn name_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    let wpan_phy = &mut (*dev_to_rdev(dev)).wpan_phy;
    sprintf(buf, b"%s\n\0".as_ptr() as *const c_char, dev_name(&wpan_phy.dev))
}

pub static mut dev_attr_name: device_attribute = device_attribute {
    attr: attribute { _private: [] },
};

unsafe extern "C" fn wpan_phy_release(dev: *mut device) {
    let rdev = dev_to_rdev(dev);
    cfg802154_dev_free(rdev);
}

#[used]
pub static mut pmib_attrs: [*mut attribute; 3] = [
    &mut dev_attr_index.attr,
    &mut dev_attr_name.attr,
    core::ptr::null_mut(),
];

#[used]
pub static mut pmib_groups: [*const attribute_group; 2] = [
    &PMIB_GROUP,
    core::ptr::null(),
];

static PMIB_GROUP: attribute_group = attribute_group {
    attrs: unsafe { &mut pmib_attrs as *mut _ },
};

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn wpan_phy_suspend(dev: *mut device) -> c_int {
    let rdev = dev_to_rdev(dev);
    let mut ret: c_int = 0;
    if !(*rdev).ops.is_null() && (*(*rdev).ops).suspend.is_some() {
        rtnl_lock();
        ret = rdev_suspend(rdev);
        rtnl_unlock();
    }
    ret
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn wpan_phy_resume(dev: *mut device) -> c_int {
    let rdev = dev_to_rdev(dev);
    let mut ret: c_int = 0;
    if !(*rdev).ops.is_null() && (*(*rdev).ops).resume.is_some() {
        rtnl_lock();
        ret = rdev_resume(rdev);
        rtnl_unlock();
    }
    ret
}

#[cfg(CONFIG_PM_SLEEP)]
static WPAN_PHY_PM_OPS: *const c_void = core::ptr::null();
#[cfg(not(CONFIG_PM_SLEEP))]
static WPAN_PHY_PM_OPS: *const c_void = core::ptr::null();

pub static wpan_phy_class: class = class {
    name: b"ieee802154\0".as_ptr() as *const c_char,
    dev_release: Some(wpan_phy_release),
    dev_groups: unsafe { &pmib_groups as *const _ as *const *const attribute_group },
    pm: WPAN_PHY_PM_OPS,
};

pub unsafe fn wpan_phy_sysfs_init() -> c_int {
    class_register(&wpan_phy_class)
}

pub unsafe fn wpan_phy_sysfs_exit() {
    class_unregister(&wpan_phy_class);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
