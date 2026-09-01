/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies from the original header:
 * <linux/types.h>, <internal/xyarray.h>, <perf/evsel.h>, <stdbool.h>
 */

pub type s8 = i8;

#[repr(C)]
pub struct xyarray {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_counts_values {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_counts {
    pub scaled: s8,
    pub values: *mut xyarray,
    pub loaded: *mut xyarray,
}

extern "C" {
    pub fn xyarray__entry(xy: *mut xyarray, x: i32, y: i32) -> *mut ::std::os::raw::c_void;
}

#[inline]
pub unsafe fn perf_counts(
    counts: *mut perf_counts,
    cpu_map_idx: i32,
    thread: i32,
) -> *mut perf_counts_values {
    unsafe { xyarray__entry((*counts).values, cpu_map_idx, thread) as *mut perf_counts_values }
}

#[inline]
pub unsafe fn perf_counts__is_loaded(
    counts: *mut perf_counts,
    cpu_map_idx: i32,
    thread: i32,
) -> bool {
    unsafe { *(xyarray__entry((*counts).loaded, cpu_map_idx, thread) as *mut bool) }
}

#[inline]
pub unsafe fn perf_counts__set_loaded(
    counts: *mut perf_counts,
    cpu_map_idx: i32,
    thread: i32,
    loaded: bool,
) {
    unsafe {
        *(xyarray__entry((*counts).loaded, cpu_map_idx, thread) as *mut bool) = loaded;
    }
}

extern "C" {
    pub fn perf_counts__new(ncpus: i32, nthreads: i32) -> *mut perf_counts;
    pub fn perf_counts__delete(counts: *mut perf_counts);
    pub fn perf_counts__reset(counts: *mut perf_counts);

    pub fn evsel__reset_counts(evsel: *mut evsel);
    pub fn evsel__alloc_counts(evsel: *mut evsel) -> i32;
    pub fn evsel__free_counts(evsel: *mut evsel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
