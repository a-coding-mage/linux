/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * devfreq-event: a framework to provide raw data and events of devfreq devices
 *
 * Copyright (C) 2014 Samsung Electronics
 * Author: Chanwoo Choi <cw00.choi@samsung.com>
 */

/* Dependency supplied by the Linux device headers. */

#[repr(C)]
pub struct devfreq_event_dev {
    pub node: list_head,
    pub dev: device,
    pub lock: mutex,
    pub enable_count: u32,
    pub desc: *const devfreq_event_desc,
}

#[repr(C)]
pub struct devfreq_event_data {
    pub load_count: ::core::ffi::c_ulong,
    pub total_count: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct devfreq_event_ops {
    pub enable: Option<unsafe extern "C" fn(edev: *mut devfreq_event_dev) -> i32>,
    pub disable: Option<unsafe extern "C" fn(edev: *mut devfreq_event_dev) -> i32>,
    pub reset: Option<unsafe extern "C" fn(edev: *mut devfreq_event_dev) -> i32>,
    pub set_event: Option<unsafe extern "C" fn(edev: *mut devfreq_event_dev) -> i32>,
    pub get_event: Option<
        unsafe extern "C" fn(
            edev: *mut devfreq_event_dev,
            edata: *mut devfreq_event_data,
        ) -> i32,
    >,
}

#[repr(C)]
pub struct devfreq_event_desc {
    pub name: *const ::core::ffi::c_char,
    pub event_type: u32,
    pub driver_data: *mut ::core::ffi::c_void,
    pub ops: *const devfreq_event_ops,
}

/* When CONFIG_PM_DEVFREQ_EVENT is enabled, these are external declarations. */
#[cfg(CONFIG_PM_DEVFREQ_EVENT)]
extern "C" {
    pub fn devfreq_event_enable_edev(edev: *mut devfreq_event_dev) -> i32;
    pub fn devfreq_event_disable_edev(edev: *mut devfreq_event_dev) -> i32;
    pub fn devfreq_event_is_enabled(edev: *mut devfreq_event_dev) -> bool;
    pub fn devfreq_event_set_event(edev: *mut devfreq_event_dev) -> i32;
    pub fn devfreq_event_get_event(
        edev: *mut devfreq_event_dev,
        edata: *mut devfreq_event_data,
    ) -> i32;
    pub fn devfreq_event_reset_event(edev: *mut devfreq_event_dev) -> i32;
    pub fn devfreq_event_get_edev_by_phandle(
        dev: *mut device,
        phandle_name: *const ::core::ffi::c_char,
        index: i32,
    ) -> *mut devfreq_event_dev;
    pub fn devfreq_event_get_edev_count(
        dev: *mut device,
        phandle_name: *const ::core::ffi::c_char,
    ) -> i32;
    pub fn devfreq_event_add_edev(
        dev: *mut device,
        desc: *mut devfreq_event_desc,
    ) -> *mut devfreq_event_dev;
    pub fn devfreq_event_remove_edev(edev: *mut devfreq_event_dev) -> i32;
    pub fn devm_devfreq_event_add_edev(
        dev: *mut device,
        desc: *mut devfreq_event_desc,
    ) -> *mut devfreq_event_dev;
    pub fn devm_devfreq_event_remove_edev(dev: *mut device, edev: *mut devfreq_event_dev);
}

#[inline]
pub unsafe fn devfreq_event_get_drvdata(edev: *mut devfreq_event_dev) -> *mut ::core::ffi::c_void {
    (*(*edev).desc).driver_data
}

/* When CONFIG_PM_DEVFREQ_EVENT is disabled, the kernel inline stubs apply. */
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devfreq_event_enable_edev(_edev: *mut devfreq_event_dev) -> i32 { -22 }
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devfreq_event_disable_edev(_edev: *mut devfreq_event_dev) -> i32 { -22 }
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devfreq_event_is_enabled(_edev: *mut devfreq_event_dev) -> bool { false }
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devfreq_event_set_event(_edev: *mut devfreq_event_dev) -> i32 { -22 }
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devfreq_event_get_event(_edev: *mut devfreq_event_dev, _edata: *mut devfreq_event_data) -> i32 { -22 }
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devfreq_event_reset_event(_edev: *mut devfreq_event_dev) -> i32 { -22 }
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devfreq_event_get_edev_by_phandle(_dev: *mut device, _phandle_name: *const ::core::ffi::c_char, _index: i32) -> *mut devfreq_event_dev { ERR_PTR(-EINVAL) }
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devfreq_event_get_edev_count(_dev: *mut device, _phandle_name: *const ::core::ffi::c_char) -> i32 { -22 }
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devfreq_event_add_edev(_dev: *mut device, _desc: *mut devfreq_event_desc) -> *mut devfreq_event_dev { ERR_PTR(-EINVAL) }
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devfreq_event_remove_edev(_edev: *mut devfreq_event_dev) -> i32 { -22 }
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devm_devfreq_event_add_edev(_dev: *mut device, _desc: *mut devfreq_event_desc) -> *mut devfreq_event_dev { ERR_PTR(-EINVAL) }
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devm_devfreq_event_remove_edev(_dev: *mut device, _edev: *mut devfreq_event_dev) {}
#[cfg(not(CONFIG_PM_DEVFREQ_EVENT))]
#[inline]
pub unsafe fn devfreq_event_get_drvdata(_edev: *mut devfreq_event_dev) -> *mut ::core::ffi::c_void { ::core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
