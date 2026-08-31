// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

/*
 * C dependencies removed from executable Rust:
 * <signal.h>, <stdio.h>, <stdlib.h>, <stdbool.h>, <sys/types.h>,
 * <sys/wait.h>, <unistd.h>, and "ebb.h".
 */

use core::ffi::{c_char, c_int};

type pid_t = c_int;

const SIGTERM: c_int = 15;

#[repr(C)]
pub struct perf_event_attr {
    pub pinned: u64,
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
    pub exclude_idle: u64,
}

#[repr(C)]
pub struct event_result {
    pub value: u64,
    pub enabled: u64,
    pub running: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub result: event_result,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pipe {
    pub fds: [c_int; 2],
}

unsafe extern "C" {
    fn event_init_named(event: *mut event, code: u64, name: *const c_char);
    fn event_open_with_pid(event: *mut event, pid: pid_t) -> c_int;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn event_read(event: *mut event) -> c_int;
    fn event_report(event: *mut event);

    fn ebb_is_supported() -> bool;
    fn ebb_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
    fn sync_with_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
    fn kill_child_and_wait(pid: pid_t);
    fn wait_for_child(pid: pid_t) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    #[link_name = "pipe"]
    fn libc_pipe(fds: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
}

macro_rules! FAIL_IF {
    ($condition:expr) => {
        if $condition {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($condition:expr) => {
        if $condition {
            return 4;
        }
    };
}

unsafe extern "C" fn setup_child_event(event: *mut event, child_pid: pid_t) -> c_int {
    unsafe {
        event_init_named(event, 0x400FA, c"PM_RUN_INST_CMPL".as_ptr());

        (*event).attr.pinned = 1;

        (*event).attr.exclude_kernel = 1;
        (*event).attr.exclude_hv = 1;
        (*event).attr.exclude_idle = 1;

        FAIL_IF!(event_open_with_pid(event, child_pid) != 0);
        FAIL_IF!(event_enable(event) != 0);

        return 0;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_event_pinned_vs_ebb() -> c_int {
    unsafe {
        let mut read_pipe: pipe = pipe { fds: [0; 2] };
        let mut write_pipe: pipe = pipe { fds: [0; 2] };
        let mut event: event = core::mem::zeroed();
        let pid: pid_t;
        let rc: c_int;

        SKIP_IF!(!ebb_is_supported());

        FAIL_IF!(libc_pipe(read_pipe.fds.as_mut_ptr()) == -1);
        FAIL_IF!(libc_pipe(write_pipe.fds.as_mut_ptr()) == -1);

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
            FAIL_IF!(sync_with_child(read_pipe, write_pipe) != 0);
        }

        /* We expect it to fail to read the event */
        FAIL_IF!(wait_for_child(pid) != 2);
        FAIL_IF!(event_disable(&mut event) != 0);
        FAIL_IF!(event_read(&mut event) != 0);

        event_report(&mut event);

        FAIL_IF!(event.result.value == 0);
        /*
         * For reasons I don't understand enabled is usually just slightly
         * lower than running. Would be good to confirm why.
         */
        FAIL_IF!(event.result.enabled == 0);
        FAIL_IF!(event.result.running == 0);

        return 0;
    }
}

fn main() -> c_int {
    unsafe { test_harness(task_event_pinned_vs_ebb, c"task_event_pinned_vs_ebb".as_ptr()) }
}
