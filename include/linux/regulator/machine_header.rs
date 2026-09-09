/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * machine.h -- SoC Regulator support, machine/board driver API.
 *
 * Rust translation of the C header. C header dependencies are supplied by
 * other translation units.
 */

// C dependencies: linux/regulator/consumer.h, linux/suspend.h

pub const REGULATOR_CHANGE_VOLTAGE: u32 = 0x1;
pub const REGULATOR_CHANGE_CURRENT: u32 = 0x2;
pub const REGULATOR_CHANGE_MODE: u32 = 0x4;
pub const REGULATOR_CHANGE_STATUS: u32 = 0x8;
pub const REGULATOR_CHANGE_DRMS: u32 = 0x10;
pub const REGULATOR_CHANGE_BYPASS: u32 = 0x20;

pub const DO_NOTHING_IN_SUSPEND: i32 = 0;
pub const DISABLE_IN_SUSPEND: i32 = 1;
pub const ENABLE_IN_SUSPEND: i32 = 2;
pub const REGULATOR_DEF_UV_LESS_CRITICAL_WINDOW_MS: i32 = 10;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum regulator_active_discharge {
    REGULATOR_ACTIVE_DISCHARGE_DEFAULT,
    REGULATOR_ACTIVE_DISCHARGE_DISABLE,
    REGULATOR_ACTIVE_DISCHARGE_ENABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_state {
    pub uV: i32,
    pub min_uV: i32,
    pub max_uV: i32,
    pub mode: u32,
    pub enabled: i32,
    pub changeable: bool,
}

pub const REGULATOR_NOTIF_LIMIT_DISABLE: i32 = -1;
pub const REGULATOR_NOTIF_LIMIT_ENABLE: i32 = -2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notification_limit {
    pub prot: i32,
    pub err: i32,
    pub warn: i32,
}

#[repr(C)]
pub struct regulation_constraints {
    pub name: *const core::ffi::c_char,
    pub min_uV: i32,
    pub max_uV: i32,
    pub uV_offset: i32,
    pub min_uA: i32,
    pub max_uA: i32,
    pub ilim_uA: i32,
    pub pw_budget_mW: i32,
    pub system_load: i32,
    pub max_spread: *mut u32,
    pub max_uV_step: i32,
    pub valid_modes_mask: u32,
    pub valid_ops_mask: u32,
    pub input_uV: i32,
    pub state_disk: regulator_state,
    pub state_mem: regulator_state,
    pub state_standby: regulator_state,
    pub over_curr_limits: notification_limit,
    pub over_voltage_limits: notification_limit,
    pub under_voltage_limits: notification_limit,
    pub temp_limits: notification_limit,
    pub initial_state: suspend_state_t,
    pub initial_mode: u32,
    pub ramp_delay: u32,
    pub settling_time: u32,
    pub settling_time_up: u32,
    pub settling_time_down: u32,
    pub enable_time: u32,
    pub uv_less_critical_window_ms: u32,
    pub active_discharge: u32,
    // C bit-fields (one-bit unsigned flags).
    pub always_on: u8,
    pub boot_on: u8,
    pub apply_uV: u8,
    pub ramp_disable: u8,
    pub soft_start: u8,
    pub pull_down: u8,
    pub system_critical: u8,
    pub over_current_protection: u8,
    pub over_current_detection: u8,
    pub over_voltage_detection: u8,
    pub under_voltage_detection: u8,
    pub over_temp_detection: u8,
}

#[repr(C)]
pub struct regulator_consumer_supply {
    pub dev_name: *const core::ffi::c_char,
    pub supply: *const core::ffi::c_char,
}

#[macro_export]
macro_rules! REGULATOR_SUPPLY {
    ($name:expr, $dev_name:expr) => {
        $crate::regulator_consumer_supply { supply: $name, dev_name: $dev_name }
    };
}

#[repr(C)]
pub struct regulator_init_data {
    pub supply_regulator: *const core::ffi::c_char,
    pub constraints: regulation_constraints,
    pub num_consumer_supplies: i32,
    pub consumer_supplies: *mut regulator_consumer_supply,
    pub driver_data: *mut core::ffi::c_void,
}

#[cfg(feature = "CONFIG_REGULATOR")]
extern "C" {
    pub fn regulator_has_full_constraints();
}

#[cfg(not(feature = "CONFIG_REGULATOR"))]
#[inline]
pub fn regulator_has_full_constraints() {}

#[inline]
pub fn regulator_suspend_prepare(_state: suspend_state_t) -> i32 { 0 }

#[inline]
pub fn regulator_suspend_finish() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
