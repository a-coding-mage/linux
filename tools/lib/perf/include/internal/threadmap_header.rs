/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies from the original C header:
// #include <linux/refcount.h>
// #include <sys/types.h>
// #include <unistd.h>

#[repr(C)]
pub struct thread_map_data {
    pub pid: pid_t,
    pub comm: *mut ::core::ffi::c_char,
}

#[repr(C)]
pub struct perf_thread_map {
    pub refcnt: refcount_t,
    pub nr: ::core::ffi::c_int,
    pub err_thread: ::core::ffi::c_int,
    // Flexible array member from C: struct thread_map_data map[];
    pub map: [thread_map_data; 0],
}

unsafe extern "C" {
    pub fn perf_thread_map__realloc(
        map: *mut perf_thread_map,
        nr: ::core::ffi::c_int,
    ) -> *mut perf_thread_map;
}
