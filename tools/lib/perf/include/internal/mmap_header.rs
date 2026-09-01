/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/* Dependencies from the original header:
 * linux/compiler.h, linux/refcount.h, linux/types.h, stdbool.h,
 * internal/cpumap.h
 */

/* perf sample has 16 bits size limit */
pub const PERF_SAMPLE_MAX_SIZE: i32 = 1 << 16;

pub type libperf_unmap_cb_t = Option<unsafe extern "C" fn(map: *mut perf_mmap)>;

/**
 * struct perf_mmap - perf's ring buffer mmap details
 *
 * @refcnt - e.g. code using PERF_EVENT_IOC_SET_OUTPUT to share this
 */
#[repr(C)]
pub struct perf_mmap {
    pub base: *mut c_void,
    pub mask: i32,
    pub fd: i32,
    pub cpu: perf_cpu,
    pub refcnt: refcount_t,
    pub prev: u64,
    pub start: u64,
    pub end: u64,
    pub overwrite: bool,
    pub flush: u64,
    pub unmap_cb: libperf_unmap_cb_t,
    pub event_copy: *mut c_void,
    pub event_copy_sz: usize,
    pub next: *mut perf_mmap,
}

#[repr(C)]
pub struct perf_mmap_param {
    pub prot: i32,
    pub mask: i32,
}

unsafe extern "C" {
    pub fn perf_mmap__mmap_len(map: *mut perf_mmap) -> usize;

    pub fn perf_mmap__init(
        map: *mut perf_mmap,
        prev: *mut perf_mmap,
        overwrite: bool,
        unmap_cb: libperf_unmap_cb_t,
    );
    pub fn perf_mmap__mmap(
        map: *mut perf_mmap,
        mp: *mut perf_mmap_param,
        fd: i32,
        cpu: perf_cpu,
    ) -> i32;
    pub fn perf_mmap__munmap(map: *mut perf_mmap);
    pub fn perf_mmap__get(map: *mut perf_mmap);
    pub fn perf_mmap__put(map: *mut perf_mmap);

    pub fn perf_mmap__read_head(map: *mut perf_mmap) -> u64;

    pub fn perf_mmap__read_self(
        map: *mut perf_mmap,
        count: *mut perf_counts_values,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
