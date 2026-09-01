// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/iostat.c.
// Original dependencies: "util/iostat.h", "util/debug.h".

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct evlist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_stat_config {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct option {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_stat_output_ctx {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct timespec {
    _unused: [u8; 0],
}

pub type iostat_mode_t = c_int;

pub const IOSTAT_NONE: iostat_mode_t = 0;

pub type iostat_print_counter_t = Option<
    unsafe extern "C" fn(
        evlist: *mut evlist,
        config: *mut perf_stat_config,
        ts: *mut timespec,
        prefix: *mut c_char,
        arg: *mut c_void,
    ),
>;

unsafe extern "C" {
    fn pr_err(fmt: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub static mut iostat_mode: iostat_mode_t = IOSTAT_NONE;

// C source declares this symbol __weak.
#[no_mangle]
pub unsafe extern "C" fn iostat_prepare(
    _evlist: *mut *mut evlist,
    _config: *mut perf_stat_config,
) -> c_int {
    -1
}

// C source declares this symbol __weak.
#[no_mangle]
pub unsafe extern "C" fn iostat_parse(
    _opt: *const option,
    _str: *const c_char,
    _unset: c_int,
) -> c_int {
    unsafe {
        pr_err(c"iostat mode is not supported on current platform\n".as_ptr());
    }
    -1
}

// C source declares this symbol __weak.
#[no_mangle]
pub unsafe extern "C" fn iostat_list(
    _evlist: *mut evlist,
    _config: *mut perf_stat_config,
) {
}

// C source declares this symbol __weak.
#[no_mangle]
pub unsafe extern "C" fn iostat_release(_evlist: *mut evlist) {}

// C source declares this symbol __weak.
#[no_mangle]
pub unsafe extern "C" fn iostat_print_header_prefix(_config: *mut perf_stat_config) {}

// C source declares this symbol __weak.
#[no_mangle]
pub unsafe extern "C" fn iostat_print_metric(
    _config: *mut perf_stat_config,
    _evsel: *mut evsel,
    _out: *mut perf_stat_output_ctx,
) {
}

// C source declares this symbol __weak.
#[no_mangle]
pub unsafe extern "C" fn iostat_prefix(
    _evlist: *mut evlist,
    _config: *mut perf_stat_config,
    _prefix: *mut c_char,
    _ts: *mut timespec,
) {
}

// C source declares this symbol __weak.
#[no_mangle]
pub unsafe extern "C" fn iostat_print_counters(
    _evlist: *mut evlist,
    _config: *mut perf_stat_config,
    _ts: *mut timespec,
    _prefix: *mut c_char,
    _print_cnt_cb: iostat_print_counter_t,
    _arg: *mut c_void,
) {
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
