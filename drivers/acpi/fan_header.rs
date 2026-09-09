/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * ACPI fan device IDs are shared between the fan driver and the device power
 * management code.
 *
 * Add new device IDs before the generic ACPI fan one.
 */

// C dependencies supplied by other headers: device types, integer types, and
// ACPI handle definitions.

pub const ACPI_FPS_NAME_LEN: usize = 20;

// Equivalent entries for the ACPI_FAN_DEVICE_IDS macro. The surrounding
// device-id type is supplied by the consumer of this header.
pub const ACPI_FAN_DEVICE_IDS: [&str; 10] = [
    "INT3404", // Fan
    "INTC1044", // Fan for Tiger Lake generation
    "INTC1048", // Fan for Alder Lake generation
    "INTC1063", // Fan for Meteor Lake generation
    "INTC106A", // Fan for Lunar Lake generation
    "INTC10A2", // Fan for Raptor Lake generation
    "INTC10D6", // Fan for Panther Lake generation
    "INTC10FE", // Fan for Wildcat Lake generation
    "INTC10F5", // Fan for Nova Lake generation
    "PNP0C0B", // Generic ACPI fan
];

#[repr(C)]
pub struct acpi_fan_fps {
    pub control: u64,
    pub trip_point: u64,
    pub speed: u64,
    pub noise_level: u64,
    pub power: u64,
    pub name: [core::ffi::c_char; ACPI_FPS_NAME_LEN],
    pub dev_attr: device_attribute,
}

#[repr(C)]
pub struct acpi_fan_fif {
    pub revision: u8,
    pub fine_grain_ctrl: u8,
    pub step_size: u8,
    pub low_speed_notification: u8,
}

#[repr(C)]
pub struct acpi_fan_fst {
    pub revision: u64,
    pub control: u64,
    pub speed: u64,
}

#[repr(C)]
pub struct acpi_fan {
    pub handle: acpi_handle,
    pub acpi4: bool,
    pub has_fst: bool,
    pub fif: acpi_fan_fif,
    pub fps: *mut acpi_fan_fps,
    pub fps_count: core::ffi::c_int,
    // A value of 0 means that trippoint-related functions are not supported
    pub fan_trip_granularity: u32,
    // Conditional on IS_REACHABLE(CONFIG_HWMON) in the C source.
    #[cfg(feature = "CONFIG_HWMON")]
    pub hdev: *mut device,
    pub cdev: *mut thermal_cooling_device,
    pub fst_speed: device_attribute,
    pub fine_grain_control: device_attribute,
}

/**
 * Check if fan speed value is valid.
 */
#[inline]
pub fn acpi_fan_speed_valid(speed: u64) -> bool {
    speed < u32::MAX as u64
}

/**
 * Check if fan power value is valid.
 */
#[inline]
pub fn acpi_fan_power_valid(power: u64) -> bool {
    power < u32::MAX as u64
}

unsafe extern "C" {
    pub fn acpi_fan_get_fst(handle: acpi_handle, fst: *mut acpi_fan_fst) -> core::ffi::c_int;
    pub fn acpi_fan_create_attributes(device: *mut acpi_device) -> core::ffi::c_int;
    pub fn acpi_fan_delete_attributes(device: *mut acpi_device);
}

// Conditional on IS_REACHABLE(CONFIG_HWMON) in the C source.
#[cfg(feature = "CONFIG_HWMON")]
unsafe extern "C" {
    pub fn devm_acpi_fan_create_hwmon(dev: *mut device) -> core::ffi::c_int;
    pub fn acpi_fan_notify_hwmon(dev: *mut device);
}

#[cfg(not(feature = "CONFIG_HWMON"))]
#[inline]
pub unsafe fn devm_acpi_fan_create_hwmon(_dev: *mut device) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_HWMON"))]
#[inline]
pub unsafe fn acpi_fan_notify_hwmon(_dev: *mut device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
