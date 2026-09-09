/* SPDX-License-Identifier: GPL-2.0 */
/*
 * devfreq_cooling: Thermal cooling device implementation for devices using
 *                  devfreq
 *
 * Copyright (C) 2014-2015 ARM Limited
 */

// C dependencies: linux/devfreq.h and linux/thermal.h.

use core::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
pub struct devfreq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thermal_cooling_device {
    _private: [u8; 0],
}

pub type u32_ = u32;

/**
 * struct devfreq_cooling_power - Devfreq cooling power ops
 * @get_real_power: When this is set, the framework uses it to ask the
 * device driver for the actual power. Some devices have more sophisticated
 * methods (like power counters) to approximate the actual power that they
 * use. This function provides more accurate data to the thermal governor.
 * When the driver does not provide such function, framework just uses
 * pre-calculated table and scale the power by 'utilization' (based on
 * 'busy_time' and 'total_time' taken from devfreq 'last_status'). The value
 * returned by this function must be lower or equal than the maximum power
 * value for the current state (which can be found in power_table[state]).
 * When this interface is used, the power_table holds max total (static +
 * dynamic) power value for each OPP.
 */
#[repr(C)]
pub struct devfreq_cooling_power {
    pub get_real_power: Option<unsafe extern "C" fn(
        df: *mut devfreq,
        power: *mut u32,
        freq: c_ulong,
        voltage: c_ulong,
    ) -> c_int>,
}

#[cfg(CONFIG_DEVFREQ_THERMAL)]
extern "C" {
    pub fn of_devfreq_cooling_register_power(
        np: *mut device_node,
        df: *mut devfreq,
        dfc_power: *mut devfreq_cooling_power,
    ) -> *mut thermal_cooling_device;
    pub fn of_devfreq_cooling_register(
        np: *mut device_node,
        df: *mut devfreq,
    ) -> *mut thermal_cooling_device;
    pub fn devfreq_cooling_register(df: *mut devfreq) -> *mut thermal_cooling_device;
    pub fn devfreq_cooling_unregister(dfc: *mut thermal_cooling_device);
    pub fn devfreq_cooling_em_register(
        df: *mut devfreq,
        dfc_power: *mut devfreq_cooling_power,
    ) -> *mut thermal_cooling_device;
}

// When CONFIG_DEVFREQ_THERMAL is disabled, these inline definitions return
// ERR_PTR(-EINVAL), preserving the Linux error-pointer semantics.
#[cfg(not(CONFIG_DEVFREQ_THERMAL))]
#[inline]
pub unsafe fn of_devfreq_cooling_register_power(
    _np: *mut device_node,
    _df: *mut devfreq,
    _dfc_power: *mut devfreq_cooling_power,
) -> *mut thermal_cooling_device {
    (-22isize) as *mut thermal_cooling_device
}

#[cfg(not(CONFIG_DEVFREQ_THERMAL))]
#[inline]
pub unsafe fn of_devfreq_cooling_register(
    _np: *mut device_node,
    _df: *mut devfreq,
) -> *mut thermal_cooling_device {
    (-22isize) as *mut thermal_cooling_device
}

#[cfg(not(CONFIG_DEVFREQ_THERMAL))]
#[inline]
pub unsafe fn devfreq_cooling_register(_df: *mut devfreq) -> *mut thermal_cooling_device {
    (-22isize) as *mut thermal_cooling_device
}

#[cfg(not(CONFIG_DEVFREQ_THERMAL))]
#[inline]
pub unsafe fn devfreq_cooling_em_register(
    _df: *mut devfreq,
    _dfc_power: *mut devfreq_cooling_power,
) -> *mut thermal_cooling_device {
    (-22isize) as *mut thermal_cooling_device
}

#[cfg(not(CONFIG_DEVFREQ_THERMAL))]
#[inline]
pub unsafe fn devfreq_cooling_unregister(_dfc: *mut thermal_cooling_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
