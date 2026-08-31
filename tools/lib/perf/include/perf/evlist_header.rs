/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from lib/perf/include/perf/evlist.h. */
/* C dependencies: <perf/core.h>, <stdbool.h>. */

use std::os::raw::{c_int, c_short};

#[repr(C)]
pub struct perf_evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_evsel {
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
pub struct perf_mmap {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn perf_evlist__add(evlist: *mut perf_evlist, evsel: *mut perf_evsel);
    pub fn perf_evlist__remove(evlist: *mut perf_evlist, evsel: *mut perf_evsel);
    pub fn perf_evlist__new() -> *mut perf_evlist;
    pub fn perf_evlist__delete(evlist: *mut perf_evlist);
    pub fn perf_evlist__next(
        evlist: *mut perf_evlist,
        evsel: *mut perf_evsel,
    ) -> *mut perf_evsel;
    pub fn perf_evlist__open(evlist: *mut perf_evlist) -> c_int;
    pub fn perf_evlist__close(evlist: *mut perf_evlist);
    pub fn perf_evlist__enable(evlist: *mut perf_evlist);
    pub fn perf_evlist__disable(evlist: *mut perf_evlist);

    pub fn perf_evlist__set_maps(
        evlist: *mut perf_evlist,
        cpus: *mut perf_cpu_map,
        threads: *mut perf_thread_map,
    );
    pub fn perf_evlist__poll(evlist: *mut perf_evlist, timeout: c_int) -> c_int;
    pub fn perf_evlist__filter_pollfd(
        evlist: *mut perf_evlist,
        revents_and_mask: c_short,
    ) -> c_int;

    pub fn perf_evlist__mmap(evlist: *mut perf_evlist, pages: c_int) -> c_int;
    pub fn perf_evlist__munmap(evlist: *mut perf_evlist);

    pub fn perf_evlist__next_mmap(
        evlist: *mut perf_evlist,
        map: *mut perf_mmap,
        overwrite: bool,
    ) -> *mut perf_mmap;

    pub fn perf_evlist__set_leader(evlist: *mut perf_evlist);
    pub fn perf_evlist__nr_groups(evlist: *mut perf_evlist) -> c_int;
}

#[macro_export]
macro_rules! perf_evlist__for_each_evsel {
    ($evlist:expr, $pos:ident, $body:block) => {{
        $pos = unsafe { perf_evlist__next($evlist, ::core::ptr::null_mut()) };
        while !$pos.is_null() {
            $body
            $pos = unsafe { perf_evlist__next($evlist, $pos) };
        }
    }};
}

#[macro_export]
macro_rules! perf_evlist__for_each_mmap {
    ($evlist:expr, $pos:ident, $overwrite:expr, $body:block) => {{
        $pos = unsafe { perf_evlist__next_mmap($evlist, ::core::ptr::null_mut(), $overwrite) };
        while !$pos.is_null() {
            $body
            $pos = unsafe { perf_evlist__next_mmap($evlist, $pos, $overwrite) };
        }
    }};
}
