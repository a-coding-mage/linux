/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency provided by the Linux type definitions.

#[repr(C)]
pub struct xt_time_info {
    pub date_start: __u32,
    pub date_stop: __u32,
    pub daytime_start: __u32,
    pub daytime_stop: __u32,
    pub monthdays_match: __u32,
    pub weekdays_match: __u8,
    pub flags: __u8,
}

/* Match against local time (instead of UTC) */
pub const XT_TIME_LOCAL_TZ: u32 = 1 << 0;

/* treat timestart > timestop (e.g. 23:00-01:00) as single period */
pub const XT_TIME_CONTIGUOUS: u32 = 1 << 1;

/* Shortcuts */
pub const XT_TIME_ALL_MONTHDAYS: u32 = 0xFFFFFFFE;
pub const XT_TIME_ALL_WEEKDAYS: u32 = 0xFE;
pub const XT_TIME_MIN_DAYTIME: u32 = 0;
pub const XT_TIME_MAX_DAYTIME: u32 = 24 * 60 * 60 - 1;

pub const XT_TIME_ALL_FLAGS: u32 = XT_TIME_LOCAL_TZ | XT_TIME_CONTIGUOUS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
