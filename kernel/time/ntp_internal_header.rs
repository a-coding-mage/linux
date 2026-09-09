/* SPDX-License-Identifier: GPL-2.0 */

// Translated from _LINUX_NTP_INTERNAL_H.

unsafe extern "C" {
    pub fn ntp_init();
    pub fn ntp_clear(tkid: ::core::ffi::c_uint, cs_tick_adj: s64);
    // Returns how long ticks are at present, in ns / 2^NTP_SCALE_SHIFT.
    pub fn ntp_tick_length(tkid: ::core::ffi::c_uint) -> u64;
    pub fn ntp_get_skew_delta(tkid: ::core::ffi::c_uint) -> s64;
    pub fn ntp_drain_skew(
        tkid: ::core::ffi::c_uint,
        amount: s64,
        shift: ::core::ffi::c_uint,
    ) -> s64;
    pub fn ntp_get_next_leap(tkid: ::core::ffi::c_uint) -> ktime_t;
    pub fn second_overflow(tkid: ::core::ffi::c_uint, secs: time64_t) -> ::core::ffi::c_int;
    pub fn ntp_adjtimex(
        tkid: ::core::ffi::c_uint,
        txc: *mut __kernel_timex,
        ts: *const timespec64,
        time_tai: *mut s32,
        ad: *mut audit_ntp_data,
    ) -> ::core::ffi::c_int;
    pub fn __hardpps(phase_ts: *const timespec64, raw_ts: *const timespec64);
}

#[cfg(any(CONFIG_GENERIC_CMOS_UPDATE, CONFIG_RTC_SYSTOHC))]
unsafe extern "C" {
    pub fn ntp_notify_cmos_timer(offset_set: bool);
}

#[cfg(not(any(CONFIG_GENERIC_CMOS_UPDATE, CONFIG_RTC_SYSTOHC)))]
#[inline]
pub fn ntp_notify_cmos_timer(_offset_set: bool) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
