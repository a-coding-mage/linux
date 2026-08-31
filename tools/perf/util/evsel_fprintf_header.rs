// SPDX-License-Identifier: GPL-2.0

// C dependencies: <stdio.h>, <stdbool.h>

#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_attr_details {
    pub freq: bool,
    pub verbose: bool,
    pub event_group: bool,
    pub force: bool,
    pub trace_fields: bool,
}

unsafe extern "C" {
    pub fn evsel__fprintf(
        evsel: *mut evsel,
        details: *mut perf_attr_details,
        fp: *mut FILE,
    ) -> ::std::os::raw::c_int;
}

pub const EVSEL__PRINT_IP: ::std::os::raw::c_uint = 1 << 0;
pub const EVSEL__PRINT_SYM: ::std::os::raw::c_uint = 1 << 1;
pub const EVSEL__PRINT_DSO: ::std::os::raw::c_uint = 1 << 2;
pub const EVSEL__PRINT_SYMOFFSET: ::std::os::raw::c_uint = 1 << 3;
pub const EVSEL__PRINT_ONELINE: ::std::os::raw::c_uint = 1 << 4;
pub const EVSEL__PRINT_SRCLINE: ::std::os::raw::c_uint = 1 << 5;
pub const EVSEL__PRINT_UNKNOWN_AS_ADDR: ::std::os::raw::c_uint = 1 << 6;
pub const EVSEL__PRINT_CALLCHAIN_ARROW: ::std::os::raw::c_uint = 1 << 7;
pub const EVSEL__PRINT_SKIP_IGNORED: ::std::os::raw::c_uint = 1 << 8;
pub const EVSEL__PRINT_DSOFF: ::std::os::raw::c_uint = 1 << 9;

#[repr(C)]
pub struct addr_location {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct callchain_cursor {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct strlist {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn sample__fprintf_callchain(
        sample: *mut perf_sample,
        left_alignment: ::std::os::raw::c_int,
        print_opts: ::std::os::raw::c_uint,
        cursor: *mut callchain_cursor,
        bt_stop_list: *mut strlist,
        fp: *mut FILE,
    ) -> ::std::os::raw::c_int;

    pub fn sample__fprintf_sym(
        sample: *mut perf_sample,
        al: *mut addr_location,
        left_alignment: ::std::os::raw::c_int,
        print_opts: ::std::os::raw::c_uint,
        cursor: *mut callchain_cursor,
        bt_stop_list: *mut strlist,
        fp: *mut FILE,
    ) -> ::std::os::raw::c_int;
}

pub type attr__fprintf_f = Option<
    unsafe extern "C" fn(
        *mut FILE,
        *const ::std::os::raw::c_char,
        *const ::std::os::raw::c_char,
        *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;

unsafe extern "C" {
    pub fn perf_event_attr__fprintf(
        fp: *mut FILE,
        attr: *mut perf_event_attr,
        attr__fprintf: attr__fprintf_f,
        priv_: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}

// FILE is supplied by the Rust translation of <stdio.h> or an equivalent libc binding.
#[repr(C)]
pub struct FILE {
    _unused: [u8; 0],
}
