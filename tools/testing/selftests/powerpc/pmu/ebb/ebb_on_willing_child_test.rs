// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

/*
 * Original C dependencies:
 *   <signal.h>, <stdio.h>, <stdlib.h>, <stdbool.h>, <sys/types.h>,
 *   <sys/wait.h>, <unistd.h>, and "ebb.h"
 */

use std::os::raw::{c_char, c_int, c_long};

type pid_t = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_attr {
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
    pub exclude_idle: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event {
    pub attr: event_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebb_stats {
    pub ebb_count: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebb_state_t {
    pub stats: ebb_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pipe {
    pub fds: [c_int; 2],
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

extern "C" {
    static mut ebb_state: ebb_state_t;

    fn pipe(fildes: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;

    fn wait_for_parent(read_pipe: pipe) -> c_int;
    fn ebb_enable_pmc_counting(pmc: c_int);
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn standard_ebb_callee();
    fn ebb_global_enable();
    fn notify_parent(write_pipe: pipe) -> c_int;
    fn core_busy_loop() -> c_int;
    fn ebb_global_disable();
    fn ebb_freeze_pmcs();
    fn dump_ebb_state();
    fn ebb_is_supported() -> bool;
    fn sync_with_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
    fn event_init_named(event: *mut event, config: u64, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open_with_pid(event: *mut event, pid: pid_t) -> c_int;
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn wait_for_child(pid: pid_t) -> c_int;
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

/*
 * Tests we can setup an EBB on our child. The child expects this and enables
 * EBBs, which are then delivered to the child, even though the event is
 * created by the parent.
 */

unsafe extern "C" fn victim_child(read_pipe: pipe, write_pipe: pipe) -> c_int {
    FAIL_IF!(wait_for_parent(read_pipe) != 0);

    /* Setup our EBB handler, before the EBB event is created */
    ebb_enable_pmc_counting(1);
    setup_ebb_handler(standard_ebb_callee);
    ebb_global_enable();

    FAIL_IF!(notify_parent(write_pipe) != 0);

    while ebb_state.stats.ebb_count < 20 {
        FAIL_IF!(core_busy_loop() != 0);
    }

    ebb_global_disable();
    ebb_freeze_pmcs();

    dump_ebb_state();

    FAIL_IF!(ebb_state.stats.ebb_count == 0);

    0
}

/* Tests we can setup an EBB on our child - if it's expecting it */
unsafe extern "C" fn ebb_on_willing_child() -> c_int {
    let mut read_pipe: pipe = pipe { fds: [0; 2] };
    let mut write_pipe: pipe = pipe { fds: [0; 2] };
    let mut event: event = std::mem::zeroed();
    let pid: pid_t;

    SKIP_IF!(!ebb_is_supported());

    FAIL_IF!(pipe(read_pipe.fds.as_mut_ptr()) == -1);
    FAIL_IF!(pipe(write_pipe.fds.as_mut_ptr()) == -1);

    pid = fork();
    if pid == 0 {
        /* NB order of pipes looks reversed */
        exit(victim_child(write_pipe, read_pipe));
    }

    /* Signal the child to setup its EBB handler */
    FAIL_IF!(sync_with_child(read_pipe, write_pipe) != 0);

    /* Child is running now */

    event_init_named(&mut event, 0x1001e, b"cycles\0".as_ptr() as *const c_char);
    event_leader_ebb_init(&mut event);

    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;

    FAIL_IF!(event_open_with_pid(&mut event, pid) != 0);
    FAIL_IF!(ebb_event_enable(&mut event) != 0);

    /* Child show now take EBBs and then exit */
    FAIL_IF!(wait_for_child(pid) != 0);

    event_close(&mut event);

    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            ebb_on_willing_child,
            b"ebb_on_willing_child\0".as_ptr() as *const c_char,
        ) as c_long as i32);
    }
}
