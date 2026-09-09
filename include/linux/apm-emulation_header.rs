/* SPDX-License-Identifier: GPL-2.0-only */
/* -*- linux-c -*-
 *
 * (C) 2003 zecke@handhelds.org
 *
 * based on arch/arm/kernel/apm.c
 * factor out the information needed by architectures to provide
 * apm status
 */

// Dependency intent: the C header includes <linux/apm_bios.h>.

/*
 * This structure gets filled in by the machine specific 'get_power_status'
 * implementation.  Any fields which are not set default to a safe value.
 */
#[repr(C)]
pub struct apm_power_info {
    pub ac_line_status: u8,
    pub battery_status: u8,
    pub battery_flag: u8,
    pub battery_life: core::ffi::c_int,
    pub time: core::ffi::c_int,
    pub units: core::ffi::c_int,
}

pub const APM_AC_OFFLINE: u8 = 0;
pub const APM_AC_ONLINE: u8 = 1;
pub const APM_AC_BACKUP: u8 = 2;
pub const APM_AC_UNKNOWN: u8 = 0xff;

pub const APM_BATTERY_STATUS_HIGH: u8 = 0;
pub const APM_BATTERY_STATUS_LOW: u8 = 1;
pub const APM_BATTERY_STATUS_CRITICAL: u8 = 2;
pub const APM_BATTERY_STATUS_CHARGING: u8 = 3;
pub const APM_BATTERY_STATUS_NOT_PRESENT: u8 = 4;
pub const APM_BATTERY_STATUS_UNKNOWN: u8 = 0xff;

pub const APM_BATTERY_FLAG_HIGH: u8 = 1 << 0;
pub const APM_BATTERY_FLAG_LOW: u8 = 1 << 1;
pub const APM_BATTERY_FLAG_CRITICAL: u8 = 1 << 2;
pub const APM_BATTERY_FLAG_CHARGING: u8 = 1 << 3;
pub const APM_BATTERY_FLAG_NOT_PRESENT: u8 = 1 << 7;
pub const APM_BATTERY_FLAG_UNKNOWN: u8 = 0xff;

pub const APM_UNITS_MINS: core::ffi::c_int = 0;
pub const APM_UNITS_SECS: core::ffi::c_int = 1;
pub const APM_UNITS_UNKNOWN: core::ffi::c_int = -1;

/*
 * This allows machines to provide their own "apm get power status" function.
 */
extern "C" {
    pub static mut apm_get_power_status:
        Option<unsafe extern "C" fn(*mut apm_power_info)>;
}

/*
 * Queue an event (APM_SYS_SUSPEND or APM_CRITICAL_SUSPEND)
 */
extern "C" {
    pub fn apm_queue_event(event: apm_event_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
