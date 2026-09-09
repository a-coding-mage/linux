/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Generic framer header file
 *
 * Copyright 2023 CS GROUP France
 *
 * Author: Herve Codina <herve.codina@bootlin.com>
 */

use core::ffi::{c_char, c_int, c_ulong};

/* Types supplied by the Linux kernel and other dependencies. */
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct blocking_notifier_head { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum framer_iface {
    FRAMER_IFACE_E1,
    FRAMER_IFACE_T1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum framer_clock_type {
    FRAMER_CLOCK_EXT,
    FRAMER_CLOCK_INT,
}

#[repr(C)]
pub struct framer_config {
    pub iface: framer_iface,
    pub clock_type: framer_clock_type,
    pub line_clock_rate: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct framer_status {
    pub link_is_on: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum framer_event {
    FRAMER_EVENT_STATUS,
}

#[repr(C)]
pub struct framer_ops { _private: [u8; 0] }

#[repr(C)]
pub struct framer {
    pub dev: device,
    pub id: c_int,
    pub ops: *const framer_ops,
    pub mutex: mutex,
    pub init_count: c_int,
    pub power_count: c_int,
    pub pwr: *mut regulator,
    pub notify_status_work: work_struct,
    pub notifier_list: blocking_notifier_head,
    pub polling_work: delayed_work,
    pub prev_status: framer_status,
}

/* CONFIG_GENERIC_FRAMER selects the external implementations below. */
#[cfg(feature = "CONFIG_GENERIC_FRAMER")]
extern "C" {
    pub fn framer_pm_runtime_get(framer: *mut framer) -> c_int;
    pub fn framer_pm_runtime_get_sync(framer: *mut framer) -> c_int;
    pub fn framer_pm_runtime_put(framer: *mut framer);
    pub fn framer_pm_runtime_put_sync(framer: *mut framer) -> c_int;
    pub fn framer_init(framer: *mut framer) -> c_int;
    pub fn framer_exit(framer: *mut framer) -> c_int;
    pub fn framer_power_on(framer: *mut framer) -> c_int;
    pub fn framer_power_off(framer: *mut framer) -> c_int;
    pub fn framer_get_status(framer: *mut framer, status: *mut framer_status) -> c_int;
    pub fn framer_get_config(framer: *mut framer, config: *mut framer_config) -> c_int;
    pub fn framer_set_config(framer: *mut framer, config: *const framer_config) -> c_int;
    pub fn framer_notifier_register(framer: *mut framer, nb: *mut notifier_block) -> c_int;
    pub fn framer_notifier_unregister(framer: *mut framer, nb: *mut notifier_block) -> c_int;
    pub fn framer_get(dev: *mut device, con_id: *const c_char) -> *mut framer;
    pub fn framer_put(dev: *mut device, framer: *mut framer);
    pub fn devm_framer_get(dev: *mut device, con_id: *const c_char) -> *mut framer;
    pub fn devm_framer_optional_get(dev: *mut device, con_id: *const c_char) -> *mut framer;
}

/* Fallback inline definitions when CONFIG_GENERIC_FRAMER is disabled. */
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
const ENOSYS: c_int = 38;

#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_pm_runtime_get(_: *mut framer) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_pm_runtime_get_sync(_: *mut framer) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_pm_runtime_put(_: *mut framer) {}
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_pm_runtime_put_sync(_: *mut framer) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_init(_: *mut framer) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_exit(_: *mut framer) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_power_on(_: *mut framer) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_power_off(_: *mut framer) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_get_status(_: *mut framer, _: *mut framer_status) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_get_config(_: *mut framer, _: *mut framer_config) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_set_config(_: *mut framer, _: *const framer_config) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_notifier_register(_: *mut framer, _: *mut notifier_block) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_notifier_unregister(_: *mut framer, _: *mut notifier_block) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_get(_: *mut device, _: *const c_char) -> *mut framer {
    core::ptr::invalid_mut::<framer>(-ENOSYS as usize)
}
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn framer_put(_: *mut device, _: *mut framer) {}
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn devm_framer_get(_: *mut device, _: *const c_char) -> *mut framer {
    core::ptr::invalid_mut::<framer>(-ENOSYS as usize)
}
#[cfg(not(feature = "CONFIG_GENERIC_FRAMER"))]
#[inline]
pub unsafe fn devm_framer_optional_get(_: *mut device, _: *const c_char) -> *mut framer {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
