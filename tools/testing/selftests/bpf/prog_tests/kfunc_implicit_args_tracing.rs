// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external declarations:
// #include <test_progs.h>
// #include "kfunc_implicit_args_tracing.skel.h"

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: u32,
}

#[repr(C)]
pub struct kfunc_implicit_args_tracing {
    pub progs: kfunc_implicit_args_tracing__progs,
    pub bss: *mut kfunc_implicit_args_tracing__bss,
}

#[repr(C)]
pub struct kfunc_implicit_args_tracing__progs {
    pub trigger_implicit_arg: *mut bpf_program,
}

#[repr(C)]
pub struct kfunc_implicit_args_tracing__bss {
    pub fentry_arg_cnt: i32,
    pub fentry_aux_arg: u64,
    pub fentry_result: i32,
    pub fexit_arg_cnt: i32,
    pub fexit_aux_arg: u64,
    pub fexit_result: i32,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn kfunc_implicit_args_tracing__open_and_load() -> *mut kfunc_implicit_args_tracing;
    fn kfunc_implicit_args_tracing__attach(skel: *mut kfunc_implicit_args_tracing) -> i32;
    fn kfunc_implicit_args_tracing__destroy(skel: *mut kfunc_implicit_args_tracing);
    fn bpf_program__fd(prog: *mut bpf_program) -> i32;
    fn bpf_prog_test_run_opts(fd: i32, opts: *mut bpf_test_run_opts) -> i32;

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_OK(err: i32, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_EQ(actual: u64, expected: u64, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_NEQ(actual: u64, expected: u64, name: *const core::ffi::c_char) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kfunc_implicit_args_tracing() {
    let skel: *mut kfunc_implicit_args_tracing;
    let mut topts: bpf_test_run_opts = core::mem::zeroed();
    let mut err: i32;
    let fd: i32;

    skel = kfunc_implicit_args_tracing__open_and_load();
    if !ASSERT_OK_PTR(skel as *const core::ffi::c_void, c"open_and_load".as_ptr()) {
        return;
    }

    err = kfunc_implicit_args_tracing__attach(skel);
    if !ASSERT_OK(err, c"attach".as_ptr()) {
        kfunc_implicit_args_tracing__destroy(skel);
        return;
    }

    fd = bpf_program__fd((*skel).progs.trigger_implicit_arg);
    err = bpf_prog_test_run_opts(fd, &mut topts);
    if !ASSERT_OK(err, c"test_run".as_ptr()) {
        kfunc_implicit_args_tracing__destroy(skel);
        return;
    }

    ASSERT_EQ(topts.retval as u64, 5, c"kfunc_retval".as_ptr());
    ASSERT_EQ((*(*skel).bss).fentry_arg_cnt as u64, 2, c"fentry_arg_cnt".as_ptr());
    ASSERT_NEQ((*(*skel).bss).fentry_aux_arg as u64, 0, c"fentry_aux_arg".as_ptr());
    ASSERT_EQ((*(*skel).bss).fentry_result as u64, 1, c"fentry_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).fexit_arg_cnt as u64, 2, c"fexit_arg_cnt".as_ptr());
    ASSERT_NEQ((*(*skel).bss).fexit_aux_arg as u64, 0, c"fexit_aux_arg".as_ptr());
    ASSERT_EQ((*(*skel).bss).fexit_result as u64, 1, c"fexit_result".as_ptr());

    kfunc_implicit_args_tracing__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
