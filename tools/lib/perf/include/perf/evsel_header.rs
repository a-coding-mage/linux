/* SPDX-License-Identifier: GPL-2.0 */

// Translated from lib/perf/include/perf/evsel.h.
// C includes removed: <stdint.h>, <perf/core.h>, <stdbool.h>, <linux/types.h>.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct perf_evsel {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _unused: [u8; 0],
}

// From linux/types.h.
pub type __s8 = i8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_counts_values__bindgen_ty_1 {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
    pub id: u64,
    pub lost: u64,
}

#[repr(C)]
pub union perf_counts_values {
    pub __bindgen_anon_1: perf_counts_values__bindgen_ty_1,
    pub values: [u64; 5],
}

unsafe extern "C" {
    pub fn perf_evsel__new(attr: *mut perf_event_attr) -> *mut perf_evsel;
    pub fn perf_evsel__delete(evsel: *mut perf_evsel);
    pub fn perf_evsel__open(
        evsel: *mut perf_evsel,
        cpus: *mut perf_cpu_map,
        threads: *mut perf_thread_map,
    ) -> c_int;
    pub fn perf_evsel__close(evsel: *mut perf_evsel);
    pub fn perf_evsel__close_cpu(evsel: *mut perf_evsel, cpu_map_idx: c_int);
    pub fn perf_evsel__mmap(evsel: *mut perf_evsel, pages: c_int) -> c_int;
    pub fn perf_evsel__munmap(evsel: *mut perf_evsel);
    pub fn perf_evsel__mmap_base(
        evsel: *mut perf_evsel,
        cpu_map_idx: c_int,
        thread: c_int,
    ) -> *mut c_void;
    pub fn perf_evsel__read(
        evsel: *mut perf_evsel,
        cpu_map_idx: c_int,
        thread: c_int,
        count: *mut perf_counts_values,
    ) -> c_int;
    pub fn perf_evsel__enable(evsel: *mut perf_evsel) -> c_int;
    pub fn perf_evsel__enable_cpu(evsel: *mut perf_evsel, cpu_map_idx: c_int) -> c_int;
    pub fn perf_evsel__enable_thread(evsel: *mut perf_evsel, thread: c_int) -> c_int;
    pub fn perf_evsel__disable(evsel: *mut perf_evsel) -> c_int;
    pub fn perf_evsel__disable_cpu(evsel: *mut perf_evsel, cpu_map_idx: c_int) -> c_int;
    pub fn perf_evsel__cpus(evsel: *mut perf_evsel) -> *mut perf_cpu_map;
    pub fn perf_evsel__threads(evsel: *mut perf_evsel) -> *mut perf_thread_map;
    pub fn perf_evsel__attr(evsel: *mut perf_evsel) -> *mut perf_event_attr;
    pub fn perf_counts_values__scale(
        count: *mut perf_counts_values,
        scale: bool,
        pscaled: *mut __s8,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
