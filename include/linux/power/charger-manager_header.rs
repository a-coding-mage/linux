/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2011 Samsung Electronics Co., Ltd.
 * MyungJoo.Ham <myungjoo.ham@samsung.com>
 *
 * Charger Manager.
 * This framework enables to control and multiple chargers and to
 * monitor charging even in the context of suspend-to-RAM with
 * an interface combining the chargers.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced by name here.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum data_source {
    CM_BATTERY_PRESENT,
    CM_NO_BATTERY,
    CM_FUEL_GAUGE,
    CM_CHARGER_STAT,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum polling_modes {
    CM_POLL_DISABLE = 0,
    CM_POLL_ALWAYS,
    CM_POLL_EXTERNAL_POWER_ONLY,
    CM_POLL_CHARGING_ONLY,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cm_batt_temp {
    CM_BATT_OK = 0,
    CM_BATT_OVERHEAT,
    CM_BATT_COLD,
}

#[repr(C)]
pub struct charger_cable {
    pub extcon_name: *const core::ffi::c_char,
    pub name: *const core::ffi::c_char,
    pub extcon_dev: *mut extcon_dev,
    pub extcon_type: u64,
    pub wq: work_struct,
    pub nb: notifier_block,
    pub attached: bool,
    pub charger: *mut charger_regulator,
    pub min_uA: i32,
    pub max_uA: i32,
    pub cm: *mut charger_manager,
}

#[repr(C)]
pub struct charger_regulator {
    pub regulator_name: *const core::ffi::c_char,
    pub consumer: *mut regulator,
    pub externally_control: i32,
    pub cables: *mut charger_cable,
    pub num_cables: i32,
    pub attr_grp: attribute_group,
    pub attr_name: device_attribute,
    pub attr_state: device_attribute,
    pub attr_externally_control: device_attribute,
    pub attrs: [*mut attribute; 4],
    pub cm: *mut charger_manager,
}

#[repr(C)]
pub struct charger_desc {
    pub psy_name: *const core::ffi::c_char,
    pub polling_mode: polling_modes,
    pub polling_interval_ms: u32,
    pub fullbatt_vchkdrop_uV: u32,
    pub fullbatt_uV: u32,
    pub fullbatt_soc: u32,
    pub fullbatt_full_capacity: u32,
    pub battery_present: data_source,
    pub psy_charger_stat: *const *const core::ffi::c_char,
    pub num_charger_regulators: i32,
    pub charger_regulators: *mut charger_regulator,
    pub sysfs_groups: *const *const attribute_group,
    pub psy_fuel_gauge: *const core::ffi::c_char,
    pub thermal_zone: *const core::ffi::c_char,
    pub temp_min: i32,
    pub temp_max: i32,
    pub temp_diff: i32,
    pub measure_battery_temp: bool,
    pub charging_max_duration_ms: u32,
    pub discharging_max_duration_ms: u32,
}

pub const PSY_NAME_MAX: usize = 30;

#[repr(C)]
pub struct charger_manager {
    pub entry: list_head,
    pub dev: *mut device,
    pub desc: *mut charger_desc,

    #[cfg(CONFIG_THERMAL)]
    pub tzd_batt: *mut thermal_zone_device,
    pub charger_enabled: bool,
    pub emergency_stop: i32,
    pub psy_name_buf: [core::ffi::c_char; PSY_NAME_MAX + 1],
    pub charger_psy_desc: power_supply_desc,
    pub charger_psy: *mut power_supply,
    pub charging_start_time: u64,
    pub charging_end_time: u64,
    pub battery_status: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
