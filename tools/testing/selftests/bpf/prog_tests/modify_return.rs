// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2020 Google LLC.
 */

// C dependencies: <test_progs.h>, <unistd.h>, "modify_return.skel.h"

use std::os::raw::{c_char, c_int};

type __u16 = u16;
type __s16 = i16;
type __u32 = u32;

const EINVAL: c_int = 22;

macro_rules! LOWER {
    ($x:expr) => {
        (($x) & 0xffff)
    };
}

macro_rules! UPPER {
    ($x:expr) => {
        (($x) >> 16)
    };
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: __u32,
}

#[repr(C)]
pub struct modify_return {
    pub bss: *mut modify_return_bss,
    pub progs: modify_return_progs,
}

#[repr(C)]
pub struct modify_return_bss {
    pub input_retval: __u32,
    pub test_pid: c_int,
    pub fentry_result: c_int,
    pub fexit_result: c_int,
    pub fmod_ret_result: c_int,
    pub fentry_result2: c_int,
    pub fexit_result2: c_int,
    pub fmod_ret_result2: c_int,
}

#[repr(C)]
pub struct modify_return_progs {
    pub fmod_ret_test: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

extern "C" {
    fn getpid() -> c_int;

    fn modify_return__open_and_load() -> *mut modify_return;
    fn modify_return__attach(skel: *mut modify_return) -> c_int;
    fn modify_return__destroy(skel: *mut modify_return);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *mut modify_return, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: i64, expected: i64, name: *const c_char) -> bool;
}

fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn run_test(input_retval: __u32, want_side_effect: __u16, want_ret: __s16) {
    let mut skel: *mut modify_return = std::ptr::null_mut();
    let mut err: c_int;
    let prog_fd: c_int;
    let side_effect: __u16;
    let ret: __s16;
    let mut topts: bpf_test_run_opts = std::mem::zeroed();

    skel = modify_return__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr(b"skel_load\0")) {
        goto_cleanup(skel);
        return;
    }

    (*(*skel).bss).input_retval = input_retval;
    (*(*skel).bss).test_pid = getpid();

    err = modify_return__attach(skel);
    if !ASSERT_OK(err, cstr(b"modify_return__attach failed\0")) {
        goto_cleanup(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.fmod_ret_test);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, cstr(b"test_run\0"));

    side_effect = UPPER!(topts.retval) as __u16;
    ret = LOWER!(topts.retval) as __s16;

    ASSERT_EQ(ret as i64, want_ret as i64, cstr(b"test_run ret\0"));
    ASSERT_EQ(
        side_effect as i64,
        want_side_effect as i64,
        cstr(b"modify_return side_effect\0"),
    );
    ASSERT_EQ(
        (*(*skel).bss).fentry_result as i64,
        1,
        cstr(b"modify_return fentry_result\0"),
    );
    ASSERT_EQ(
        (*(*skel).bss).fexit_result as i64,
        1,
        cstr(b"modify_return fexit_result\0"),
    );
    ASSERT_EQ(
        (*(*skel).bss).fmod_ret_result as i64,
        1,
        cstr(b"modify_return fmod_ret_result\0"),
    );

    ASSERT_EQ(
        (*(*skel).bss).fentry_result2 as i64,
        1,
        cstr(b"modify_return fentry_result2\0"),
    );
    ASSERT_EQ(
        (*(*skel).bss).fexit_result2 as i64,
        1,
        cstr(b"modify_return fexit_result2\0"),
    );
    ASSERT_EQ(
        (*(*skel).bss).fmod_ret_result2 as i64,
        1,
        cstr(b"modify_return fmod_ret_result2\0"),
    );

    goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut modify_return) {
    modify_return__destroy(skel);
}

pub unsafe extern "C" fn test_modify_return() {
    run_test(
        0, /* input_retval */
        2, /* want_side_effect */
        33, /* want_ret */
    );
    run_test(
        (-EINVAL) as __u32, /* input_retval */
        0,                 /* want_side_effect */
        (-EINVAL * 2) as __s16, /* want_ret */
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
