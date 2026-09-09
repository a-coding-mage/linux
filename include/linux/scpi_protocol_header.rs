/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SCPI Message Protocol driver header
 *
 * Copyright (C) 2014 ARM Ltd.
 */

// C dependency: <linux/types.h>

#[repr(C, packed)]
pub struct scpi_opp {
    pub freq: u32,
    pub m_volt: u32,
}

#[repr(C)]
pub struct scpi_dvfs_info {
    pub count: u32,
    pub latency: u32, /* in nanoseconds */
    pub opps: *mut scpi_opp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum scpi_sensor_class {
    TEMPERATURE,
    VOLTAGE,
    CURRENT,
    POWER,
    ENERGY,
}

#[repr(C, packed)]
pub struct scpi_sensor_info {
    pub sensor_id: u16,
    pub class: u8,
    pub trigger_type: u8,
    pub name: [core::ffi::c_char; 20],
}

// C dependency: struct device.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/**
 * struct scpi_ops - represents the various operations provided
 *	by SCP through SCPI message protocol
 * @get_version: returns the major and minor revision on the SCPI
 *	message protocol
 * @clk_get_range: gets clock range limit(min - max in Hz)
 * @clk_get_val: gets clock value(in Hz)
 * @clk_set_val: sets the clock value, setting to 0 will disable the
 *	clock (if supported)
 * @dvfs_get_idx: gets the Operating Point of the given power domain.
 *	OPP is an index to the list return by @dvfs_get_info
 * @dvfs_set_idx: sets the Operating Point of the given power domain.
 *	OPP is an index to the list return by @dvfs_get_info
 * @dvfs_get_info: returns the DVFS capabilities of the given power
 *	domain. It includes the OPP list and the latency information
 * @device_domain_id: gets the scpi domain id for a given device
 * @get_transition_latency: gets the DVFS transition latency for a given device
 * @add_opps_to_device: adds all the OPPs for a given device
 * @sensor_get_capability: get the list of capabilities for the sensors
 * @sensor_get_info: get the information of the specified sensor
 * @sensor_get_value: gets the current value of the sensor
 * @device_get_power_state: gets the power state of a power domain
 * @device_set_power_state: sets the power state of a power domain
 */
#[repr(C)]
pub struct scpi_ops {
    pub get_version: Option<unsafe extern "C" fn() -> u32>,
    pub clk_get_range: Option<unsafe extern "C" fn(u16, *mut core::ffi::c_ulong, *mut core::ffi::c_ulong) -> i32>,
    pub clk_get_val: Option<unsafe extern "C" fn(u16) -> core::ffi::c_ulong>,
    pub clk_set_val: Option<unsafe extern "C" fn(u16, core::ffi::c_ulong) -> i32>,
    pub dvfs_get_idx: Option<unsafe extern "C" fn(u8) -> i32>,
    pub dvfs_set_idx: Option<unsafe extern "C" fn(u8, u8) -> i32>,
    pub dvfs_get_info: Option<unsafe extern "C" fn(u8) -> *mut scpi_dvfs_info>,
    pub device_domain_id: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub get_transition_latency: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub add_opps_to_device: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub sensor_get_capability: Option<unsafe extern "C" fn(*mut u16) -> i32>,
    pub sensor_get_info: Option<unsafe extern "C" fn(u16, *mut scpi_sensor_info) -> i32>,
    pub sensor_get_value: Option<unsafe extern "C" fn(u16, *mut u64) -> i32>,
    pub device_get_power_state: Option<unsafe extern "C" fn(u16) -> i32>,
    pub device_set_power_state: Option<unsafe extern "C" fn(u16, u8) -> i32>,
}

// Build-time condition preserved from IS_REACHABLE(CONFIG_ARM_SCPI_PROTOCOL).
#[cfg(feature = "CONFIG_ARM_SCPI_PROTOCOL")]
extern "C" {
    pub fn get_scpi_ops() -> *mut scpi_ops;
}

#[cfg(not(feature = "CONFIG_ARM_SCPI_PROTOCOL"))]
#[inline]
pub fn get_scpi_ops() -> *mut scpi_ops {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
