/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SharpSL Battery/PM Driver
 *
 * Copyright (c) 2004-2005 Richard Purdie
 */

#[repr(C)]
pub struct sharpsl_charger_machinfo {
    pub init: Option<unsafe extern "C" fn()>,
    pub exit: Option<unsafe extern "C" fn()>,
    pub gpio_acin: i32,
    pub gpio_batfull: i32,
    pub batfull_irq: i32,
    pub gpio_batlock: i32,
    pub gpio_fatal: i32,
    pub discharge: Option<unsafe extern "C" fn(i32)>,
    pub discharge1: Option<unsafe extern "C" fn(i32)>,
    pub charge: Option<unsafe extern "C" fn(i32)>,
    pub measure_temp: Option<unsafe extern "C" fn(i32)>,
    pub presuspend: Option<unsafe extern "C" fn()>,
    pub postsuspend: Option<unsafe extern "C" fn()>,
    pub earlyresume: Option<unsafe extern "C" fn()>,
    pub read_devdata: Option<unsafe extern "C" fn(i32) -> ::core::ffi::c_ulong>,
    pub charger_wakeup: Option<unsafe extern "C" fn() -> bool>,
    pub should_wakeup: Option<unsafe extern "C" fn(u32) -> i32>,
    pub backlight_limit: Option<unsafe extern "C" fn(i32)>,
    pub backlight_get_status: Option<unsafe extern "C" fn() -> i32>,
    pub charge_on_volt: i32,
    pub charge_on_temp: i32,
    pub charge_acin_high: i32,
    pub charge_acin_low: i32,
    pub fatal_acin_volt: i32,
    pub fatal_noacin_volt: i32,
    pub bat_levels: i32,
    pub bat_levels_noac: *mut battery_thresh,
    pub bat_levels_acin: *mut battery_thresh,
    pub bat_levels_noac_bl: *mut battery_thresh,
    pub bat_levels_acin_bl: *mut battery_thresh,
    pub status_high_acin: i32,
    pub status_low_acin: i32,
    pub status_high_noac: i32,
    pub status_low_noac: i32,
}

#[repr(C)]
pub struct battery_thresh {
    pub voltage: i32,
    pub percentage: i32,
}

#[repr(C)]
pub struct battery_stat {
    pub ac_status: i32,       /* APM AC Present/Not Present */
    pub mainbat_status: i32,  /* APM Main Battery Status */
    pub mainbat_percent: i32, /* Main Battery Percentage Charge */
    pub mainbat_voltage: i32, /* Main Battery Voltage */
}

#[repr(C)]
pub struct sharpsl_pm_status {
    pub dev: *mut device,
    pub ac_timer: timer_list,
    pub chrg_full_timer: timer_list,
    pub charge_mode: i32,
    pub flags: u32,
    pub full_count: i32,
    pub charge_start_time: ::core::ffi::c_ulong,
    pub machinfo: *mut sharpsl_charger_machinfo,
    pub battstat: battery_stat,
}

pub const SHARPSL_BATT_VOLT: i32 = 1;
pub const SHARPSL_BATT_TEMP: i32 = 2;
pub const SHARPSL_ACIN_VOLT: i32 = 3;
pub const SHARPSL_STATUS_ACIN: i32 = 4;
pub const SHARPSL_STATUS_LOCK: i32 = 5;
pub const SHARPSL_STATUS_CHRGFULL: i32 = 6;
pub const SHARPSL_STATUS_FATAL: i32 = 7;

pub const CHRG_ERROR: i32 = -1;
pub const CHRG_OFF: i32 = 0;
pub const CHRG_ON: i32 = 1;
pub const CHRG_DONE: i32 = 2;

pub const SHARPSL_SUSPENDED: u32 = 1 << 0;       /* Device is Suspended */
pub const SHARPSL_ALARM_ACTIVE: u32 = 1 << 1;    /* Alarm is for charging event (not user) */
pub const SHARPSL_BL_LIMIT: u32 = 1 << 2;        /* Backlight Intensity Limited */
pub const SHARPSL_APM_QUEUED: u32 = 1 << 3;      /* APM Event Queued */
pub const SHARPSL_DO_OFFLINE_CHRG: u32 = 1 << 4; /* Trigger the offline charger */

extern "C" {
    pub static mut sharpsl_pm: sharpsl_pm_status;
    pub static mut sharpsl_battery_levels_acin: [battery_thresh; 0];
    pub static mut sharpsl_battery_levels_noac: [battery_thresh; 0];

    pub fn sharpsl_battery_kick();
    pub fn sharpsl_pm_led(val: i32);
    pub fn sharpsl_pm_pxa_read_max1111(channel: i32) -> i32;
}

pub const SHARPSL_LED_ERROR: i32 = 2;
pub const SHARPSL_LED_ON: i32 = 1;
pub const SHARPSL_LED_OFF: i32 = 0;

/* MAX1111 Channel Definitions */
pub const MAX1111_BATT_VOLT: u32 = 4;
pub const MAX1111_BATT_TEMP: u32 = 2;
pub const MAX1111_ACIN_VOLT: u32 = 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
