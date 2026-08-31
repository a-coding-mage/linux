/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from perf/util/block-info.h.
 *
 * C header includes omitted from executable Rust:
 * - <linux/types.h>
 * - "hist.h"
 * - "symbol.h"
 * - "sort.h"
 * - "ui/ui.h"
 *
 * The referenced types and constants are expected to be supplied by the
 * surrounding translated project.
 */

use core::ffi::{c_char, c_float, c_int, c_uint};

#[repr(C)]
pub struct block_info {
    pub sym: *mut symbol,
    pub start: u64,
    pub end: u64,
    pub cycles: u64,
    pub cycles_aggr: u64,
    pub cycles_spark: [i64; NUM_SPARKS],
    pub total_cycles: u64,
    pub num: c_int,
    pub num_aggr: c_int,
    pub br_cntr_nr: c_int,
    pub br_cntr: *mut u64,
    pub evsel: *mut evsel,
}

#[repr(C)]
pub struct block_fmt {
    pub fmt: perf_hpp_fmt,
    pub idx: c_int,
    pub width: c_int,
    pub header: *const c_char,
    pub total_cycles: u64,
    pub block_cycles: u64,
}

pub const PERF_HPP_REPORT__BLOCK_TOTAL_CYCLES_PCT: c_int = 0;
pub const PERF_HPP_REPORT__BLOCK_LBR_CYCLES: c_int = 1;
pub const PERF_HPP_REPORT__BLOCK_CYCLES_PCT: c_int = 2;
pub const PERF_HPP_REPORT__BLOCK_AVG_CYCLES: c_int = 3;
pub const PERF_HPP_REPORT__BLOCK_RANGE: c_int = 4;
pub const PERF_HPP_REPORT__BLOCK_DSO: c_int = 5;
pub const PERF_HPP_REPORT__BLOCK_BRANCH_COUNTER: c_int = 6;
pub const PERF_HPP_REPORT__BLOCK_MAX_INDEX: c_int = 7;

#[repr(C)]
pub struct block_report {
    pub hist: block_hist,
    pub cycles: u64,
    pub fmts: [block_fmt; PERF_HPP_REPORT__BLOCK_MAX_INDEX as usize],
    pub nr_fmts: c_int,
}

unsafe extern "C" {
    pub fn block_info__delete(bi: *mut block_info);

    pub fn __block_info__cmp(left: *mut hist_entry, right: *mut hist_entry) -> i64;

    pub fn block_info__cmp(
        fmt: *mut perf_hpp_fmt,
        left: *mut hist_entry,
        right: *mut hist_entry,
    ) -> i64;

    pub fn block_info__process_sym(
        he: *mut hist_entry,
        bh: *mut block_hist,
        block_cycles_aggr: *mut u64,
        total_cycles: u64,
        br_cntr_nr: c_uint,
    ) -> c_int;

    pub fn block_info__create_report(
        evlist: *mut evlist,
        total_cycles: u64,
        block_hpps: *mut c_int,
        nr_hpps: c_int,
        nr_reps: *mut c_int,
    ) -> *mut block_report;

    pub fn block_info__free_report(reps: *mut block_report, nr_reps: c_int);

    pub fn report__browse_block_hists(
        bh: *mut block_hist,
        min_percent: c_float,
        evsel: *mut evsel,
        env: *mut perf_env,
    ) -> c_int;

    pub fn block_info__total_cycles_percent(he: *mut hist_entry) -> c_float;
}

