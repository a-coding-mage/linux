// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

/*
 * C dependencies removed from executable Rust:
 * signal.h, stdio.h, stdlib.h, stdbool.h, sys/types.h, sys/wait.h, unistd.h,
 * and "ebb.h".
 */

use core::ffi::{c_char, c_int};

extern "C" {
    fn wait_for_parent(read_pipe: pipe) -> c_int;
    fn notify_parent(write_pipe: pipe) -> c_int;
    fn write_pmc1();
    fn ebb_is_supported() -> bool;
    fn pipe(fildes: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn sync_with_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
    fn event_init_named(event: *mut event, event_code: u64, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open_with_pid(event: *mut event, pid: pid_t) -> c_int;
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn wait_for_child(pid: pid_t) -> c_int;
    fn event_close(event: *mut event);
    fn test_harness(test: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

/*
 * Types and macros supplied by "ebb.h" in the original C source are expected to
 * be supplied by the surrounding Rust translation.
 */
type pid_t = c_int;

#[repr(C)]
pub union pipe {
    pub fds: [c_int; 2],
}

#[repr(C)]
pub struct perf_event_attr {
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
    pub exclude_idle: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 0;
        }
    };
}

/*
 * Tests we can setup an EBB on our child. Nothing interesting happens, because
 * even though the event is enabled and running the child hasn't enabled the
 * actual delivery of the EBBs.
 */

unsafe extern "C" fn victim_child(read_pipe: pipe, write_pipe: pipe) -> c_int {
    let mut i: c_int;

    FAIL_IF!(wait_for_parent(read_pipe) != 0);
    FAIL_IF!(notify_parent(write_pipe) != 0);

    /* Parent creates EBB event */

    FAIL_IF!(wait_for_parent(read_pipe) != 0);
    FAIL_IF!(notify_parent(write_pipe) != 0);

    /* Check the EBB is enabled by writing PMC1 */
    write_pmc1();

    /* EBB event is enabled here */
    i = 0;
    while i < 1000000 {
        i += 1;
    }

    0
}

pub unsafe extern "C" fn ebb_on_child() -> c_int {
    let mut read_pipe: pipe = core::mem::zeroed();
    let mut write_pipe: pipe = core::mem::zeroed();
    let mut event: event = core::mem::zeroed();
    let pid: pid_t;

    SKIP_IF!(!ebb_is_supported());

    FAIL_IF!(pipe(read_pipe.fds.as_mut_ptr()) == -1);
    FAIL_IF!(pipe(write_pipe.fds.as_mut_ptr()) == -1);

    pid = fork();
    if pid == 0 {
        /* NB order of pipes looks reversed */
        exit(victim_child(write_pipe, read_pipe));
    }

    FAIL_IF!(sync_with_child(read_pipe, write_pipe) != 0);

    /* Child is running now */

    event_init_named(&mut event, 0x1001e, b"cycles\0".as_ptr() as *const c_char);
    event_leader_ebb_init(&mut event);

    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;

    FAIL_IF!(event_open_with_pid(&mut event, pid) != 0);
    FAIL_IF!(ebb_event_enable(&mut event) != 0);

    FAIL_IF!(sync_with_child(read_pipe, write_pipe) != 0);

    /* Child should just exit happily */
    FAIL_IF!(wait_for_child(pid) != 0);

    event_close(&mut event);

    0
}

pub extern "C" fn main() -> c_int {
    unsafe { test_harness(ebb_on_child, b"ebb_on_child\0".as_ptr() as *const c_char) }
}
