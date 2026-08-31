// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
// C dependencies translated as external Rust dependencies:
// <test_progs.h>, <network_helpers.h>, <sys/sysinfo.h>
// "arena_spin_lock.skel.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::{mem, ptr};

type u32 = c_uint;
type pthread_t = usize;

#[repr(C)]
pub struct __qspinlock {
    pub val: c_int,
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pthread_barrier_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const c_void,
    pub data_size_in: u32,
    pub repeat: c_int,
    pub retval: c_int,
}

#[repr(C)]
pub struct arena_spin_lock_data {
    pub test_skip: c_int,
}

#[repr(C)]
pub struct arena_spin_lock_bss {
    pub cs_count: c_int,
    pub limit: c_int,
    pub counter: c_int,
}

#[repr(C)]
pub struct arena_spin_lock_progs {
    pub prog: *mut bpf_program,
}

#[repr(C)]
pub struct arena_spin_lock {
    pub data: *mut arena_spin_lock_data,
    pub bss: *mut arena_spin_lock_bss,
    pub progs: arena_spin_lock_progs,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

const PTHREAD_BARRIER_SERIAL_THREAD: c_int = -1;
const EOPNOTSUPP: c_int = 95;

static mut cpu: c_long = 0;
static mut repeat: c_int = 0;

static mut barrier: pthread_barrier_t = pthread_barrier_t { _private: [] };

unsafe extern "C" {
    static pkt_v4: c_void;

    fn CPU_ZERO(set: *mut cpu_set_t);
    fn CPU_SET(cpu: c_long, set: *mut cpu_set_t);
    fn __sync_fetch_and_add(ptr: *mut c_long, value: c_long) -> c_long;

    fn pthread_self() -> pthread_t;
    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: usize, cpuset: *const cpu_set_t) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: c_uint,
    ) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;

    fn get_nprocs() -> c_int;

    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn arena_spin_lock__open_and_load() -> *mut arena_spin_lock;
    fn arena_spin_lock__destroy(skel: *mut arena_spin_lock);

    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn printf(format: *const c_char, ...) -> c_int;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut arena_spin_lock, name: *const c_char) -> bool;
    fn ASSERT_FALSE(condition: bool, name: *const c_char) -> bool;
    fn ASSERT_EQ_int(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ_ptr(actual: *mut c_void, expected: *const c_void, name: *const c_char) -> bool;
}

unsafe extern "C" fn spin_lock_thread(arg: *mut c_void) -> *mut c_void {
    let mut err: c_int;
    let prog_fd: c_int = *(arg as *mut u32) as c_int;
    let mut topts = bpf_test_run_opts {
        data_in: &pkt_v4 as *const c_void,
        data_size_in: mem::size_of_val(&pkt_v4) as u32,
        repeat,
        retval: 0,
    };
    let mut cpuset: cpu_set_t = mem::zeroed();

    CPU_ZERO(&mut cpuset);
    CPU_SET(__sync_fetch_and_add(&raw mut cpu, 1), &mut cpuset);
    ASSERT_OK(
        pthread_setaffinity_np(pthread_self(), mem::size_of_val(&cpuset), &cpuset),
        c"cpu affinity".as_ptr(),
    );

    err = pthread_barrier_wait(&raw mut barrier);
    if err != PTHREAD_BARRIER_SERIAL_THREAD && err != 0 {
        ASSERT_FALSE(true, c"pthread_barrier".as_ptr());
    }

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run err".as_ptr());

    if topts.retval == -EOPNOTSUPP {
        pthread_exit(arg);
    }

    ASSERT_EQ_int(topts.retval as c_int, 0, c"test_run retval".as_ptr());

    pthread_exit(arg);
}

unsafe fn test_arena_spin_lock_size(size: c_int) {
    let mut topts = bpf_test_run_opts {
        data_in: ptr::null(),
        data_size_in: 0,
        repeat: 0,
        retval: 0,
    };
    let mut skel: *mut arena_spin_lock;
    let mut thread_id: [pthread_t; 16] = [0; 16];
    let prog_fd: c_int;
    let mut i: c_int;
    let mut err: c_int;
    let nthreads: c_int;
    let mut ret: *mut c_void = ptr::null_mut();

    nthreads = core::cmp::min(get_nprocs(), thread_id.len() as c_int);
    if nthreads < 2 {
        test__skip();
        return;
    }

    skel = arena_spin_lock__open_and_load();
    if !ASSERT_OK_PTR(skel, c"arena_spin_lock__open_and_load".as_ptr()) {
        return;
    }

    if (*(*skel).data).test_skip == 2 {
        test__skip();
        arena_spin_lock__destroy(skel);
        return;
    }
    (*(*skel).bss).cs_count = size;
    (*(*skel).bss).limit = repeat * nthreads;

    ASSERT_OK(
        pthread_barrier_init(&raw mut barrier, ptr::null(), nthreads as c_uint),
        c"barrier init".as_ptr(),
    );

    prog_fd = bpf_program__fd((*skel).progs.prog);
    i = 0;
    while i < nthreads {
        err = pthread_create(
            &mut thread_id[i as usize],
            ptr::null(),
            Some(spin_lock_thread),
            &prog_fd as *const c_int as *mut c_void,
        );
        if !ASSERT_OK(err, c"pthread_create".as_ptr()) {
            pthread_barrier_destroy(&raw mut barrier);
            arena_spin_lock__destroy(skel);
            return;
        }
        i += 1;
    }

    i = 0;
    while i < nthreads {
        if !ASSERT_OK(
            pthread_join(thread_id[i as usize], &mut ret),
            c"pthread_join".as_ptr(),
        ) {
            pthread_barrier_destroy(&raw mut barrier);
            arena_spin_lock__destroy(skel);
            return;
        }
        if !ASSERT_EQ_ptr(
            ret,
            &prog_fd as *const c_int as *const c_void,
            c"ret == prog_fd".as_ptr(),
        ) {
            pthread_barrier_destroy(&raw mut barrier);
            arena_spin_lock__destroy(skel);
            return;
        }
        i += 1;
    }

    if (*(*skel).data).test_skip == 3 {
        printf(
            c"%s:SKIP: CONFIG_NR_CPUS exceed the maximum supported by arena spinlock\n".as_ptr(),
            c"test_arena_spin_lock_size".as_ptr(),
        );
        test__skip();
        pthread_barrier_destroy(&raw mut barrier);
        arena_spin_lock__destroy(skel);
        return;
    }

    ASSERT_EQ_int((*(*skel).bss).counter, repeat * nthreads, c"check counter value".as_ptr());

    pthread_barrier_destroy(&raw mut barrier);
    arena_spin_lock__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_arena_spin_lock() {
    repeat = 1000;
    if test__start_subtest(c"arena_spin_lock_1".as_ptr()) {
        test_arena_spin_lock_size(1);
    }
    cpu = 0;
    if test__start_subtest(c"arena_spin_lock_1000".as_ptr()) {
        test_arena_spin_lock_size(1000);
    }
    cpu = 0;
    repeat = 100;
    if test__start_subtest(c"arena_spin_lock_50000".as_ptr()) {
        test_arena_spin_lock_size(50000);
    }
}
