// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// Translated from testing/selftests/powerpc/pmu/ebb/multi_ebb_procs_test.c.
// C dependencies from <stdbool.h>, <stdio.h>, <stdlib.h>, <signal.h>, and "ebb.h"
// are represented as external declarations below.

use core::ffi::{c_char, c_int, c_ulong};

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
pub struct ebb_stats {
    pub ebb_count: u64,
}

#[repr(C)]
pub struct ebb_state_t {
    pub stats: ebb_stats,
}

#[repr(C)]
pub struct sigaction {
    pub sa_handler: Option<extern "C" fn(c_int)>,
}

type pid_t = c_int;

const SIGINT: c_int = 2;
const SPRN_PMC1: c_int = 771;
const BIND_CPU_ANY: c_int = -1;
const NR_CHILDREN: usize = 4;

unsafe extern "C" {
    static mut ebb_state: ebb_state_t;
    static mut sample_period: u64;
    static standard_ebb_callee: extern "C" fn();

    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn sleep(seconds: c_ulong) -> c_ulong;
    fn kill(pid: pid_t, sig: c_int) -> c_int;

    fn event_init_named(event: *mut event, event_code: u64, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> c_int;
    fn event_close(event: *mut event);
    fn ebb_enable_pmc_counting(pmc: c_int);
    fn setup_ebb_handler(handler: extern "C" fn());
    fn ebb_global_enable();
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn mtspr(sprn: c_int, value: u64);
    fn pmc_sample_period(period: u64) -> u64;
    fn core_busy_loop() -> c_int;
    fn ebb_check_mmcr0() -> c_int;
    fn ebb_global_disable();
    fn ebb_freeze_pmcs();
    fn dump_summary_ebb_state();
    fn ebb_is_supported() -> bool;
    fn bind_to_cpu(cpu: c_int) -> c_int;
    fn wait_for_child(pid: pid_t) -> c_int;
    fn test_harness(test: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
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
 * Test running multiple EBB using processes at once on a single CPU. They
 * should all run happily without interfering with each other.
 */

static mut child_should_exit: bool = false;

extern "C" fn sigint_handler(_signal: c_int) {
    unsafe {
        child_should_exit = true;
    }
}

#[unsafe(no_mangle)]
pub static mut sigint_action: sigaction = sigaction {
    sa_handler: Some(sigint_handler),
};

unsafe fn cycles_child() -> c_int {
    let mut event: event = core::mem::zeroed();

    if sigaction(SIGINT, &raw const sigint_action, core::ptr::null_mut()) != 0 {
        perror(c"sigaction".as_ptr());
        return 1;
    }

    event_init_named(&mut event, 0x1001e, c"cycles".as_ptr());
    event_leader_ebb_init(&mut event);

    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;

    FAIL_IF!(event_open(&mut event) != 0);

    ebb_enable_pmc_counting(1);
    setup_ebb_handler(standard_ebb_callee);
    ebb_global_enable();

    FAIL_IF!(ebb_event_enable(&mut event) != 0);

    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));

    while !child_should_exit {
        FAIL_IF!(core_busy_loop() != 0);
        FAIL_IF!(ebb_check_mmcr0() != 0);
    }

    ebb_global_disable();
    ebb_freeze_pmcs();

    dump_summary_ebb_state();

    event_close(&mut event);

    FAIL_IF!(ebb_state.stats.ebb_count == 0);

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn multi_ebb_procs() -> c_int {
    let mut pids: [pid_t; NR_CHILDREN] = [0; NR_CHILDREN];
    let mut rc: c_int;
    let mut i: usize;

    SKIP_IF!(!ebb_is_supported());

    FAIL_IF!(bind_to_cpu(BIND_CPU_ANY) < 0);

    i = 0;
    while i < NR_CHILDREN {
        pids[i] = fork();
        if pids[i] == 0 {
            exit(cycles_child());
        }
        i += 1;
    }

    /* Have them all run for "a while" */
    sleep(10);

    rc = 0;
    i = 0;
    while i < NR_CHILDREN {
        /* Tell them to stop */
        kill(pids[i], SIGINT);
        /* And wait */
        rc |= wait_for_child(pids[i]);
        i += 1;
    }

    rc
}

fn main() -> c_int {
    unsafe { test_harness(multi_ebb_procs, c"multi_ebb_procs".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
