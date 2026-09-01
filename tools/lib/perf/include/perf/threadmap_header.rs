/* SPDX-License-Identifier: GPL-2.0 */

// Translated from lib/perf/include/perf/threadmap.h.
// C dependencies: <perf/core.h> for LIBPERF_API visibility and <sys/types.h> for pid_t.

pub type pid_t = libc::pid_t;

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn perf_thread_map__new_dummy() -> *mut perf_thread_map;
    pub fn perf_thread_map__new_array(nr_threads: ::std::os::raw::c_int, array: *mut pid_t) -> *mut perf_thread_map;

    pub fn perf_thread_map__set_pid(map: *mut perf_thread_map, idx: ::std::os::raw::c_int, pid: pid_t);
    pub fn perf_thread_map__comm(map: *mut perf_thread_map, idx: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    pub fn perf_thread_map__nr(threads: *mut perf_thread_map) -> ::std::os::raw::c_int;
    pub fn perf_thread_map__pid(map: *mut perf_thread_map, idx: ::std::os::raw::c_int) -> pid_t;
    pub fn perf_thread_map__idx(map: *mut perf_thread_map, pid: pid_t) -> ::std::os::raw::c_int;

    pub fn perf_thread_map__get(map: *mut perf_thread_map) -> *mut perf_thread_map;
    pub fn perf_thread_map__put(map: *mut perf_thread_map);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
