/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <cpu/rtc.h>; its declarations are supplied by another
// translated dependency.

pub const RTC_CAP_4_DIGIT_YEAR: u32 = 1 << 0;

#[repr(C)]
pub struct sh_rtc_platform_info {
    pub capabilities: core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
