// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C dependencies: signal.h, stdio.h, stdlib.h, stdbool.h, sys/types.h,
// sys/wait.h, unistd.h, and "ebb.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int};

type pid_t = c_int;

const BIND_CPU_ANY: c_int = -1;

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
    fn event_init_named(event: *mut event, event_code: u64, name: *const c_char);
    fn require_paranoia_below(paranoia: c_int) -> c_int;
    fn event_open_with_cpu(event: *mut event, cpu: c_int) -> c_int;
    fn event_enable(event: *mut event) -> c_int;
    fn ebb_is_supported() -> c_int;
    fn bind_to_cpu(cpu: c_int) -> c_int;
    fn pipe(fds: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn ebb_child(write_pipe: pipe, read_pipe: pipe) -> c_int;
    fn kill_child_and_wait(pid: pid_t);
    fn sync_with_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
    fn wait_for_child(pid: pid_t) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn event_read(event: *mut event) -> c_int;
    fn event_report(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

/*
 * Tests a pinned cpu event vs an EBB - in that order. The pinned cpu event
 * should remain and the EBB event should fail to enable.
 */

unsafe fn setup_cpu_event(event: *mut event, cpu: c_int) -> c_int {
    unsafe {
        event_init_named(event, 0x400FA, c"PM_RUN_INST_CMPL".as_ptr());

        (*event).attr.pinned = 1;

        (*event).attr.exclude_kernel = 1;
        (*event).attr.exclude_hv = 1;
        (*event).attr.exclude_idle = 1;

        SKIP_IF!(require_paranoia_below(1));
        FAIL_IF!(event_open_with_cpu(event, cpu));
        FAIL_IF!(event_enable(event));

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpu_event_pinned_vs_ebb() -> c_int {
    unsafe {
        let mut read_pipe: pipe = core::mem::zeroed();
        let mut write_pipe: pipe = core::mem::zeroed();
        let mut event: event = core::mem::zeroed();
        let cpu: c_int;
        let rc: c_int;
        let pid: pid_t;

        SKIP_IF!(ebb_is_supported() == 0);

        cpu = bind_to_cpu(BIND_CPU_ANY);
        FAIL_IF!(cpu < 0);

        FAIL_IF!(pipe(read_pipe.fds.as_mut_ptr()) == -1);
        FAIL_IF!(pipe(write_pipe.fds.as_mut_ptr()) == -1);

        pid = fork();
        if pid == 0 {
            /* NB order of pipes looks reversed */
            exit(ebb_child(write_pipe, read_pipe));
        }

        /* We setup the cpu event first */
        rc = setup_cpu_event(&mut event, cpu);
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

        /* We expect it to fail to read the event */
        FAIL_IF!(wait_for_child(pid) != 2);

        FAIL_IF!(event_disable(&mut event));
        FAIL_IF!(event_read(&mut event));

        event_report(&mut event);

        /* The cpu event should have run */
        FAIL_IF!(event.result.value == 0);
        FAIL_IF!(event.result.enabled != event.result.running);

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    unsafe { test_harness(cpu_event_pinned_vs_ebb, c"cpu_event_pinned_vs_ebb".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
