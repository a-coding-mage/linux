/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <stdio.h>, <linux/types.h>

pub type u32 = ::std::os::raw::c_uint;
pub type u64 = ::std::os::raw::c_ulonglong;

#[repr(C)]
pub struct FILE {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_read_values {
    pub threads: ::std::os::raw::c_int,
    pub threads_max: ::std::os::raw::c_int,
    pub pid: *mut u32,
    pub tid: *mut u32,
    pub num_counters: ::std::os::raw::c_int,
    pub counters_max: ::std::os::raw::c_int,
    pub counters: *mut *mut evsel,
    pub value: *mut *mut u64,
}

unsafe extern "C" {
    pub fn perf_read_values_init(values: *mut perf_read_values) -> ::std::os::raw::c_int;
    pub fn perf_read_values_destroy(values: *mut perf_read_values);

    pub fn perf_read_values_add_value(
        values: *mut perf_read_values,
        pid: u32,
        tid: u32,
        evsel: *mut evsel,
        value: u64,
    ) -> ::std::os::raw::c_int;

    pub fn perf_read_values_display(
        fp: *mut FILE,
        values: *mut perf_read_values,
        raw: ::std::os::raw::c_int,
    );
}
