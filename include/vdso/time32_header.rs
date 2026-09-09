/* SPDX-License-Identifier: GPL-2.0 */

#[allow(non_camel_case_types)]
pub type old_time32_t = i32;

#[repr(C)]
pub struct old_timespec32 {
    pub tv_sec: old_time32_t,
    pub tv_nsec: i32,
}

#[repr(C)]
pub struct old_timeval32 {
    pub tv_sec: old_time32_t,
    pub tv_usec: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
