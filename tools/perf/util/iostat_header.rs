/* SPDX-License-Identifier: GPL-2.0 */
/*
 * perf iostat
 *
 * Copyright (C) 2020, Intel Corporation
 *
 * Authors: Alexander Antonov <alexander.antonov@linux.intel.com>
 */

/* C header dependencies:
 * <subcmd/parse-options.h>
 * "util/stat.h"
 * "util/parse-events.h"
 * "util/evlist.h"
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_stat_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timespec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_stat_output_ctx {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum iostat_mode_t {
    IOSTAT_NONE = -1,
    IOSTAT_RUN = 0,
    IOSTAT_LIST = 1,
}

pub type iostat_print_counter_t = Option<
    unsafe extern "C" fn(
        config: *mut perf_stat_config,
        evsel: *mut evsel,
        arg: *mut c_void,
    ),
>;

unsafe extern "C" {
    pub static mut iostat_mode: iostat_mode_t;

    pub fn iostat_prepare(evlist: *mut *mut evlist, config: *mut perf_stat_config) -> c_int;
    pub fn iostat_parse(opt: *const option, str: *const c_char, unset: c_int) -> c_int;
    pub fn iostat_list(evlist: *mut evlist, config: *mut perf_stat_config);
    pub fn iostat_release(evlist: *mut evlist);
    pub fn iostat_prefix(
        evlist: *mut evlist,
        config: *mut perf_stat_config,
        prefix: *mut c_char,
        ts: *mut timespec,
    );
    pub fn iostat_print_header_prefix(config: *mut perf_stat_config);
    pub fn iostat_print_metric(
        config: *mut perf_stat_config,
        evsel: *mut evsel,
        out: *mut perf_stat_output_ctx,
    );
    pub fn iostat_print_counters(
        evlist: *mut evlist,
        config: *mut perf_stat_config,
        ts: *mut timespec,
        prefix: *mut c_char,
        print_cnt_cb: iostat_print_counter_t,
        arg: *mut c_void,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
