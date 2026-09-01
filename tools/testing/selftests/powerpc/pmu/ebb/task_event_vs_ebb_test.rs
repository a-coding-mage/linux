// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

/*
 * C dependencies translated as external items:
 *   <signal.h>, <stdio.h>, <stdlib.h>, <stdbool.h>, <sys/types.h>,
 *   <sys/wait.h>, <unistd.h>, and "ebb.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int};

type pid_t = c_int;

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

#[repr(C)]
pub union pipe {
    pub fds: [c_int; 2],
}

impl Copy for pipe {}

impl Clone for pipe {
    fn clone(&self) -> Self {
        *self
    }
}

trait CTruthy {
    fn c_truthy(self) -> bool;
}

impl CTruthy for bool {
    fn c_truthy(self) -> bool {
        self
    }
}

impl CTruthy for c_int {
    fn c_truthy(self) -> bool {
        self != 0
    }
}

unsafe extern "C" {
    fn event_init_named(event: *mut event, config: u64, name: *const c_char);
    fn event_open_with_pid(event: *mut event, pid: pid_t) -> c_int;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn event_read(event: *mut event) -> c_int;
    fn event_report(event: *mut event);

    fn ebb_is_supported() -> c_int;
    fn ebb_child(write_pipe: pipe, read_pipe: pipe) -> c_int;
    fn sync_with_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
    fn kill_child_and_wait(pid: pid_t);
    fn wait_for_child(pid: pid_t) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    fn pipe(fds: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if CTruthy::c_truthy($cond) {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if CTruthy::c_truthy($cond) {
            return 0;
        }
    };
}

/*
 * Tests a per-task event vs an EBB - in that order. The EBB should push the
 * per-task event off the PMU.
 */
unsafe fn setup_child_event(event: *mut event, child_pid: pid_t) -> c_int {
    unsafe {
        event_init_named(event, 0x400FA, c"PM_RUN_INST_CMPL".as_ptr());

        (*event).attr.exclude_kernel = 1;
        (*event).attr.exclude_hv = 1;
        (*event).attr.exclude_idle = 1;

        FAIL_IF!(event_open_with_pid(event, child_pid));
        FAIL_IF!(event_enable(event));

        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_event_vs_ebb() -> c_int {
    unsafe {
        let mut read_pipe: pipe = core::mem::zeroed();
        let mut write_pipe: pipe = core::mem::zeroed();
        let mut event: event = core::mem::zeroed();
        let pid: pid_t;
        let rc: c_int;

        SKIP_IF!(ebb_is_supported() == 0);

        FAIL_IF!(pipe(read_pipe.fds.as_mut_ptr()) == -1);
        FAIL_IF!(pipe(write_pipe.fds.as_mut_ptr()) == -1);

        pid = fork();
        if pid == 0 {
            /* NB order of pipes looks reversed */
            exit(ebb_child(write_pipe, read_pipe));
        }

        /* We setup the task event first */
        rc = setup_child_event(&mut event, pid);
        if rc != 0 {
            kill_child_and_wait(pid);
            return rc;
        }

        /* Signal the child to install its EBB event and wait */
        if sync_with_child(read_pipe, write_pipe) != 0 {
            /* If it fails, wait for it to exit */
        } else {
            /* Signal the child to run */
            FAIL_IF!(sync_with_child(read_pipe, write_pipe));
        }

        /* The EBB event should push the task event off so the child should succeed */
        FAIL_IF!(wait_for_child(pid));
        FAIL_IF!(event_disable(&mut event));
        FAIL_IF!(event_read(&mut event));

        event_report(&mut event);

        /* The task event may have run, or not so we can't assert anything about it */

        0
    }
}

fn main() {
    unsafe {
        std::process::exit(test_harness(task_event_vs_ebb, c"task_event_vs_ebb".as_ptr()));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
