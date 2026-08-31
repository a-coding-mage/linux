// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
// C includes translated as external dependencies:
// #include <test_progs.h>
// #include "timer_start_deadlock.skel.h"

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub retval: u32,
}

#[repr(C)]
pub struct timer_start_deadlock {
    pub progs: timer_start_deadlock__progs,
    pub bss: *mut timer_start_deadlock__bss,
}

#[repr(C)]
pub struct timer_start_deadlock__progs {
    pub start_timer: *mut bpf_program,
}

#[repr(C)]
pub struct timer_start_deadlock__bss {
    pub tp_called: c_int,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn timer_start_deadlock__open_and_load() -> *mut timer_start_deadlock;
    fn timer_start_deadlock__attach(skel: *mut timer_start_deadlock) -> c_int;
    fn timer_start_deadlock__destroy(skel: *mut timer_start_deadlock);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_timer_start_deadlock() {
    let skel: *mut timer_start_deadlock;
    let mut err: c_int;
    let prog_fd: c_int;
    // LIBBPF_OPTS(bpf_test_run_opts, opts);
    let mut opts: bpf_test_run_opts = core::mem::zeroed();
    opts.sz = core::mem::size_of::<bpf_test_run_opts>();

    skel = timer_start_deadlock__open_and_load();
    if !ASSERT_OK_PTR(skel as *const core::ffi::c_void, c"skel_open_and_load".as_ptr()) {
        return;
    }

    err = timer_start_deadlock__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        timer_start_deadlock__destroy(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.start_timer);

    /*
     * Run the syscall program that attempts to deadlock.
     * If the kernel deadlocks, this call will never return.
     */
    err = bpf_prog_test_run_opts(prog_fd, &mut opts);
    ASSERT_OK(err, c"prog_test_run".as_ptr());
    ASSERT_EQ(opts.retval, 0, c"prog_retval".as_ptr());

    ASSERT_EQ((*(*skel).bss).tp_called, 1, c"tp_called".as_ptr());
    timer_start_deadlock__destroy(skel);
}
