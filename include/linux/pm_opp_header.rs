/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/pm_opp.h. External kernel types/functions are supplied elsewhere. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct cpufreq_frequency_table { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct dev_pm_opp { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct opp_table { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }

pub type bool_ = bool;
pub type u32_ = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dev_pm_opp_event {
    OPP_EVENT_ADD,
    OPP_EVENT_REMOVE,
    OPP_EVENT_ENABLE,
    OPP_EVENT_DISABLE,
    OPP_EVENT_ADJUST_VOLTAGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_opp_supply {
    pub u_volt: c_ulong,
    pub u_volt_min: c_ulong,
    pub u_volt_max: c_ulong,
    pub u_amp: c_ulong,
    pub u_watt: c_ulong,
}

pub type config_regulators_t = Option<unsafe extern "C" fn(*mut device, *mut dev_pm_opp, *mut dev_pm_opp, *mut *mut regulator, c_uint) -> c_int>;
pub type config_clks_t = Option<unsafe extern "C" fn(*mut device, *mut opp_table, *mut dev_pm_opp, *mut c_void, bool) -> c_int>;

#[repr(C)]
pub struct dev_pm_opp_config {
    pub clk_names: *const *const c_char,
    pub config_clks: config_clks_t,
    pub prop_name: *const c_char,
    pub config_regulators: config_regulators_t,
    pub supported_hw: *const c_uint,
    pub supported_hw_count: c_uint,
    pub regulator_names: *const *const c_char,
    pub required_dev: *mut device,
    pub required_dev_index: c_uint,
}

pub const OPP_LEVEL_UNSET: u32 = u32::MAX;

#[repr(C)]
pub struct dev_pm_opp_data { pub turbo: bool, pub level: c_uint, pub freq: c_ulong, pub u_volt: c_ulong }
#[repr(C)]
pub struct dev_pm_opp_key { pub freq: c_ulong, pub level: c_uint, pub bw: u32 }

#[cfg(feature = "CONFIG_PM_OPP")]
extern "C" {
    pub fn dev_pm_opp_get_opp_table(dev: *mut device) -> *mut opp_table;
    pub fn dev_pm_opp_get_opp_table_ref(t: *mut opp_table) -> *mut opp_table;
    pub fn dev_pm_opp_put_opp_table(t: *mut opp_table);
    pub fn dev_pm_opp_get_bw(opp: *mut dev_pm_opp, peak: bool, index: c_int) -> c_ulong;
    pub fn dev_pm_opp_get_voltage(opp: *mut dev_pm_opp) -> c_ulong;
    pub fn dev_pm_opp_get_supplies(opp: *mut dev_pm_opp, supplies: *mut dev_pm_opp_supply) -> c_int;
    pub fn dev_pm_opp_get_power(opp: *mut dev_pm_opp) -> c_ulong;
    pub fn dev_pm_opp_get_freq_indexed(opp: *mut dev_pm_opp, index: u32) -> c_ulong;
    pub fn dev_pm_opp_get_level(opp: *mut dev_pm_opp) -> c_uint;
    pub fn dev_pm_opp_get_required_pstate(opp: *mut dev_pm_opp, index: c_uint) -> c_uint;
    pub fn dev_pm_opp_is_turbo(opp: *mut dev_pm_opp) -> bool;
    pub fn dev_pm_opp_get_opp_count(dev: *mut device) -> c_int;
    pub fn dev_pm_opp_get_max_clock_latency(dev: *mut device) -> c_ulong;
    pub fn dev_pm_opp_get_max_volt_latency(dev: *mut device) -> c_ulong;
    pub fn dev_pm_opp_get_max_transition_latency(dev: *mut device) -> c_ulong;
    pub fn dev_pm_opp_get_suspend_opp_freq(dev: *mut device) -> c_ulong;
    pub fn dev_pm_opp_find_freq_exact(dev: *mut device, freq: c_ulong, available: bool) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_find_key_exact(dev: *mut device, key: *mut dev_pm_opp_key, available: bool) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_find_freq_exact_indexed(dev: *mut device, freq: c_ulong, index: u32, available: bool) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_find_freq_floor(dev: *mut device, freq: *mut c_ulong) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_find_freq_floor_indexed(dev: *mut device, freq: *mut c_ulong, index: u32) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_find_freq_ceil(dev: *mut device, freq: *mut c_ulong) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_find_freq_ceil_indexed(dev: *mut device, freq: *mut c_ulong, index: u32) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_find_level_exact(dev: *mut device, level: c_uint) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_find_level_ceil(dev: *mut device, level: *mut c_uint) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_find_level_floor(dev: *mut device, level: *mut c_uint) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_find_bw_ceil(dev: *mut device, bw: *mut c_uint, index: c_int) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_find_bw_floor(dev: *mut device, bw: *mut c_uint, index: c_int) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_get(opp: *mut dev_pm_opp) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_put(opp: *mut dev_pm_opp);
    pub fn dev_pm_opp_add_dynamic(dev: *mut device, opp: *mut dev_pm_opp_data) -> c_int;
    pub fn dev_pm_opp_remove(dev: *mut device, freq: c_ulong);
    pub fn dev_pm_opp_remove_all_dynamic(dev: *mut device);
    pub fn dev_pm_opp_adjust_voltage(dev: *mut device, freq: c_ulong, u_volt: c_ulong, u_volt_min: c_ulong, u_volt_max: c_ulong) -> c_int;
    pub fn dev_pm_opp_enable(dev: *mut device, freq: c_ulong) -> c_int;
    pub fn dev_pm_opp_disable(dev: *mut device, freq: c_ulong) -> c_int;
    pub fn dev_pm_opp_register_notifier(dev: *mut device, nb: *mut notifier_block) -> c_int;
    pub fn dev_pm_opp_unregister_notifier(dev: *mut device, nb: *mut notifier_block) -> c_int;
    pub fn dev_pm_opp_set_config(dev: *mut device, config: *mut dev_pm_opp_config) -> c_int;
    pub fn devm_pm_opp_set_config(dev: *mut device, config: *mut dev_pm_opp_config) -> c_int;
    pub fn dev_pm_opp_clear_config(token: c_int);
    pub fn dev_pm_opp_config_clks_simple(dev: *mut device, table: *mut opp_table, opp: *mut dev_pm_opp, data: *mut c_void, scaling_down: bool) -> c_int;
    pub fn dev_pm_opp_xlate_required_opp(src: *mut opp_table, dst: *mut opp_table, opp: *mut dev_pm_opp) -> *mut dev_pm_opp;
    pub fn dev_pm_opp_xlate_performance_state(src: *mut opp_table, dst: *mut opp_table, pstate: c_uint) -> c_int;
    pub fn dev_pm_opp_set_rate(dev: *mut device, target_freq: c_ulong) -> c_int;
    pub fn dev_pm_opp_set_opp(dev: *mut device, opp: *mut dev_pm_opp) -> c_int;
    pub fn dev_pm_opp_set_sharing_cpus(dev: *mut device, mask: *const cpumask) -> c_int;
    pub fn dev_pm_opp_get_sharing_cpus(dev: *mut device, mask: *mut cpumask) -> c_int;
    pub fn dev_pm_opp_remove_table(dev: *mut device);
    pub fn dev_pm_opp_cpumask_remove_table(mask: *const cpumask);
    pub fn dev_pm_opp_sync_regulators(dev: *mut device) -> c_int;
}

/* CONFIG_PM_OPP-disabled inline definitions retain the kernel's sentinel behavior. */
#[cfg(not(feature = "CONFIG_PM_OPP"))]
pub unsafe fn dev_pm_opp_get_opp_table(_dev: *mut device) -> *mut opp_table { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_PM_OPP"))]
pub unsafe fn dev_pm_opp_get_opp_table_ref(t: *mut opp_table) -> *mut opp_table { t }
#[cfg(not(feature = "CONFIG_PM_OPP"))]
pub unsafe fn dev_pm_opp_put_opp_table(_t: *mut opp_table) {}

/* The remaining disabled-configuration helpers return the C header's zero, NULL,
 * or error sentinels; external errno/error-pointer facilities are dependency-provided. */

#[cfg(feature = "CONFIG_CPU_FREQ")]
extern "C" { pub fn dev_pm_opp_init_cpufreq_table(dev: *mut device, table: *mut *mut cpufreq_frequency_table) -> c_int; pub fn dev_pm_opp_free_cpufreq_table(dev: *mut device, table: *mut *mut cpufreq_frequency_table); }
#[cfg(not(feature = "CONFIG_CPU_FREQ"))]
pub unsafe fn dev_pm_opp_init_cpufreq_table(_dev: *mut device, _table: *mut *mut cpufreq_frequency_table) -> c_int { -22 }
#[cfg(not(feature = "CONFIG_CPU_FREQ"))]
pub unsafe fn dev_pm_opp_free_cpufreq_table(_dev: *mut device, _table: *mut *mut cpufreq_frequency_table) {}

/* CONFIG_OF declarations and fallback helpers. */
#[cfg(all(feature = "CONFIG_PM_OPP", feature = "CONFIG_OF"))]
extern "C" {
    pub fn dev_pm_opp_of_add_table(dev: *mut device) -> c_int;
    pub fn dev_pm_opp_of_add_table_indexed(dev: *mut device, index: c_int) -> c_int;
    pub fn devm_pm_opp_of_add_table_indexed(dev: *mut device, index: c_int) -> c_int;
    pub fn dev_pm_opp_of_remove_table(dev: *mut device);
    pub fn devm_pm_opp_of_add_table(dev: *mut device) -> c_int;
    pub fn dev_pm_opp_of_cpumask_add_table(mask: *const cpumask) -> c_int;
    pub fn dev_pm_opp_of_cpumask_remove_table(mask: *const cpumask);
    pub fn dev_pm_opp_of_get_sharing_cpus(dev: *mut device, mask: *mut cpumask) -> c_int;
    pub fn dev_pm_opp_of_get_opp_desc_node(dev: *mut device) -> *mut device_node;
    pub fn dev_pm_opp_get_of_node(opp: *mut dev_pm_opp) -> *mut device_node;
    pub fn of_get_required_opp_performance_state(np: *mut device_node, index: c_int) -> c_int;
    pub fn dev_pm_opp_of_has_required_opp(dev: *mut device) -> bool;
    pub fn dev_pm_opp_of_find_icc_paths(dev: *mut device, table: *mut opp_table) -> c_int;
    pub fn dev_pm_opp_of_register_em(dev: *mut device, cpus: *mut cpumask) -> c_int;
    pub fn dev_pm_opp_calc_power(dev: *mut device, uW: *mut c_ulong, kHz: *mut c_ulong) -> c_int;
}

/* Scope-cleanup DEFINE_FREE declarations are represented by the corresponding
 * unsafe cleanup operations when the surrounding kernel cleanup framework is available. */
#[inline] pub unsafe fn dev_pm_opp_add(dev: *mut device, freq: c_ulong, u_volt: c_ulong) -> c_int { let mut data = dev_pm_opp_data { turbo: false, level: 0, freq, u_volt }; dev_pm_opp_add_dynamic(dev, &mut data) }
#[inline] pub unsafe fn dev_pm_opp_get_freq(opp: *mut dev_pm_opp) -> c_ulong { dev_pm_opp_get_freq_indexed(opp, 0) }
#[inline] pub unsafe fn dev_pm_opp_put_regulators(token: c_int) { dev_pm_opp_clear_config(token) }
#[inline] pub unsafe fn dev_pm_opp_put_supported_hw(token: c_int) { dev_pm_opp_clear_config(token) }
#[inline] pub unsafe fn dev_pm_opp_put_clkname(token: c_int) { dev_pm_opp_clear_config(token) }
#[inline] pub unsafe fn dev_pm_opp_put_config_regulators(token: c_int) { dev_pm_opp_clear_config(token) }
#[inline] pub unsafe fn dev_pm_opp_put_prop_name(token: c_int) { dev_pm_opp_clear_config(token) }

#[inline]
pub unsafe fn dev_pm_opp_set_regulators(dev: *mut device, names: *const *const c_char) -> c_int {
    let mut config = dev_pm_opp_config { clk_names: core::ptr::null(), config_clks: None, prop_name: core::ptr::null(), config_regulators: None, supported_hw: core::ptr::null(), supported_hw_count: 0, regulator_names: names, required_dev: core::ptr::null_mut(), required_dev_index: 0 };
    dev_pm_opp_set_config(dev, &mut config)
}
#[inline]
pub unsafe fn devm_pm_opp_set_regulators(dev: *mut device, names: *const *const c_char) -> c_int {
    let mut config = dev_pm_opp_config { clk_names: core::ptr::null(), config_clks: None, prop_name: core::ptr::null(), config_regulators: None, supported_hw: core::ptr::null(), supported_hw_count: 0, regulator_names: names, required_dev: core::ptr::null_mut(), required_dev_index: 0 };
    devm_pm_opp_set_config(dev, &mut config)
}
#[inline]
pub unsafe fn dev_pm_opp_set_supported_hw(dev: *mut device, versions: *const u32, count: c_uint) -> c_int {
    let mut config = dev_pm_opp_config { clk_names: core::ptr::null(), config_clks: None, prop_name: core::ptr::null(), config_regulators: None, supported_hw: versions, supported_hw_count: count, regulator_names: core::ptr::null(), required_dev: core::ptr::null_mut(), required_dev_index: 0 };
    dev_pm_opp_set_config(dev, &mut config)
}
#[inline]
pub unsafe fn devm_pm_opp_set_supported_hw(dev: *mut device, versions: *const u32, count: c_uint) -> c_int {
    let mut config = dev_pm_opp_config { clk_names: core::ptr::null(), config_clks: None, prop_name: core::ptr::null(), config_regulators: None, supported_hw: versions, supported_hw_count: count, regulator_names: core::ptr::null(), required_dev: core::ptr::null_mut(), required_dev_index: 0 };
    devm_pm_opp_set_config(dev, &mut config)
}
#[inline]
pub unsafe fn dev_pm_opp_set_clkname(dev: *mut device, name: *const c_char) -> c_int {
    let names = [name, core::ptr::null()];
    let mut config = dev_pm_opp_config { clk_names: names.as_ptr(), config_clks: None, prop_name: core::ptr::null(), config_regulators: None, supported_hw: core::ptr::null(), supported_hw_count: 0, regulator_names: core::ptr::null(), required_dev: core::ptr::null_mut(), required_dev_index: 0 };
    dev_pm_opp_set_config(dev, &mut config)
}
#[inline]
pub unsafe fn devm_pm_opp_set_clkname(dev: *mut device, name: *const c_char) -> c_int {
    let names = [name, core::ptr::null()];
    let mut config = dev_pm_opp_config { clk_names: names.as_ptr(), config_clks: None, prop_name: core::ptr::null(), config_regulators: None, supported_hw: core::ptr::null(), supported_hw_count: 0, regulator_names: core::ptr::null(), required_dev: core::ptr::null_mut(), required_dev_index: 0 };
    devm_pm_opp_set_config(dev, &mut config)
}
#[inline] pub unsafe fn dev_pm_opp_set_config_regulators(dev: *mut device, helper: config_regulators_t) -> c_int { let mut c = dev_pm_opp_config { clk_names: core::ptr::null(), config_clks: None, prop_name: core::ptr::null(), config_regulators: helper, supported_hw: core::ptr::null(), supported_hw_count: 0, regulator_names: core::ptr::null(), required_dev: core::ptr::null_mut(), required_dev_index: 0 }; dev_pm_opp_set_config(dev, &mut c) }
#[inline] pub unsafe fn dev_pm_opp_set_prop_name(dev: *mut device, name: *const c_char) -> c_int { let mut c = dev_pm_opp_config { clk_names: core::ptr::null(), config_clks: None, prop_name: name, config_regulators: None, supported_hw: core::ptr::null(), supported_hw_count: 0, regulator_names: core::ptr::null(), required_dev: core::ptr::null_mut(), required_dev_index: 0 }; dev_pm_opp_set_config(dev, &mut c) }
#[inline] pub unsafe fn dev_pm_opp_set_level(dev: *mut device, level: c_uint) -> c_int { let opp = dev_pm_opp_find_level_exact(dev, level); if opp.is_null() { return -22; } dev_pm_opp_set_opp(dev, opp) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
