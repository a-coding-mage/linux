// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

/*
 * Translated from C.  Declarations supplied by signal.h, stdio.h, stdlib.h,
 * stdbool.h, sys/types.h, sys/wait.h, unistd.h, setjmp.h, and "ebb.h" are
 * represented here as external dependencies.
 */

use core::ffi::{c_char, c_int, c_ulong};

type pid_t = c_int;

#[repr(C)]
pub struct event {
    /*
     * The real layout is supplied by "ebb.h".  This file only uses the object
     * through pointers, matching the source-level dependency on that header.
     */
    _private: [u8; 0],
}

unsafe extern "C" {
    static SPRN_BESCR: c_int;
    static SPRN_EBBHR: c_int;
    static SPRN_EBBRR: c_int;
    static SPRN_MMCR0: c_int;
    static SPRN_PMC1: c_int;
    static MMCR0_FC: c_ulong;
    static sample_period: c_ulong;

    static standard_ebb_callee: unsafe extern "C" fn();

    fn mfspr(spr: c_int) -> c_ulong;
    fn mtspr(spr: c_int, val: c_ulong);
    fn catch_sigill(func: unsafe extern "C" fn()) -> c_int;
    fn write_pmc1();
    fn event_read(event: *mut event) -> c_int;
    fn ebb_is_supported() -> bool;
    fn event_init_named(event: *mut event, event_code: c_ulong, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> c_int;
    fn ebb_enable_pmc_counting(pmc: c_int);
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn ebb_global_enable();
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn pmc_sample_period(period: c_ulong) -> c_ulong;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn wait_for_child(pid: pid_t) -> c_int;
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
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
            /*
             * The exact skip return value is defined by the selftest harness
             * headers outside this isolated source file.
             */
            return 4;
        }
    };
}

/*
 * Test that a fork clears the PMU state of the child. eg. BESCR/EBBHR/EBBRR
 * are cleared, and MMCR0_PMCC is reset, preventing the child from accessing
 * the PMU.
 */

static mut event: event = event { _private: [] };

unsafe extern "C" fn child() -> c_int {
    /* Even though we have EBE=0 we can still see the EBB regs */
    FAIL_IF!(mfspr(SPRN_BESCR) != 0);
    FAIL_IF!(mfspr(SPRN_EBBHR) != 0);
    FAIL_IF!(mfspr(SPRN_EBBRR) != 0);

    FAIL_IF!(catch_sigill(write_pmc1) != 0);

    /* We can still read from the event, though it is on our parent */
    FAIL_IF!(event_read(&raw mut event) != 0);

    0
}

/* Tests that fork clears EBB state */
#[no_mangle]
pub unsafe extern "C" fn fork_cleanup() -> c_int {
    let mut pid: pid_t;

    SKIP_IF!(!ebb_is_supported());

    event_init_named(&raw mut event, 0x1001e, c"cycles".as_ptr());
    event_leader_ebb_init(&raw mut event);

    FAIL_IF!(event_open(&raw mut event) != 0);

    ebb_enable_pmc_counting(1);
    setup_ebb_handler(standard_ebb_callee);
    ebb_global_enable();

    FAIL_IF!(ebb_event_enable(&raw mut event) != 0);

    mtspr(SPRN_MMCR0, MMCR0_FC);
    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));

    /* Don't need to actually take any EBBs */

    pid = fork();
    if pid == 0 {
        exit(child());
    }

    /* Child does the actual testing */
    FAIL_IF!(wait_for_child(pid) != 0);

    /* After fork */
    event_close(&raw mut event);

    0
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    test_harness(fork_cleanup, c"fork_cleanup".as_ptr())
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
