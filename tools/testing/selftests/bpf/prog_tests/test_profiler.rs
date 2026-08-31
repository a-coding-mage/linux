// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/*
 * Translated from C source depending on:
 * <test_progs.h>
 * "progs/profiler.h"
 * "profiler1.skel.h"
 * "profiler2.skel.h"
 * "profiler3.skel.h"
 */

use std::ffi::{c_char, c_int, c_void};
use std::mem;
use std::ptr;

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub ctx_in: *mut c_void,
    pub ctx_size_in: __u32,
    pub retval: __u32,
}

#[repr(C)]
pub struct profiler1_progs {
    pub raw_tracepoint__sched_process_exec: *mut bpf_program,
}

#[repr(C)]
pub struct profiler1 {
    pub progs: profiler1_progs,
}

#[repr(C)]
pub struct profiler2_progs {
    pub raw_tracepoint__sched_process_exec: *mut bpf_program,
}

#[repr(C)]
pub struct profiler2 {
    pub progs: profiler2_progs,
}

#[repr(C)]
pub struct profiler3_progs {
    pub raw_tracepoint__sched_process_exec: *mut bpf_program,
}

#[repr(C)]
pub struct profiler3 {
    pub progs: profiler3_progs,
}

unsafe extern "C" {
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;

    fn profiler1__open_and_load() -> *mut profiler1;
    fn profiler1__attach(skel: *mut profiler1) -> c_int;
    fn profiler1__destroy(skel: *mut profiler1);

    fn profiler2__open_and_load() -> *mut profiler2;
    fn profiler2__attach(skel: *mut profiler2) -> c_int;
    fn profiler2__destroy(skel: *mut profiler2);

    fn profiler3__open_and_load() -> *mut profiler3;
    fn profiler3__attach(skel: *mut profiler3) -> c_int;
    fn profiler3__destroy(skel: *mut profiler3);
}

unsafe fn sanity_run(prog: *mut bpf_program) -> c_int {
    let mut test_attr: bpf_test_run_opts = mem::zeroed();
    let mut args: [__u64; 3] = [1, 2, 3];
    let err: c_int;
    let prog_fd: c_int;

    prog_fd = bpf_program__fd(prog);
    test_attr.ctx_in = args.as_mut_ptr() as *mut c_void;
    test_attr.ctx_size_in = mem::size_of_val(&args) as __u32;
    err = bpf_prog_test_run_opts(prog_fd, &mut test_attr);
    if !ASSERT_OK(err, c"test_run".as_ptr()) {
        return -1;
    }

    if !ASSERT_OK(test_attr.retval as c_int, c"test_run retval".as_ptr()) {
        return -1;
    }

    return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_test_profiler() {
    let mut profiler1_skel: *mut profiler1 = ptr::null_mut();
    let mut profiler2_skel: *mut profiler2 = ptr::null_mut();
    let mut profiler3_skel: *mut profiler3 = ptr::null_mut();
    let _duration: __u32 = 0;
    let mut err: c_int;

    profiler1_skel = profiler1__open_and_load();
    if CHECK(
        profiler1_skel.is_null(),
        c"profiler1_skel_load".as_ptr(),
        c"profiler1 skeleton failed\n".as_ptr(),
    ) {
        goto_cleanup(profiler1_skel, profiler2_skel, profiler3_skel);
        return;
    }

    err = profiler1__attach(profiler1_skel);
    if CHECK(
        err != 0,
        c"profiler1_attach".as_ptr(),
        c"profiler1 attach failed: %d\n".as_ptr(),
        err,
    ) {
        goto_cleanup(profiler1_skel, profiler2_skel, profiler3_skel);
        return;
    }

    if sanity_run((*profiler1_skel).progs.raw_tracepoint__sched_process_exec) != 0 {
        goto_cleanup(profiler1_skel, profiler2_skel, profiler3_skel);
        return;
    }

    profiler2_skel = profiler2__open_and_load();
    if CHECK(
        profiler2_skel.is_null(),
        c"profiler2_skel_load".as_ptr(),
        c"profiler2 skeleton failed\n".as_ptr(),
    ) {
        goto_cleanup(profiler1_skel, profiler2_skel, profiler3_skel);
        return;
    }

    err = profiler2__attach(profiler2_skel);
    if CHECK(
        err != 0,
        c"profiler2_attach".as_ptr(),
        c"profiler2 attach failed: %d\n".as_ptr(),
        err,
    ) {
        goto_cleanup(profiler1_skel, profiler2_skel, profiler3_skel);
        return;
    }

    if sanity_run((*profiler2_skel).progs.raw_tracepoint__sched_process_exec) != 0 {
        goto_cleanup(profiler1_skel, profiler2_skel, profiler3_skel);
        return;
    }

    profiler3_skel = profiler3__open_and_load();
    if CHECK(
        profiler3_skel.is_null(),
        c"profiler3_skel_load".as_ptr(),
        c"profiler3 skeleton failed\n".as_ptr(),
    ) {
        goto_cleanup(profiler1_skel, profiler2_skel, profiler3_skel);
        return;
    }

    err = profiler3__attach(profiler3_skel);
    if CHECK(
        err != 0,
        c"profiler3_attach".as_ptr(),
        c"profiler3 attach failed: %d\n".as_ptr(),
        err,
    ) {
        goto_cleanup(profiler1_skel, profiler2_skel, profiler3_skel);
        return;
    }

    if sanity_run((*profiler3_skel).progs.raw_tracepoint__sched_process_exec) != 0 {
        goto_cleanup(profiler1_skel, profiler2_skel, profiler3_skel);
        return;
    }

    goto_cleanup(profiler1_skel, profiler2_skel, profiler3_skel);
}

unsafe fn goto_cleanup(
    profiler1_skel: *mut profiler1,
    profiler2_skel: *mut profiler2,
    profiler3_skel: *mut profiler3,
) {
    profiler1__destroy(profiler1_skel);
    profiler2__destroy(profiler2_skel);
    profiler3__destroy(profiler3_skel);
}
