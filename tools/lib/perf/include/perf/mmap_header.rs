/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <perf/core.h>-dependent declarations.

#[repr(C)]
pub struct perf_mmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn perf_mmap__consume(map: *mut perf_mmap);
    pub fn perf_mmap__read_init(map: *mut perf_mmap) -> ::std::os::raw::c_int;
    pub fn perf_mmap__read_done(map: *mut perf_mmap);
    pub fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
