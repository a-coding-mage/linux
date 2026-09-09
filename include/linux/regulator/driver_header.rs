/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/regulator/driver.h. */

/* External types supplied by the surrounding kernel translation. */
pub enum device {}
pub enum device_node {}
pub enum linear_range {}
pub enum module {}
pub enum gpio_desc {}
pub enum regmap {}
pub enum regulator_dev {}
pub enum regulator_init_data {}
pub enum regulator_enable_gpio {}
pub enum regulator_coupler {}
pub enum list_head {}
pub enum blocking_notifier_head {}
pub enum ww_mutex {}
pub enum task_struct {}
pub enum regulation_constraints {}
pub enum regulator {}
pub enum delayed_work {}
pub enum dentry {}
pub enum notifier_block {}
pub enum spinlock_t {}
pub enum ktime_t {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum regulator_status {
    REGULATOR_STATUS_OFF,
    REGULATOR_STATUS_ON,
    REGULATOR_STATUS_ERROR,
    REGULATOR_STATUS_FAST,
    REGULATOR_STATUS_NORMAL,
    REGULATOR_STATUS_IDLE,
    REGULATOR_STATUS_STANDBY,
    REGULATOR_STATUS_BYPASS,
    REGULATOR_STATUS_UNDEFINED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum regulator_detection_severity {
    REGULATOR_SEVERITY_PROT,
    REGULATOR_SEVERITY_ERR,
    REGULATOR_SEVERITY_WARN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum regulator_type { REGULATOR_VOLTAGE, REGULATOR_CURRENT }

#[repr(C)]
pub struct regulator_ops {
    pub list_voltage: Option<unsafe extern "C" fn(*mut regulator_dev, u32) -> i32>,
    pub set_voltage: Option<unsafe extern "C" fn(*mut regulator_dev, i32, i32, *mut u32) -> i32>,
    pub map_voltage: Option<unsafe extern "C" fn(*mut regulator_dev, i32, i32) -> i32>,
    pub set_voltage_sel: Option<unsafe extern "C" fn(*mut regulator_dev, u32) -> i32>,
    pub get_voltage: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub get_voltage_sel: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub set_current_limit: Option<unsafe extern "C" fn(*mut regulator_dev, i32, i32) -> i32>,
    pub get_current_limit: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub set_input_current_limit: Option<unsafe extern "C" fn(*mut regulator_dev, i32) -> i32>,
    pub set_over_current_protection: Option<unsafe extern "C" fn(*mut regulator_dev, i32, i32, bool) -> i32>,
    pub set_over_voltage_protection: Option<unsafe extern "C" fn(*mut regulator_dev, i32, i32, bool) -> i32>,
    pub set_under_voltage_protection: Option<unsafe extern "C" fn(*mut regulator_dev, i32, i32, bool) -> i32>,
    pub set_thermal_protection: Option<unsafe extern "C" fn(*mut regulator_dev, i32, i32, bool) -> i32>,
    pub set_active_discharge: Option<unsafe extern "C" fn(*mut regulator_dev, bool) -> i32>,
    pub enable: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub set_mode: Option<unsafe extern "C" fn(*mut regulator_dev, u32) -> i32>,
    pub get_mode: Option<unsafe extern "C" fn(*mut regulator_dev) -> u32>,
    pub get_error_flags: Option<unsafe extern "C" fn(*mut regulator_dev, *mut u32) -> i32>,
    pub enable_time: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub set_ramp_delay: Option<unsafe extern "C" fn(*mut regulator_dev, i32) -> i32>,
    pub set_voltage_time: Option<unsafe extern "C" fn(*mut regulator_dev, i32, i32) -> i32>,
    pub set_voltage_time_sel: Option<unsafe extern "C" fn(*mut regulator_dev, u32, u32) -> i32>,
    pub set_soft_start: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub get_status: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub get_optimum_mode: Option<unsafe extern "C" fn(*mut regulator_dev, i32, i32, i32) -> u32>,
    pub set_load: Option<unsafe extern "C" fn(*mut regulator_dev, i32) -> i32>,
    pub set_bypass: Option<unsafe extern "C" fn(*mut regulator_dev, bool) -> i32>,
    pub get_bypass: Option<unsafe extern "C" fn(*mut regulator_dev, *mut bool) -> i32>,
    pub set_suspend_voltage: Option<unsafe extern "C" fn(*mut regulator_dev, i32) -> i32>,
    pub set_suspend_enable: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub set_suspend_disable: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub set_suspend_mode: Option<unsafe extern "C" fn(*mut regulator_dev, u32) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
    pub set_pull_down: Option<unsafe extern "C" fn(*mut regulator_dev) -> i32>,
}

#[repr(C)]
pub struct regulator_desc {
    pub name: *const i8, pub supply_name: *const i8, pub of_match: *const i8,
    pub of_match_full_name: bool, pub regulators_node: *const i8,
    pub of_parse_cb: Option<unsafe extern "C" fn(*mut device_node, *const regulator_desc, *mut regulator_config) -> i32>,
    pub init_cb: Option<unsafe extern "C" fn(*mut regulator_dev, *mut regulator_config) -> i32>,
    pub id: i32, pub continuous_voltage_range: u32, pub n_voltages: u32,
    pub n_current_limits: u32, pub ops: *const regulator_ops, pub irq: i32,
    pub type_: regulator_type, pub owner: *mut module, pub min_uV: u32,
    pub uV_step: u32, pub linear_min_sel: u32, pub fixed_uV: i32,
    pub ramp_delay: u32, pub min_dropout_uV: i32, pub linear_ranges: *const linear_range,
    pub linear_range_selectors_bitfield: *const u32, pub n_linear_ranges: i32,
    pub volt_table: *const u32, pub curr_table: *const u32, pub vsel_range_reg: u32,
    pub vsel_range_mask: u32, pub range_applied_by_vsel: bool, pub vsel_reg: u32,
    pub vsel_mask: u32, pub vsel_step: u32, pub csel_reg: u32, pub csel_mask: u32,
    pub apply_reg: u32, pub apply_bit: u32, pub enable_reg: u32, pub enable_mask: u32,
    pub enable_val: u32, pub disable_val: u32, pub enable_is_inverted: bool,
    pub bypass_reg: u32, pub bypass_mask: u32, pub bypass_val_on: u32, pub bypass_val_off: u32,
    pub active_discharge_on: u32, pub active_discharge_off: u32, pub active_discharge_mask: u32,
    pub active_discharge_reg: u32, pub soft_start_reg: u32, pub soft_start_mask: u32,
    pub soft_start_val_on: u32, pub pull_down_reg: u32, pub pull_down_mask: u32,
    pub pull_down_val_on: u32, pub ramp_reg: u32, pub ramp_mask: u32,
    pub ramp_delay_table: *const u32, pub n_ramp_values: u32, pub enable_time: u32,
    pub off_on_delay: u32, pub poll_enabled_time: u32,
    pub of_map_mode: Option<unsafe extern "C" fn(u32) -> u32>,
}

#[repr(C)]
pub struct regulator_config {
    pub dev: *mut device, pub init_data: *const regulator_init_data, pub driver_data: *mut core::ffi::c_void,
    pub of_node: *mut device_node, pub regmap: *mut regmap, pub ena_gpiod: *mut gpio_desc,
}

#[repr(C)]
pub struct regulator_err_state { pub rdev: *mut regulator_dev, pub notifs: usize, pub errors: usize, pub possible_errs: i32 }
#[repr(C)]
pub struct regulator_irq_data { pub states: *mut regulator_err_state, pub num_states: i32, pub data: *mut core::ffi::c_void, pub opaque: isize }
#[repr(C)]
pub struct regulator_irq_desc {
    pub name: *const i8, pub fatal_cnt: i32, pub reread_ms: i32, pub irq_off_ms: i32,
    pub skip_off: bool, pub high_prio: bool, pub data: *mut core::ffi::c_void,
    pub die: Option<unsafe extern "C" fn(*mut regulator_irq_data) -> i32>,
    pub map_event: Option<unsafe extern "C" fn(i32, *mut regulator_irq_data, *mut usize) -> i32>,
    pub renable: Option<unsafe extern "C" fn(*mut regulator_irq_data) -> i32>,
}

pub const REGULATOR_ERROR_CLEARED: i32 = 0;
pub const REGULATOR_FAILED_RETRY: i32 = 1;
pub const REGULATOR_ERROR_ON: i32 = 2;

#[repr(C)]
pub struct coupling_desc { pub coupled_rdevs: *mut *mut regulator_dev, pub coupler: *mut regulator_coupler, pub n_resolved: i32, pub n_coupled: i32 }

#[repr(C)]
pub struct regulator_dev {
    pub desc: *const regulator_desc, pub exclusive: i32, pub use_count: u32, pub open_count: u32, pub bypass_count: u32,
    pub list: list_head, pub consumer_list: list_head, pub coupling_desc: coupling_desc,
    pub notifier: blocking_notifier_head, pub mutex: ww_mutex, pub mutex_owner: *mut task_struct,
    pub ref_cnt: i32, pub owner: *mut module, pub dev: device, pub bdev: device,
    pub constraints: *mut regulation_constraints, pub supply: *mut regulator, pub supply_name: *const i8,
    pub regmap: *mut regmap, pub disable_work: delayed_work, pub reg_data: *mut core::ffi::c_void,
    pub debugfs: *mut dentry, pub ena_pin: *mut regulator_enable_gpio, pub ena_gpio_state: u32,
    pub constraints_pending: u32, pub is_switch: u32, pub last_off: ktime_t, pub cached_err: i32,
    pub use_cached_err: bool, pub err_lock: spinlock_t, pub pw_requested_mW: i32, pub supply_fwd_nb: notifier_block,
}

/* Error constants and notification constants are supplied by consumer.h. */
pub unsafe fn regulator_err2notif(err: i32) -> i32 {
    match err {
        REGULATOR_ERROR_UNDER_VOLTAGE => REGULATOR_EVENT_UNDER_VOLTAGE,
        REGULATOR_ERROR_OVER_CURRENT => REGULATOR_EVENT_OVER_CURRENT,
        REGULATOR_ERROR_REGULATION_OUT => REGULATOR_EVENT_REGULATION_OUT,
        REGULATOR_ERROR_FAIL => REGULATOR_EVENT_FAIL,
        REGULATOR_ERROR_OVER_TEMP => REGULATOR_EVENT_OVER_TEMP,
        REGULATOR_ERROR_UNDER_VOLTAGE_WARN => REGULATOR_EVENT_UNDER_VOLTAGE_WARN,
        REGULATOR_ERROR_OVER_CURRENT_WARN => REGULATOR_EVENT_OVER_CURRENT_WARN,
        REGULATOR_ERROR_OVER_VOLTAGE_WARN => REGULATOR_EVENT_OVER_VOLTAGE_WARN,
        REGULATOR_ERROR_OVER_TEMP_WARN => REGULATOR_EVENT_OVER_TEMP_WARN,
        _ => 0,
    }
}

unsafe extern "C" {
    pub fn regulator_register(dev: *mut device, desc: *const regulator_desc, config: *const regulator_config) -> *mut regulator_dev;
    pub fn devm_regulator_register(dev: *mut device, desc: *const regulator_desc, config: *const regulator_config) -> *mut regulator_dev;
    pub fn regulator_unregister(rdev: *mut regulator_dev);
    pub fn regulator_notifier_call_chain(rdev: *mut regulator_dev, event: usize, data: *mut core::ffi::c_void) -> i32;
    pub fn devm_regulator_irq_helper(dev: *mut device, d: *const regulator_irq_desc, irq: i32, irq_flags: i32, common_errs: i32, per_rdev_errs: *mut i32, rdev: *mut *mut regulator_dev, rdev_amount: i32) -> *mut core::ffi::c_void;
    pub fn regulator_irq_helper(dev: *mut device, d: *const regulator_irq_desc, irq: i32, irq_flags: i32, common_errs: i32, per_rdev_errs: *mut i32, rdev: *mut *mut regulator_dev, rdev_amount: i32) -> *mut core::ffi::c_void;
    pub fn regulator_irq_helper_cancel(handle: *mut *mut core::ffi::c_void);
    pub fn regulator_irq_map_event_simple(irq: i32, rid: *mut regulator_irq_data, dev_mask: *mut usize) -> i32;
    pub fn rdev_get_drvdata(rdev: *mut regulator_dev) -> *mut core::ffi::c_void;
    pub fn rdev_get_dev(rdev: *mut regulator_dev) -> *mut device;
    pub fn rdev_get_regmap(rdev: *mut regulator_dev) -> *mut regmap;
    pub fn rdev_get_id(rdev: *mut regulator_dev) -> i32;
    pub fn regulator_mode_to_status(mode: u32) -> i32;
    pub fn regulator_list_voltage_linear(rdev: *mut regulator_dev, selector: u32) -> i32;
    pub fn regulator_list_voltage_pickable_linear_range(rdev: *mut regulator_dev, selector: u32) -> i32;
    pub fn regulator_list_voltage_linear_range(rdev: *mut regulator_dev, selector: u32) -> i32;
    pub fn regulator_list_voltage_table(rdev: *mut regulator_dev, selector: u32) -> i32;
    pub fn regulator_map_voltage_linear(rdev: *mut regulator_dev, min_uV: i32, max_uV: i32) -> i32;
    pub fn regulator_map_voltage_pickable_linear_range(rdev: *mut regulator_dev, min_uV: i32, max_uV: i32) -> i32;
    pub fn regulator_map_voltage_linear_range(rdev: *mut regulator_dev, min_uV: i32, max_uV: i32) -> i32;
    pub fn regulator_map_voltage_iterate(rdev: *mut regulator_dev, min_uV: i32, max_uV: i32) -> i32;
    pub fn regulator_map_voltage_ascend(rdev: *mut regulator_dev, min_uV: i32, max_uV: i32) -> i32;
    pub fn regulator_get_voltage_sel_pickable_regmap(rdev: *mut regulator_dev) -> i32;
    pub fn regulator_set_voltage_sel_pickable_regmap(rdev: *mut regulator_dev, sel: u32) -> i32;
    pub fn regulator_get_voltage_sel_regmap(rdev: *mut regulator_dev) -> i32;
    pub fn regulator_set_voltage_sel_regmap(rdev: *mut regulator_dev, sel: u32) -> i32;
    pub fn regulator_is_enabled_regmap(rdev: *mut regulator_dev) -> i32;
    pub fn regulator_enable_regmap(rdev: *mut regulator_dev) -> i32;
    pub fn regulator_disable_regmap(rdev: *mut regulator_dev) -> i32;
    pub fn regulator_set_voltage_time_sel(rdev: *mut regulator_dev, old_selector: u32, new_selector: u32) -> i32;
    pub fn regulator_set_bypass_regmap(rdev: *mut regulator_dev, enable: bool) -> i32;
    pub fn regulator_get_bypass_regmap(rdev: *mut regulator_dev, enable: *mut bool) -> i32;
    pub fn regulator_set_soft_start_regmap(rdev: *mut regulator_dev) -> i32;
    pub fn regulator_set_pull_down_regmap(rdev: *mut regulator_dev) -> i32;
    pub fn regulator_set_active_discharge_regmap(rdev: *mut regulator_dev, enable: bool) -> i32;
    pub fn regulator_set_current_limit_regmap(rdev: *mut regulator_dev, min_uA: i32, max_uA: i32) -> i32;
    pub fn regulator_get_current_limit_regmap(rdev: *mut regulator_dev) -> i32;
    pub fn regulator_get_init_drvdata(data: *mut regulator_init_data) -> *mut core::ffi::c_void;
    pub fn regulator_find_closest_bigger(target: u32, table: *const u32, num_sel: u32, sel: *mut u32) -> i32;
    pub fn regulator_set_ramp_delay_regmap(rdev: *mut regulator_dev, ramp_delay: i32) -> i32;
    pub fn regulator_sync_voltage_rdev(rdev: *mut regulator_dev) -> i32;
    pub fn regulator_desc_list_voltage_linear_range(desc: *const regulator_desc, selector: u32) -> i32;
    pub fn regulator_desc_list_voltage_linear(desc: *const regulator_desc, selector: u32) -> i32;
}

#[cfg(feature = "CONFIG_REGULATOR")]
pub unsafe extern "C" fn rdev_get_name(rdev: *mut regulator_dev) -> *const i8;

#[cfg(not(feature = "CONFIG_REGULATOR"))]
pub unsafe fn rdev_get_name(_rdev: *mut regulator_dev) -> *const i8 { core::ptr::null() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
