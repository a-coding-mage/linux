// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */
// C dependencies:
// #include <test_progs.h>
// #include "fentry_test.lskel.h"
// #include "fexit_test.lskel.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub type __u64 = u64;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: i32,
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
pub struct fentry_test_lskel_bss {
    pub test1_result: __u64,
}

#[repr(C)]
pub struct fexit_test_lskel_bss {
    pub test1_result: __u64,
}

#[repr(C)]
pub struct fexit_test_lskel_prog {
    pub prog_fd: i32,
}

#[repr(C)]
pub struct fexit_test_lskel_progs {
    pub test1: fexit_test_lskel_prog,
}

#[repr(C)]
pub struct fentry_test_lskel {
    pub keyring_id: i32,
    pub bss: *mut fentry_test_lskel_bss,
}

#[repr(C)]
pub struct fexit_test_lskel {
    pub keyring_id: i32,
    pub bss: *mut fexit_test_lskel_bss,
    pub progs: fexit_test_lskel_progs,
}

unsafe extern "C" {
    static mut KEY_SPEC_SESSION_KEYRING: i32;

    fn fentry_test_lskel__open() -> *mut fentry_test_lskel;
    fn fentry_test_lskel__load(skel: *mut fentry_test_lskel) -> i32;
    fn fentry_test_lskel__attach(skel: *mut fentry_test_lskel) -> i32;
    fn fentry_test_lskel__destroy(skel: *mut fentry_test_lskel);

    fn fexit_test_lskel__open() -> *mut fexit_test_lskel;
    fn fexit_test_lskel__load(skel: *mut fexit_test_lskel) -> i32;
    fn fexit_test_lskel__attach(skel: *mut fexit_test_lskel) -> i32;
    fn fexit_test_lskel__destroy(skel: *mut fexit_test_lskel);

    fn bpf_prog_test_run_opts(prog_fd: i32, opts: *mut bpf_test_run_opts) -> i32;

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const u8) -> bool;
    fn ASSERT_OK(err: i32, name: *const u8) -> bool;
    fn ASSERT_EQ(actual: __u64, expected: __u64, name: *const u8) -> bool;
    fn printf(fmt: *const u8, ...) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn test_fentry_fexit() {
    let mut fentry_skel: *mut fentry_test_lskel = core::ptr::null_mut();
    let mut fexit_skel: *mut fexit_test_lskel = core::ptr::null_mut();
    let mut fentry_res: *mut __u64;
    let mut fexit_res: *mut __u64;
    let mut err: i32;
    let prog_fd: i32;
    let mut i: i32;
    // LIBBPF_OPTS(bpf_test_run_opts, topts);
    let mut topts: bpf_test_run_opts = bpf_test_run_opts::default();

    fentry_skel = fentry_test_lskel__open();
    if !ASSERT_OK_PTR(fentry_skel as *const core::ffi::c_void, b"fentry_skel_load\0".as_ptr()) {
        goto_close_prog(fentry_skel, fexit_skel);
        return;
    }

    (*fentry_skel).keyring_id = KEY_SPEC_SESSION_KEYRING;
    err = fentry_test_lskel__load(fentry_skel);
    if !ASSERT_OK(err, b"fentry_skel_load\0".as_ptr()) {
        goto_close_prog(fentry_skel, fexit_skel);
        return;
    }

    fexit_skel = fexit_test_lskel__open();
    if !ASSERT_OK_PTR(fexit_skel as *const core::ffi::c_void, b"fexit_skel_load\0".as_ptr()) {
        goto_close_prog(fentry_skel, fexit_skel);
        return;
    }

    (*fexit_skel).keyring_id = KEY_SPEC_SESSION_KEYRING;
    err = fexit_test_lskel__load(fexit_skel);
    if !ASSERT_OK(err, b"fexit_skel_load\0".as_ptr()) {
        goto_close_prog(fentry_skel, fexit_skel);
        return;
    }

    err = fentry_test_lskel__attach(fentry_skel);
    if !ASSERT_OK(err, b"fentry_attach\0".as_ptr()) {
        goto_close_prog(fentry_skel, fexit_skel);
        return;
    }
    err = fexit_test_lskel__attach(fexit_skel);
    if !ASSERT_OK(err, b"fexit_attach\0".as_ptr()) {
        goto_close_prog(fentry_skel, fexit_skel);
        return;
    }

    prog_fd = (*fexit_skel).progs.test1.prog_fd;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, b"ipv6 test_run\0".as_ptr());
    ASSERT_OK(topts.retval, b"ipv6 test retval\0".as_ptr());

    fentry_res = (*fentry_skel).bss as *mut __u64;
    fexit_res = (*fexit_skel).bss as *mut __u64;
    printf(
        b"%lld\n\0".as_ptr(),
        (*(*fentry_skel).bss).test1_result as i64,
    );
    i = 0;
    while i < 8 {
        ASSERT_EQ(*fentry_res.offset(i as isize), 1, b"fentry result\0".as_ptr());
        ASSERT_EQ(*fexit_res.offset(i as isize), 1, b"fexit result\0".as_ptr());
        i += 1;
    }

    goto_close_prog(fentry_skel, fexit_skel);
}

unsafe fn goto_close_prog(
    fentry_skel: *mut fentry_test_lskel,
    fexit_skel: *mut fexit_test_lskel,
) {
    fentry_test_lskel__destroy(fentry_skel);
    fexit_test_lskel__destroy(fexit_skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
