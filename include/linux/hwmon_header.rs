/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of hwmon.h. */

use core::ffi::c_char;

pub type UmodeT = u16;
pub type U32 = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hwmon_sensor_types {
    hwmon_chip,
    hwmon_temp,
    hwmon_in,
    hwmon_curr,
    hwmon_power,
    hwmon_energy,
    hwmon_energy64,
    hwmon_humidity,
    hwmon_fan,
    hwmon_pwm,
    hwmon_intrusion,
    hwmon_max,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hwmon_chip_attributes {
    hwmon_chip_temp_reset_history, hwmon_chip_in_reset_history,
    hwmon_chip_curr_reset_history, hwmon_chip_power_reset_history,
    hwmon_chip_register_tz, hwmon_chip_update_interval,
    hwmon_chip_update_interval_us, hwmon_chip_alarms, hwmon_chip_samples,
    hwmon_chip_curr_samples, hwmon_chip_in_samples, hwmon_chip_power_samples,
    hwmon_chip_temp_samples, hwmon_chip_beep_enable, hwmon_chip_pec,
}

macro_rules! hwmon_enum {
    ($name:ident { $($item:ident),* $(,)? }) => {
        #[repr(C)] #[derive(Copy, Clone)] pub enum $name { $($item),* }
    };
}
hwmon_enum!(hwmon_temp_attributes { hwmon_temp_enable, hwmon_temp_input, hwmon_temp_type, hwmon_temp_lcrit, hwmon_temp_lcrit_hyst, hwmon_temp_min, hwmon_temp_min_hyst, hwmon_temp_max, hwmon_temp_max_hyst, hwmon_temp_crit, hwmon_temp_crit_hyst, hwmon_temp_emergency, hwmon_temp_emergency_hyst, hwmon_temp_alarm, hwmon_temp_lcrit_alarm, hwmon_temp_min_alarm, hwmon_temp_max_alarm, hwmon_temp_crit_alarm, hwmon_temp_emergency_alarm, hwmon_temp_fault, hwmon_temp_offset, hwmon_temp_label, hwmon_temp_lowest, hwmon_temp_highest, hwmon_temp_reset_history, hwmon_temp_rated_min, hwmon_temp_rated_max, hwmon_temp_beep });
hwmon_enum!(hwmon_in_attributes { hwmon_in_enable, hwmon_in_input, hwmon_in_min, hwmon_in_max, hwmon_in_lcrit, hwmon_in_crit, hwmon_in_average, hwmon_in_lowest, hwmon_in_highest, hwmon_in_reset_history, hwmon_in_label, hwmon_in_alarm, hwmon_in_min_alarm, hwmon_in_max_alarm, hwmon_in_lcrit_alarm, hwmon_in_crit_alarm, hwmon_in_rated_min, hwmon_in_rated_max, hwmon_in_beep, hwmon_in_fault });
hwmon_enum!(hwmon_curr_attributes { hwmon_curr_enable, hwmon_curr_input, hwmon_curr_min, hwmon_curr_max, hwmon_curr_lcrit, hwmon_curr_crit, hwmon_curr_average, hwmon_curr_lowest, hwmon_curr_highest, hwmon_curr_reset_history, hwmon_curr_label, hwmon_curr_alarm, hwmon_curr_min_alarm, hwmon_curr_max_alarm, hwmon_curr_lcrit_alarm, hwmon_curr_crit_alarm, hwmon_curr_rated_min, hwmon_curr_rated_max, hwmon_curr_beep });
hwmon_enum!(hwmon_power_attributes { hwmon_power_enable, hwmon_power_average, hwmon_power_average_interval, hwmon_power_average_interval_max, hwmon_power_average_interval_min, hwmon_power_average_highest, hwmon_power_average_lowest, hwmon_power_average_max, hwmon_power_average_min, hwmon_power_input, hwmon_power_input_highest, hwmon_power_input_lowest, hwmon_power_reset_history, hwmon_power_accuracy, hwmon_power_cap, hwmon_power_cap_hyst, hwmon_power_cap_max, hwmon_power_cap_min, hwmon_power_min, hwmon_power_max, hwmon_power_crit, hwmon_power_lcrit, hwmon_power_label, hwmon_power_alarm, hwmon_power_cap_alarm, hwmon_power_min_alarm, hwmon_power_max_alarm, hwmon_power_lcrit_alarm, hwmon_power_crit_alarm, hwmon_power_rated_min, hwmon_power_rated_max });
hwmon_enum!(hwmon_energy_attributes { hwmon_energy_enable, hwmon_energy_input, hwmon_energy_label });
hwmon_enum!(hwmon_humidity_attributes { hwmon_humidity_enable, hwmon_humidity_input, hwmon_humidity_label, hwmon_humidity_min, hwmon_humidity_min_hyst, hwmon_humidity_max, hwmon_humidity_max_hyst, hwmon_humidity_alarm, hwmon_humidity_fault, hwmon_humidity_rated_min, hwmon_humidity_rated_max, hwmon_humidity_min_alarm, hwmon_humidity_max_alarm });
hwmon_enum!(hwmon_fan_attributes { hwmon_fan_enable, hwmon_fan_input, hwmon_fan_label, hwmon_fan_min, hwmon_fan_max, hwmon_fan_div, hwmon_fan_pulses, hwmon_fan_target, hwmon_fan_alarm, hwmon_fan_min_alarm, hwmon_fan_max_alarm, hwmon_fan_fault, hwmon_fan_beep });
hwmon_enum!(hwmon_pwm_attributes { hwmon_pwm_input, hwmon_pwm_enable, hwmon_pwm_mode, hwmon_pwm_freq, hwmon_pwm_auto_channels_temp });
hwmon_enum!(hwmon_intrusion_attributes { hwmon_intrusion_alarm, hwmon_intrusion_beep });

macro_rules! hwmon_bit { ($name:ident, $value:ident) => { pub const $name: u32 = 1u32 << ($value as u32); }; }

hwmon_bit!(HWMON_C_TEMP_RESET_HISTORY, hwmon_chip_attributes::hwmon_chip_temp_reset_history); hwmon_bit!(HWMON_C_IN_RESET_HISTORY, hwmon_chip_attributes::hwmon_chip_in_reset_history); hwmon_bit!(HWMON_C_CURR_RESET_HISTORY, hwmon_chip_attributes::hwmon_chip_curr_reset_history); hwmon_bit!(HWMON_C_POWER_RESET_HISTORY, hwmon_chip_attributes::hwmon_chip_power_reset_history); hwmon_bit!(HWMON_C_REGISTER_TZ, hwmon_chip_attributes::hwmon_chip_register_tz); hwmon_bit!(HWMON_C_UPDATE_INTERVAL, hwmon_chip_attributes::hwmon_chip_update_interval); hwmon_bit!(HWMON_C_UPDATE_INTERVAL_US, hwmon_chip_attributes::hwmon_chip_update_interval_us); hwmon_bit!(HWMON_C_ALARMS, hwmon_chip_attributes::hwmon_chip_alarms); hwmon_bit!(HWMON_C_SAMPLES, hwmon_chip_attributes::hwmon_chip_samples); hwmon_bit!(HWMON_C_CURR_SAMPLES, hwmon_chip_attributes::hwmon_chip_curr_samples); hwmon_bit!(HWMON_C_IN_SAMPLES, hwmon_chip_attributes::hwmon_chip_in_samples); hwmon_bit!(HWMON_C_POWER_SAMPLES, hwmon_chip_attributes::hwmon_chip_power_samples); hwmon_bit!(HWMON_C_TEMP_SAMPLES, hwmon_chip_attributes::hwmon_chip_temp_samples); hwmon_bit!(HWMON_C_BEEP_ENABLE, hwmon_chip_attributes::hwmon_chip_beep_enable); hwmon_bit!(HWMON_C_PEC, hwmon_chip_attributes::hwmon_chip_pec);

#[repr(C)] pub struct hwmon_ops { pub visible: UmodeT, pub is_visible: Option<unsafe extern "C" fn(*const core::ffi::c_void, hwmon_sensor_types, U32, i32) -> UmodeT>, pub read: Option<unsafe extern "C" fn(*mut device, hwmon_sensor_types, U32, i32, *mut i64) -> i32>, pub read_string: Option<unsafe extern "C" fn(*mut device, hwmon_sensor_types, U32, i32, *mut *const c_char) -> i32>, pub write: Option<unsafe extern "C" fn(*mut device, hwmon_sensor_types, U32, i32, i64) -> i32> }
#[repr(C)] pub struct hwmon_channel_info { pub type_: hwmon_sensor_types, pub config: *const U32 }
#[repr(C)] pub struct hwmon_chip_info { pub ops: *const hwmon_ops, pub info: *const *const hwmon_channel_info }

#[macro_export]
macro_rules! HWMON_CHANNEL_INFO { ($stype:ident, $($config:expr),* $(,)?) => {{ static CONFIG: &[u32] = &[$($config),*, 0]; static INFO: $crate::hwmon_channel_info = $crate::hwmon_channel_info { type_: $crate::hwmon_sensor_types::hwmon_$stype, config: CONFIG.as_ptr() }; &INFO }}; }

extern "C" {
    pub fn hwmon_device_register(dev: *mut device) -> *mut device;
    pub fn hwmon_device_register_with_groups(dev: *mut device, name: *const c_char, drvdata: *mut core::ffi::c_void, groups: *const *const attribute_group) -> *mut device;
    pub fn devm_hwmon_device_register_with_groups(dev: *mut device, name: *const c_char, drvdata: *mut core::ffi::c_void, groups: *const *const attribute_group) -> *mut device;
    pub fn hwmon_device_register_with_info(dev: *mut device, name: *const c_char, drvdata: *mut core::ffi::c_void, info: *const hwmon_chip_info, extra_groups: *const *const attribute_group) -> *mut device;
    pub fn hwmon_device_register_for_thermal(dev: *mut device, name: *const c_char, drvdata: *mut core::ffi::c_void) -> *mut device;
    pub fn devm_hwmon_device_register_with_info(dev: *mut device, name: *const c_char, drvdata: *mut core::ffi::c_void, info: *const hwmon_chip_info, extra_groups: *const *const attribute_group) -> *mut device;
    pub fn hwmon_device_unregister(dev: *mut device);
    pub fn hwmon_notify_event(dev: *mut device, type_: hwmon_sensor_types, attr: U32, channel: i32) -> i32;
    pub fn hwmon_sanitize_name(name: *const c_char) -> *mut c_char;
    pub fn devm_hwmon_sanitize_name(dev: *mut device, name: *const c_char) -> *mut c_char;
    pub fn hwmon_lock(dev: *mut device);
    pub fn hwmon_unlock(dev: *mut device);
}

#[inline]
pub fn hwmon_is_bad_char(ch: c_char) -> bool { matches!(ch as u8, b'-' | b'*' | b' ' | b'\t' | b'\n') }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
