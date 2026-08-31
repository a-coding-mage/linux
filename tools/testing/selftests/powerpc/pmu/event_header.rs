/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2013, Michael Ellerman, IBM Corp.
 */

// C dependencies: <unistd.h>, <linux/perf_event.h>, and "utils.h".
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct event_result {
    pub value: u64,
    pub running: u64,
    pub enabled: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub name: *mut c_char,
    pub fd: c_int,
    /* This must match the read_format we use */
    pub result: event_result,
    /*
     * mmap buffer used while recording sample.
     * Accessed as "struct perf_event_mmap_page"
     */
    pub mmap_buffer: *mut c_void,
}

unsafe extern "C" {
    pub fn event_init(e: *mut event, config: u64);
    pub fn event_init_named(e: *mut event, config: u64, name: *mut c_char);
    pub fn event_init_opts(e: *mut event, config: u64, type_: c_int, name: *mut c_char);
    pub fn event_init_sampling(e: *mut event, config: u64);
    pub fn event_open_with_options(
        e: *mut event,
        pid: libc::pid_t,
        cpu: c_int,
        group_fd: c_int,
    ) -> c_int;
    pub fn event_open_with_group(e: *mut event, group_fd: c_int) -> c_int;
    pub fn event_open_with_pid(e: *mut event, pid: libc::pid_t) -> c_int;
    pub fn event_open_with_cpu(e: *mut event, cpu: c_int) -> c_int;
    pub fn event_open(e: *mut event) -> c_int;
    pub fn event_close(e: *mut event);
    pub fn event_enable(e: *mut event) -> c_int;
    pub fn event_disable(e: *mut event) -> c_int;
    pub fn event_reset(e: *mut event) -> c_int;
    pub fn event_read(e: *mut event) -> c_int;
    pub fn event_report_justified(e: *mut event, name_width: c_int, result_width: c_int);
    pub fn event_report(e: *mut event);
}
