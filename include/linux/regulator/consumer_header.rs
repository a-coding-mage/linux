/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/regulator/consumer.h. */

use core::ffi::c_void;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct regulator_dev { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
pub type suspend_state_t = i32;

pub const REGULATOR_MODE_INVALID: u32 = 0x0;
pub const REGULATOR_MODE_FAST: u32 = 0x1;
pub const REGULATOR_MODE_NORMAL: u32 = 0x2;
pub const REGULATOR_MODE_IDLE: u32 = 0x4;
pub const REGULATOR_MODE_STANDBY: u32 = 0x8;
pub const REGULATOR_ERROR_UNDER_VOLTAGE: u32 = 1 << 1;
pub const REGULATOR_ERROR_OVER_CURRENT: u32 = 1 << 2;
pub const REGULATOR_ERROR_REGULATION_OUT: u32 = 1 << 3;
pub const REGULATOR_ERROR_FAIL: u32 = 1 << 4;
pub const REGULATOR_ERROR_OVER_TEMP: u32 = 1 << 5;
pub const REGULATOR_ERROR_UNDER_VOLTAGE_WARN: u32 = 1 << 6;
pub const REGULATOR_ERROR_OVER_CURRENT_WARN: u32 = 1 << 7;
pub const REGULATOR_ERROR_OVER_VOLTAGE_WARN: u32 = 1 << 8;
pub const REGULATOR_ERROR_OVER_TEMP_WARN: u32 = 1 << 9;

#[repr(C)] pub struct pre_voltage_change_data { pub old_uV: usize, pub min_uV: usize, pub max_uV: usize }
#[repr(C)] pub struct regulator_bulk_data {
    pub supply: *const i8,
    pub consumer: *mut regulator,
    pub init_load_uA: i32,
    pub ret: i32,
}

#[cfg(feature = "CONFIG_REGULATOR")]
extern "C" {
    pub fn regulator_get(dev: *mut device, id: *const i8) -> *mut regulator;
    pub fn devm_regulator_get(dev: *mut device, id: *const i8) -> *mut regulator;
    pub fn regulator_get_exclusive(dev: *mut device, id: *const i8) -> *mut regulator;
    pub fn devm_regulator_get_exclusive(dev: *mut device, id: *const i8) -> *mut regulator;
    pub fn regulator_get_optional(dev: *mut device, id: *const i8) -> *mut regulator;
    pub fn devm_regulator_get_optional(dev: *mut device, id: *const i8) -> *mut regulator;
    pub fn devm_regulator_get_enable(dev: *mut device, id: *const i8) -> i32;
    pub fn devm_regulator_get_enable_optional(dev: *mut device, id: *const i8) -> i32;
    pub fn devm_regulator_get_enable_read_voltage(dev: *mut device, id: *const i8) -> i32;
    pub fn regulator_put(r: *mut regulator); pub fn devm_regulator_put(r: *mut regulator);
    pub fn regulator_enable(r: *mut regulator) -> i32; pub fn regulator_disable(r: *mut regulator) -> i32;
    pub fn regulator_force_disable(r: *mut regulator) -> i32; pub fn regulator_is_enabled(r: *mut regulator) -> i32;
    pub fn regulator_disable_deferred(r: *mut regulator, ms: i32) -> i32;
    pub fn regulator_bulk_get(d: *mut device, n: i32, c: *mut regulator_bulk_data) -> i32;
    pub fn devm_regulator_bulk_get(d: *mut device, n: i32, c: *mut regulator_bulk_data) -> i32;
    pub fn devm_regulator_bulk_put(c: *mut regulator_bulk_data);
    pub fn regulator_bulk_enable(n: i32, c: *mut regulator_bulk_data) -> i32;
    pub fn regulator_bulk_disable(n: i32, c: *mut regulator_bulk_data) -> i32;
    pub fn regulator_bulk_force_disable(n: i32, c: *mut regulator_bulk_data) -> i32;
    pub fn regulator_bulk_free(n: i32, c: *mut regulator_bulk_data);
    pub fn regulator_count_voltages(r: *mut regulator) -> i32;
    pub fn regulator_list_voltage(r: *mut regulator, selector: u32) -> i32;
    pub fn regulator_is_supported_voltage(r: *mut regulator, min: i32, max: i32) -> i32;
    pub fn regulator_get_linear_step(r: *mut regulator) -> u32;
    pub fn regulator_set_voltage(r: *mut regulator, min: i32, max: i32) -> i32;
    pub fn regulator_set_voltage_time(r: *mut regulator, old: i32, new: i32) -> i32;
    pub fn regulator_get_voltage(r: *mut regulator) -> i32; pub fn regulator_sync_voltage(r: *mut regulator) -> i32;
    pub fn regulator_set_current_limit(r: *mut regulator, min: i32, max: i32) -> i32;
    pub fn regulator_get_current_limit(r: *mut regulator) -> i32;
    pub fn regulator_get_unclaimed_power_budget(r: *mut regulator) -> i32;
    pub fn regulator_request_power_budget(r: *mut regulator, req: u32) -> i32;
    pub fn regulator_free_power_budget(r: *mut regulator, pw: u32);
    pub fn regulator_set_mode(r: *mut regulator, mode: u32) -> i32; pub fn regulator_get_mode(r: *mut regulator) -> u32;
    pub fn regulator_get_error_flags(r: *mut regulator, flags: *mut u32) -> i32;
    pub fn regulator_set_load(r: *mut regulator, load: i32) -> i32;
    pub fn regulator_allow_bypass(r: *mut regulator, allow: bool) -> i32;
    pub fn regulator_get_regmap(r: *mut regulator) -> *mut regmap;
    pub fn regulator_register_notifier(r: *mut regulator, n: *mut notifier_block) -> i32;
    pub fn regulator_unregister_notifier(r: *mut regulator, n: *mut notifier_block) -> i32;
    pub fn regulator_get_drvdata(r: *mut regulator) -> *mut c_void;
    pub fn regulator_set_drvdata(r: *mut regulator, data: *mut c_void);
    pub fn regulator_register_supply_alias(d: *mut device, id: *const i8, ad: *mut device, aid: *const i8) -> i32;
    pub fn regulator_unregister_supply_alias(d: *mut device, id: *const i8);
    pub fn regulator_bulk_register_supply_alias(d: *mut device, id: *const *const i8, ad: *mut device, aid: *const *const i8, n: i32) -> i32;
    pub fn regulator_bulk_unregister_supply_alias(d: *mut device, id: *const *const i8, n: i32);
    pub fn devm_regulator_register_supply_alias(d: *mut device, id: *const i8, ad: *mut device, aid: *const i8) -> i32;
    pub fn devm_regulator_bulk_register_supply_alias(d: *mut device, id: *const *const i8, ad: *mut device, aid: *const *const i8, n: i32) -> i32;
    pub fn devm_regulator_bulk_get_exclusive(d: *mut device, n: i32, c: *mut regulator_bulk_data) -> i32;
    pub fn devm_regulator_bulk_get_const(d: *mut device, n: i32, i: *const regulator_bulk_data, o: *mut *mut regulator_bulk_data) -> i32;
    pub fn devm_regulator_bulk_get_enable(d: *mut device, n: i32, id: *const *const i8) -> i32;
    pub fn regulator_set_suspend_voltage(r: *mut regulator, min: i32, max: i32, state: suspend_state_t) -> i32;
    pub fn regulator_suspend_enable(r: *mut regulator_dev, state: suspend_state_t) -> i32;
    pub fn regulator_suspend_disable(r: *mut regulator_dev, state: suspend_state_t) -> i32;
    pub fn regulator_set_hardware_vsel_register(r: *mut regulator, reg: *mut u32, mask: *mut u32) -> i32;
    pub fn regulator_list_hardware_vsel(r: *mut regulator, selector: u32) -> i32;
    pub fn regulator_hardware_enable(r: *mut regulator, enable: bool) -> i32;
    pub fn regulator_set_supply_names(c: *mut regulator_bulk_data, n: *const *const i8, count: u32);
    pub fn regulator_is_equal(a: *mut regulator, b: *mut regulator) -> bool;
}

#[inline] pub unsafe fn regulator_set_voltage_triplet(r: *mut regulator, min: i32, target: i32, max: i32) -> i32 {
    if regulator_set_voltage(r, target, max) == 0 { 0 } else { regulator_set_voltage(r, min, max) }
}
#[inline] pub unsafe fn regulator_set_voltage_tol(r: *mut regulator, new_uV: i32, tol_uV: i32) -> i32 {
    if regulator_set_voltage(r, new_uV, new_uV.wrapping_add(tol_uV)) == 0 { 0 }
    else { regulator_set_voltage(r, new_uV.wrapping_sub(tol_uV), new_uV.wrapping_add(tol_uV)) }
}
#[inline] pub unsafe fn regulator_is_supported_voltage_tol(r: *mut regulator, target: i32, tol: i32) -> i32 {
    regulator_is_supported_voltage(r, target.wrapping_sub(tol), target.wrapping_add(tol))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
