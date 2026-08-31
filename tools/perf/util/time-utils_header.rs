/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int};

pub type u64 = u64;
pub type size_t = usize;

#[repr(C)]
pub struct perf_time_interval {
    pub start: u64,
    pub end: u64,
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn parse_nsec_time(str: *const c_char, ptime: *mut u64) -> c_int;

    pub fn perf_time__parse_str(ptime: *mut perf_time_interval, ostr: *const c_char) -> c_int;

    pub fn perf_time__percent_parse_str(
        ptime_buf: *mut perf_time_interval,
        num: c_int,
        ostr: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;

    pub fn perf_time__range_alloc(
        ostr: *const c_char,
        size: *mut c_int,
    ) -> *mut perf_time_interval;

    pub fn perf_time__skip_sample(ptime: *mut perf_time_interval, timestamp: u64) -> bool;

    pub fn perf_time__ranges_skip_sample(
        ptime_buf: *mut perf_time_interval,
        num: c_int,
        timestamp: u64,
    ) -> bool;

    pub fn perf_time__parse_for_ranges_reltime(
        str: *const c_char,
        session: *mut perf_session,
        ranges: *mut *mut perf_time_interval,
        range_size: *mut c_int,
        range_num: *mut c_int,
        reltime: bool,
    ) -> c_int;

    pub fn perf_time__parse_for_ranges(
        str: *const c_char,
        session: *mut perf_session,
        ranges: *mut *mut perf_time_interval,
        range_size: *mut c_int,
        range_num: *mut c_int,
    ) -> c_int;

    pub fn timestamp__scnprintf_usec(timestamp: u64, buf: *mut c_char, sz: size_t) -> c_int;
    pub fn timestamp__scnprintf_nsec(timestamp: u64, buf: *mut c_char, sz: size_t) -> c_int;

    pub fn fetch_current_timestamp(buf: *mut c_char, sz: size_t) -> c_int;
}

pub unsafe fn rdclock() -> u64 {
    let mut ts: libc::timespec = core::mem::zeroed();

    libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
    (ts.tv_sec as u64)
        .wrapping_mul(1000000000u64)
        .wrapping_add(ts.tv_nsec as u64)
}
