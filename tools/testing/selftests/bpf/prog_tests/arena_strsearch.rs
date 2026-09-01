// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
// C dependencies: <test_progs.h>, "arena_strsearch.skel.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: c_int,
}

#[repr(C)]
pub struct arena_strsearch {
    pub progs: arena_strsearch_progs,
    pub bss: *mut arena_strsearch_bss,
}

#[repr(C)]
pub struct arena_strsearch_progs {
    pub arena_strsearch: *mut c_void,
}

#[repr(C)]
pub struct arena_strsearch_bss {
    pub skip: bool,
}

unsafe extern "C" {
    fn arena_strsearch__open_and_load() -> *mut arena_strsearch;
    fn arena_strsearch__destroy(skel: *mut arena_strsearch);
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut c_void) -> c_int;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn printf(fmt: *const c_char, ...) -> c_int;
}

unsafe fn ASSERT_OK_PTR(ptr: *mut arena_strsearch, _name: *const c_char) -> bool {
    !ptr.is_null()
}

unsafe fn ASSERT_OK(ret: c_int, _name: *const c_char) -> bool {
    ret == 0
}

unsafe fn test_arena_str() {
    let mut opts: bpf_test_run_opts = core::mem::zeroed();
    let skel: *mut arena_strsearch;
    let ret: c_int;

    skel = arena_strsearch__open_and_load();
    if !ASSERT_OK_PTR(skel, c"arena_strsearch__open_and_load".as_ptr()) {
        return;
    }

    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.arena_strsearch),
        &mut opts as *mut bpf_test_run_opts,
    );
    ASSERT_OK(ret, c"ret_add".as_ptr());
    ASSERT_OK(opts.retval, c"retval".as_ptr());
    if (*(*skel).bss).skip {
        printf(
            c"%s:SKIP:compiler doesn't support arena_cast\n".as_ptr(),
            c"test_arena_str".as_ptr(),
        );
        test__skip();
    }
    arena_strsearch__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_arena_strsearch() {
    if test__start_subtest(c"arena_strsearch".as_ptr()) {
        test_arena_str();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
