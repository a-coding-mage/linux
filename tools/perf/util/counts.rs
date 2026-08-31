// SPDX-License-Identifier: GPL-2.0
// C dependencies removed from executable Rust:
// errno.h, stdlib.h, string.h, evsel.h, counts.h, perf/threadmap.h,
// linux/zalloc.h

use core::ffi::{c_int, c_void};
use core::mem::size_of;
use core::ptr;

pub const ENOMEM: c_int = 12;

#[repr(C)]
pub struct xyarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_counts_values {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_counts {
    pub values: *mut xyarray,
    pub loaded: *mut xyarray,
}

#[repr(C)]
pub struct evsel_core {
    pub threads: *mut perf_thread_map,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub counts: *mut perf_counts,
}

extern "C" {
    fn zalloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn xyarray__new(xlen: c_int, ylen: c_int, entry_size: usize) -> *mut xyarray;
    fn xyarray__delete(xy: *mut xyarray);
    fn xyarray__reset(xy: *mut xyarray);

    fn evsel__cpus(evsel: *mut evsel) -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_int;
    fn perf_thread_map__nr(threads: *const perf_thread_map) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn perf_counts__new(ncpus: c_int, nthreads: c_int) -> *mut perf_counts {
    let counts = zalloc(size_of::<perf_counts>()) as *mut perf_counts;

    if !counts.is_null() {
        let mut values: *mut xyarray;

        values = xyarray__new(ncpus, nthreads, size_of::<perf_counts_values>());
        if values.is_null() {
            free(counts as *mut c_void);
            return ptr::null_mut();
        }

        (*counts).values = values;

        values = xyarray__new(ncpus, nthreads, size_of::<bool>());
        if values.is_null() {
            xyarray__delete((*counts).values);
            free(counts as *mut c_void);
            return ptr::null_mut();
        }

        (*counts).loaded = values;
    }

    counts
}

#[no_mangle]
pub unsafe extern "C" fn perf_counts__delete(counts: *mut perf_counts) {
    if !counts.is_null() {
        xyarray__delete((*counts).loaded);
        xyarray__delete((*counts).values);
        free(counts as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_counts__reset(counts: *mut perf_counts) {
    xyarray__reset((*counts).loaded);
    xyarray__reset((*counts).values);
}

#[no_mangle]
pub unsafe extern "C" fn evsel__reset_counts(evsel: *mut evsel) {
    perf_counts__reset((*evsel).counts);
}

#[no_mangle]
pub unsafe extern "C" fn evsel__alloc_counts(evsel: *mut evsel) -> c_int {
    let cpus = evsel__cpus(evsel);
    let nthreads = perf_thread_map__nr((*evsel).core.threads);

    (*evsel).counts = perf_counts__new(perf_cpu_map__nr(cpus), nthreads);
    if !(*evsel).counts.is_null() {
        0
    } else {
        -ENOMEM
    }
}

#[no_mangle]
pub unsafe extern "C" fn evsel__free_counts(evsel: *mut evsel) {
    perf_counts__delete((*evsel).counts);
    (*evsel).counts = ptr::null_mut();
}
