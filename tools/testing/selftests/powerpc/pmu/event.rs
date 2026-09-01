// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2013, Michael Ellerman, IBM Corp.
 */

// C source included:
// #define _GNU_SOURCE
// #include <unistd.h>
// #include <sys/syscall.h>
// #include <string.h>
// #include <stdio.h>
// #include <stdbool.h>
// #include <sys/ioctl.h>
// #include "event.h"

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type pid_t = c_int;
type ssize_t = isize;

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn syscall(number: c_long, ...) -> c_long;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn perror(s: *const c_char);
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
}

unsafe extern "C" {
    static __NR_perf_event_open: c_long;

    static PERF_FORMAT_TOTAL_TIME_ENABLED: u64;
    static PERF_FORMAT_TOTAL_TIME_RUNNING: u64;
    static PERF_SAMPLE_REGS_INTR: u64;
    static PERF_TYPE_RAW: c_int;
    static PERF_EVENT_IOC_ENABLE: c_ulong;
    static PERF_EVENT_IOC_DISABLE: c_ulong;
    static PERF_EVENT_IOC_RESET: c_ulong;
}

// Provided by the translated equivalent of "event.h".
// The struct layouts and fields must match the C header.
use crate::event;
use crate::perf_event_attr;

#[no_mangle]
pub unsafe extern "C" fn perf_event_open(
    attr: *mut perf_event_attr,
    pid: pid_t,
    cpu: c_int,
    group_fd: c_int,
    flags: c_ulong,
) -> c_int {
    unsafe { syscall(__NR_perf_event_open, attr, pid, cpu, group_fd, flags) as c_int }
}

unsafe fn __event_init_opts(
    e: *mut event,
    config: u64,
    type_: c_int,
    name: *mut c_char,
    sampling: bool,
) {
    unsafe {
        memset(e as *mut c_void, 0, size_of::<event>());

        (*e).name = name;

        (*e).attr.type_ = type_;
        (*e).attr.config = config;
        (*e).attr.size = size_of_val(&(*e).attr) as _;
        /* This has to match the structure layout in the header */
        (*e).attr.read_format = PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING;
        if sampling {
            (*e).attr.sample_period = 1000;
            (*e).attr.sample_type = PERF_SAMPLE_REGS_INTR;
            (*e).attr.disabled = 1;
        }
    }
}

#[inline]
unsafe fn size_of_val<T>(_: *const T) -> usize {
    size_of::<T>()
}

#[no_mangle]
pub unsafe extern "C" fn event_init_opts(
    e: *mut event,
    config: u64,
    type_: c_int,
    name: *mut c_char,
) {
    unsafe {
        __event_init_opts(e, config, type_, name, false);
    }
}

#[no_mangle]
pub unsafe extern "C" fn event_init_named(e: *mut event, config: u64, name: *mut c_char) {
    unsafe {
        event_init_opts(e, config, PERF_TYPE_RAW, name);
    }
}

#[no_mangle]
pub unsafe extern "C" fn event_init(e: *mut event, config: u64) {
    unsafe {
        event_init_opts(e, config, PERF_TYPE_RAW, c"event".as_ptr() as *mut c_char);
    }
}

#[no_mangle]
pub unsafe extern "C" fn event_init_sampling(e: *mut event, config: u64) {
    unsafe {
        __event_init_opts(e, config, PERF_TYPE_RAW, c"event".as_ptr() as *mut c_char, true);
    }
}

const PERF_CURRENT_PID: pid_t = 0;
const PERF_NO_PID: pid_t = -1;
const PERF_NO_CPU: c_int = -1;
const PERF_NO_GROUP: c_int = -1;

#[no_mangle]
pub unsafe extern "C" fn event_open_with_options(
    e: *mut event,
    pid: pid_t,
    cpu: c_int,
    group_fd: c_int,
) -> c_int {
    unsafe {
        (*e).fd = perf_event_open(ptr::addr_of_mut!((*e).attr), pid, cpu, group_fd, 0);
        if (*e).fd == -1 {
            perror(c"perf_event_open".as_ptr());
            return -1;
        }

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn event_open_with_group(e: *mut event, group_fd: c_int) -> c_int {
    unsafe { event_open_with_options(e, PERF_CURRENT_PID, PERF_NO_CPU, group_fd) }
}

#[no_mangle]
pub unsafe extern "C" fn event_open_with_pid(e: *mut event, pid: pid_t) -> c_int {
    unsafe { event_open_with_options(e, pid, PERF_NO_CPU, PERF_NO_GROUP) }
}

#[no_mangle]
pub unsafe extern "C" fn event_open_with_cpu(e: *mut event, cpu: c_int) -> c_int {
    unsafe { event_open_with_options(e, PERF_NO_PID, cpu, PERF_NO_GROUP) }
}

#[no_mangle]
pub unsafe extern "C" fn event_open(e: *mut event) -> c_int {
    unsafe { event_open_with_options(e, PERF_CURRENT_PID, PERF_NO_CPU, PERF_NO_GROUP) }
}

#[no_mangle]
pub unsafe extern "C" fn event_close(e: *mut event) {
    unsafe {
        close((*e).fd);
    }
}

#[no_mangle]
pub unsafe extern "C" fn event_enable(e: *mut event) -> c_int {
    unsafe { ioctl((*e).fd, PERF_EVENT_IOC_ENABLE) }
}

#[no_mangle]
pub unsafe extern "C" fn event_disable(e: *mut event) -> c_int {
    unsafe { ioctl((*e).fd, PERF_EVENT_IOC_DISABLE) }
}

#[no_mangle]
pub unsafe extern "C" fn event_reset(e: *mut event) -> c_int {
    unsafe { ioctl((*e).fd, PERF_EVENT_IOC_RESET) }
}

#[no_mangle]
pub unsafe extern "C" fn event_read(e: *mut event) -> c_int {
    let rc: c_int;

    unsafe {
        rc = read(
            (*e).fd,
            ptr::addr_of_mut!((*e).result) as *mut c_void,
            size_of_val(ptr::addr_of!((*e).result)),
        ) as c_int;
        if rc != size_of_val(ptr::addr_of!((*e).result)) as c_int {
            fprintf(stderr, c"read error on event %p!\n".as_ptr(), e);
            return -1;
        }

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn event_report_justified(
    e: *mut event,
    name_width: c_int,
    result_width: c_int,
) {
    unsafe {
        printf(
            c"%*s: result %*llu ".as_ptr(),
            name_width,
            (*e).name,
            result_width,
            (*e).result.value,
        );

        if (*e).result.running == (*e).result.enabled {
            printf(c"running/enabled %llu\n".as_ptr(), (*e).result.running);
        } else {
            printf(
                c"running %llu enabled %llu\n".as_ptr(),
                (*e).result.running,
                (*e).result.enabled,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn event_report(e: *mut event) {
    unsafe {
        event_report_justified(e, 0, 0);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
