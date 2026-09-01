// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */

/* Translated from:
 * - <sched.h>
 * - <pthread.h>
 * - <stdbool.h>
 * - <test_progs.h>
 * - "preempted_bpf_ma_op.skel.h"
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

const ALLOC_THREAD_NR: usize = 4;
const ALLOC_LOOP_NR: c_int = 512;

#[repr(C)]
struct cpu_set_t {
    __opaque: [u8; 0],
}

type pthread_t = usize;

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct preempted_bpf_ma_op_bss {
    nomem_err: bool,
}

#[repr(C)]
struct preempted_bpf_ma_op {
    obj: *mut bpf_object,
    bss: *mut preempted_bpf_ma_op_bss,
}

#[repr(C)]
struct bpf_test_run_opts {
    retval: c_int,
}

#[repr(C)]
struct alloc_ctx {
    /* output */
    run_err: c_int,
    /* input */
    fd: c_int,
    nomem_err: *mut bool,
}

extern "C" {
    fn CPU_ZERO(cpusetp: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, cpusetp: *mut cpu_set_t);
    fn pthread_self() -> pthread_t;
    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: usize, cpuset: *const cpu_set_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;

    fn preempted_bpf_ma_op__open_and_load() -> *mut preempted_bpf_ma_op;
    fn preempted_bpf_ma_op__attach(skel: *mut preempted_bpf_ma_op) -> c_int;
    fn preempted_bpf_ma_op__destroy(skel: *mut preempted_bpf_ma_op);

    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_FALSE(condition: bool, name: *const c_char) -> bool;
}

unsafe extern "C" fn run_alloc_prog(data: *mut c_void) -> *mut c_void {
    let ctx = data as *mut alloc_ctx;
    let mut cpu_set: cpu_set_t = mem::zeroed();
    let mut i: c_int;

    CPU_ZERO(&mut cpu_set);
    CPU_SET(0, &mut cpu_set);
    pthread_setaffinity_np(pthread_self(), mem::size_of_val(&cpu_set), &cpu_set);

    i = 0;
    while i < ALLOC_LOOP_NR && !*(*ctx).nomem_err {
        let mut topts: bpf_test_run_opts = mem::zeroed();
        let err: c_int;

        err = bpf_prog_test_run_opts((*ctx).fd, &mut topts);
        (*ctx).run_err |= err | topts.retval;

        i += 1;
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn test_preempted_bpf_ma_op() {
    let mut ctx: [alloc_ctx; ALLOC_THREAD_NR] = mem::zeroed();
    let mut skel: *mut preempted_bpf_ma_op;
    let mut tid: [pthread_t; ALLOC_THREAD_NR] = mem::zeroed();
    let mut i: usize;
    let mut err: c_int;

    skel = preempted_bpf_ma_op__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, b"open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    err = preempted_bpf_ma_op__attach(skel);
    if !ASSERT_OK(err, b"attach\0".as_ptr() as *const c_char) {
        goto_out(skel);
        return;
    }

    i = 0;
    while i < ctx.len() {
        let mut prog: *mut bpf_program;
        let mut name: [c_char; 8] = [0; 8];

        snprintf(
            name.as_mut_ptr(),
            mem::size_of_val(&name),
            b"test%d\0".as_ptr() as *const c_char,
            i as c_int,
        );
        prog = bpf_object__find_program_by_name((*skel).obj, name.as_ptr());
        if !ASSERT_OK_PTR(prog as *mut c_void, b"no test prog\0".as_ptr() as *const c_char) {
            goto_out(skel);
            return;
        }

        ctx[i].run_err = 0;
        ctx[i].fd = bpf_program__fd(prog);
        ctx[i].nomem_err = &mut (*(*skel).bss).nomem_err;

        i += 1;
    }

    memset(
        tid.as_mut_ptr() as *mut c_void,
        0,
        mem::size_of_val(&tid),
    );
    i = 0;
    while i < tid.len() {
        err = pthread_create(
            &mut tid[i],
            ptr::null(),
            run_alloc_prog,
            &mut ctx[i] as *mut alloc_ctx as *mut c_void,
        );
        if !ASSERT_OK(err, b"pthread_create\0".as_ptr() as *const c_char) {
            break;
        }

        i += 1;
    }

    i = 0;
    while i < tid.len() {
        if tid[i] == 0 {
            break;
        }
        pthread_join(tid[i], ptr::null_mut());
        ASSERT_EQ(ctx[i].run_err, 0, b"run prog err\0".as_ptr() as *const c_char);

        i += 1;
    }

    ASSERT_FALSE((*(*skel).bss).nomem_err, b"ENOMEM\0".as_ptr() as *const c_char);

    goto_out(skel);
}

unsafe fn goto_out(skel: *mut preempted_bpf_ma_op) {
    preempted_bpf_ma_op__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
