/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/thread_map.h. */
/* C includes:
 * - <sys/types.h> for pid_t
 * - <stdio.h> for FILE
 * - <perf/threadmap.h> for struct perf_thread_map
 */

use core::ffi::{c_char, c_int};

pub type pid_t = i32;
pub type size_t = usize;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_record_thread_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn thread_map__new_dummy() -> *mut perf_thread_map;
    pub fn thread_map__new_by_pid(pid: pid_t) -> *mut perf_thread_map;
    pub fn thread_map__new_by_tid(tid: pid_t) -> *mut perf_thread_map;
    pub fn thread_map__new(pid: pid_t, tid: pid_t) -> *mut perf_thread_map;
    pub fn thread_map__new_event(event: *mut perf_record_thread_map) -> *mut perf_thread_map;

    pub fn thread_map__new_str(
        pid: *const c_char,
        tid: *const c_char,
        all_threads: bool,
    ) -> *mut perf_thread_map;

    pub fn thread_map__new_by_tid_str(tid_str: *const c_char) -> *mut perf_thread_map;

    pub fn thread_map__fprintf(threads: *mut perf_thread_map, fp: *mut FILE) -> size_t;

    pub fn thread_map__read_comms(threads: *mut perf_thread_map);
    pub fn thread_map__has(threads: *mut perf_thread_map, pid: pid_t) -> bool;
    pub fn thread_map__remove(threads: *mut perf_thread_map, idx: c_int) -> c_int;
}
