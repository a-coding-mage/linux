// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

/*
 * Translated from C implementation source.
 *
 * Original includes:
 *   <test_progs.h>
 *   "fexit_test.lskel.h"
 *   "fexit_many_args.skel.h"
 *
 * The ASSERT_* and test__start_subtest macros/functions, BPF skeleton types,
 * KEY_SPEC_SESSION_KEYRING, and libbpf helpers are expected to be supplied by
 * the surrounding translated test infrastructure.
 */

use core::ffi::c_char;

#[repr(C)]
pub struct fexit_test_lskel {
    pub progs: fexit_test_lskel_progs,
    pub bss: *mut fexit_test_lskel_bss,
    pub keyring_id: i32,
}

#[repr(C)]
pub struct fexit_test_lskel_progs {
    pub test1: fexit_test_lskel_prog,
}

#[repr(C)]
pub struct fexit_test_lskel_prog {
    pub prog_fd: i32,
}

#[repr(C)]
pub struct fexit_test_lskel_bss {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fexit_many_args {
    pub bss: *mut fexit_many_args_bss,
}

#[repr(C)]
pub struct fexit_many_args_bss {
    pub test1_result: u64,
    pub test2_result: u64,
    pub test3_result: u64,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: u32,
}

unsafe extern "C" {
    static KEY_SPEC_SESSION_KEYRING: i32;

    fn fexit_test_lskel__attach(fexit_skel: *mut fexit_test_lskel) -> i32;
    fn fexit_test_lskel__test1__attach(fexit_skel: *mut fexit_test_lskel) -> i32;
    fn fexit_test_lskel__detach(fexit_skel: *mut fexit_test_lskel);
    fn fexit_test_lskel__open() -> *mut fexit_test_lskel;
    fn fexit_test_lskel__load(fexit_skel: *mut fexit_test_lskel) -> i32;
    fn fexit_test_lskel__destroy(fexit_skel: *mut fexit_test_lskel);

    fn fexit_many_args__open_and_load() -> *mut fexit_many_args;
    fn fexit_many_args__attach(fexit_skel: *mut fexit_many_args) -> i32;
    fn fexit_many_args__destroy(fexit_skel: *mut fexit_many_args);

    fn bpf_prog_test_run_opts(prog_fd: i32, topts: *mut bpf_test_run_opts) -> i32;
    fn trigger_module_test_read(arg: i32) -> i32;
    fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn fexit_test_common(fexit_skel: *mut fexit_test_lskel) -> i32 {
    let mut err: i32;
    let prog_fd: i32;
    let mut i: usize;
    let link_fd: i32;
    let result: *mut u64;
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    err = fexit_test_lskel__attach(fexit_skel);
    if !ASSERT_OK!(err, b"fexit_attach\0".as_ptr() as *const c_char) {
        return err;
    }

    /* Check that already linked program can't be attached again. */
    link_fd = fexit_test_lskel__test1__attach(fexit_skel);
    if !ASSERT_LT!(link_fd, 0, b"fexit_attach_link\0".as_ptr() as *const c_char) {
        return -1;
    }

    prog_fd = (*fexit_skel).progs.test1.prog_fd;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK!(err, b"test_run\0".as_ptr() as *const c_char);
    ASSERT_EQ!(topts.retval, 0, b"test_run\0".as_ptr() as *const c_char);

    result = (*fexit_skel).bss as *mut u64;
    i = 0;
    while i < core::mem::size_of_val(&*(*fexit_skel).bss) / core::mem::size_of::<u64>() {
        if !ASSERT_EQ!(*result.add(i), 1, b"fexit_result\0".as_ptr() as *const c_char) {
            return -1;
        }
        i += 1;
    }

    fexit_test_lskel__detach(fexit_skel);

    /* zero results for re-attach test */
    core::ptr::write_bytes(
        (*fexit_skel).bss as *mut u8,
        0,
        core::mem::size_of_val(&*(*fexit_skel).bss),
    );
    0
}

unsafe fn fexit_test() {
    let mut fexit_skel: *mut fexit_test_lskel = core::ptr::null_mut();
    let mut err: i32;

    fexit_skel = fexit_test_lskel__open();
    if !ASSERT_OK_PTR!(fexit_skel, b"fexit_skel_open\0".as_ptr() as *const c_char) {
        goto_cleanup_fexit_test(fexit_skel);
        return;
    }

    (*fexit_skel).keyring_id = KEY_SPEC_SESSION_KEYRING;
    err = fexit_test_lskel__load(fexit_skel);
    if !ASSERT_OK!(err, b"fexit_skel_load\0".as_ptr() as *const c_char) {
        goto_cleanup_fexit_test(fexit_skel);
        return;
    }

    err = fexit_test_common(fexit_skel);
    if !ASSERT_OK!(err, b"fexit_first_attach\0".as_ptr() as *const c_char) {
        goto_cleanup_fexit_test(fexit_skel);
        return;
    }

    err = fexit_test_common(fexit_skel);
    ASSERT_OK!(err, b"fexit_second_attach\0".as_ptr() as *const c_char);

    goto_cleanup_fexit_test(fexit_skel);
}

unsafe fn goto_cleanup_fexit_test(fexit_skel: *mut fexit_test_lskel) {
    fexit_test_lskel__destroy(fexit_skel);
}

unsafe fn fexit_many_args() {
    let mut fexit_skel: *mut fexit_many_args = core::ptr::null_mut();
    let mut err: i32;

    fexit_skel = fexit_many_args__open_and_load();
    if !ASSERT_OK_PTR!(
        fexit_skel,
        b"fexit_many_args_skel_load\0".as_ptr() as *const c_char
    ) {
        goto_cleanup_fexit_many_args(fexit_skel);
        return;
    }

    err = fexit_many_args__attach(fexit_skel);
    if !ASSERT_OK!(
        err,
        b"fexit_many_args_attach\0".as_ptr() as *const c_char
    ) {
        goto_cleanup_fexit_many_args(fexit_skel);
        return;
    }

    ASSERT_OK!(
        trigger_module_test_read(1),
        b"trigger_read\0".as_ptr() as *const c_char
    );

    ASSERT_EQ!(
        (*(*fexit_skel).bss).test1_result,
        1,
        b"fexit_many_args_result1\0".as_ptr() as *const c_char
    );
    ASSERT_EQ!(
        (*(*fexit_skel).bss).test2_result,
        1,
        b"fexit_many_args_result2\0".as_ptr() as *const c_char
    );
    ASSERT_EQ!(
        (*(*fexit_skel).bss).test3_result,
        1,
        b"fexit_many_args_result3\0".as_ptr() as *const c_char
    );

    goto_cleanup_fexit_many_args(fexit_skel);
}

unsafe fn goto_cleanup_fexit_many_args(fexit_skel: *mut fexit_many_args) {
    fexit_many_args__destroy(fexit_skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_fexit_test() {
    if test__start_subtest(b"fexit\0".as_ptr() as *const c_char) {
        fexit_test();
    }
    if test__start_subtest(b"fexit_many_args\0".as_ptr() as *const c_char) {
        fexit_many_args();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
