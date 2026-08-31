// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

// C dependencies: errno.h, pthread.h, stdint.h, stdio.h, stdlib.h, time.h,
// unistd.h, sys/auxv.h, sys/mman.h, sys/prctl.h, sys/types.h, sys/wait.h,
// kselftest.h, mte_common_util.h, and mte_def.h.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type pid_t = c_int;
type pthread_t = c_ulong;
type time_t = c_long;

const NUM_ITERATIONS: usize = 1024;
const MAX_THREADS: usize = 5;
const THREAD_ITERATIONS: c_int = 1000;

// Constants supplied by kselftest.h and mte_def.h in the original C build.
extern "C" {
    static KSFT_PASS: c_int;
    static KSFT_FAIL: c_int;
    static PR_MTE_TCF_SYNC: u64;
    static PR_MTE_TCF_ASYNC: u64;
    static PR_TAGGED_ADDR_ENABLE: u64;
    static PR_MTE_TAG_SHIFT: c_int;
    static PR_SET_TAGGED_ADDR_CTRL: c_int;
    static PR_GET_TAGGED_ADDR_CTRL: c_int;
}

extern "C" {
    fn gettid() -> pid_t;
    fn getpid() -> pid_t;
    fn fork() -> pid_t;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn time(tloc: *mut time_t) -> time_t;
    fn srand(seed: c_uint);
    fn rand() -> c_int;
    fn perror(s: *const c_char);
    fn prctl(option: c_int, ...) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut c_void) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(cnt: c_uint);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_print_cnts();
    fn ksft_get_fail_cnt() -> c_int;
    fn evaluate_test(result: c_int, msg: *const c_char);
    fn mte_default_setup() -> c_int;
    fn mte_restore_setup();
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe extern "C" fn execute_thread(x: *mut c_void) -> *mut c_void {
    let pid: pid_t = *(x as *mut pid_t);
    let tid: pid_t = gettid();
    let mut prctl_tag_mask: u64;
    let prctl_set: u64;
    let prctl_get: u64;
    let prctl_tcf: u64;

    srand((time(ptr::null_mut()) ^ ((pid as time_t) << 16) ^ ((tid as time_t) << 16)) as c_uint);

    prctl_tag_mask = (rand() & 0xffff) as u64;

    if prctl_tag_mask % 2 != 0 {
        prctl_tcf = PR_MTE_TCF_SYNC;
    } else {
        prctl_tcf = PR_MTE_TCF_ASYNC;
    }

    prctl_set =
        PR_TAGGED_ADDR_ENABLE | prctl_tcf | (prctl_tag_mask << (PR_MTE_TAG_SHIFT as u32));

    for _j in 0..THREAD_ITERATIONS {
        if prctl(PR_SET_TAGGED_ADDR_CTRL, prctl_set, 0, 0, 0) != 0 {
            perror(c"prctl() failed".as_ptr());
            return KSFT_FAIL as usize as *mut c_void;
        }

        prctl_get = prctl(PR_GET_TAGGED_ADDR_CTRL, 0, 0, 0, 0) as u64;

        if prctl_set != prctl_get {
            ksft_print_msg(
                c"Error: prctl_set: 0x%lx != prctl_get: 0x%lx\n".as_ptr(),
                prctl_set,
                prctl_get,
            );
            return KSFT_FAIL as usize as *mut c_void;
        }
    }

    KSFT_PASS as usize as *mut c_void
}

unsafe fn execute_test(pid: pid_t) -> c_int {
    let mut thread_id: [pthread_t; MAX_THREADS] = [0; MAX_THREADS];
    let mut thread_data: [c_int; MAX_THREADS] = [0; MAX_THREADS];

    for i in 0..MAX_THREADS {
        pthread_create(
            &mut thread_id[i],
            ptr::null(),
            execute_thread,
            &pid as *const pid_t as *mut c_void,
        );
    }

    for i in 0..MAX_THREADS {
        pthread_join(thread_id[i], &mut thread_data[i] as *mut c_int as *mut c_void);
    }

    for i in 0..MAX_THREADS {
        if thread_data[i] == KSFT_FAIL {
            return KSFT_FAIL;
        }
    }

    KSFT_PASS
}

unsafe fn mte_gcr_fork_test() -> c_int {
    let mut pid: pid_t;
    let mut results: [c_int; NUM_ITERATIONS] = [0; NUM_ITERATIONS];
    let cpid: pid_t;
    let mut res: c_int = 0;

    for _i in 0..NUM_ITERATIONS {
        pid = fork();

        if pid < 0 {
            return KSFT_FAIL;
        }

        if pid == 0 {
            cpid = getpid();

            res = execute_test(cpid);

            exit(res);
        }
    }

    let mut i = 0usize;
    while i < NUM_ITERATIONS {
        wait(&mut res);

        if WIFEXITED(res) {
            results[i] = WEXITSTATUS(res);
            i += 1;
        }
    }

    for i in 0..NUM_ITERATIONS {
        if results[i] == KSFT_FAIL {
            return KSFT_FAIL;
        }
    }

    KSFT_PASS
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let err: c_int;

    err = mte_default_setup();
    if err != 0 {
        return err;
    }

    ksft_print_header();
    ksft_set_plan(1);

    evaluate_test(
        mte_gcr_fork_test(),
        c"Verify that GCR_EL1 is set correctly on context switch\n".as_ptr(),
    );

    mte_restore_setup();
    ksft_print_cnts();

    if ksft_get_fail_cnt() == 0 {
        KSFT_PASS
    } else {
        KSFT_FAIL
    }
}
