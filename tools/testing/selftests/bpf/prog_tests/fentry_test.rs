// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */
/* Depends on test_progs.h, fentry_test.lskel.h, and fentry_many_args.skel.h. */

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

type __u64 = u64;

const KEY_SPEC_SESSION_KEYRING: c_int = -3;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: u32,
}

#[repr(C)]
pub struct fentry_test_lskel_progs_test1 {
    pub prog_fd: c_int,
}

#[repr(C)]
pub struct fentry_test_lskel_progs {
    pub test1: fentry_test_lskel_progs_test1,
}

#[repr(C)]
pub struct fentry_test_lskel_bss {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fentry_test_lskel {
    pub progs: fentry_test_lskel_progs,
    pub bss: *mut fentry_test_lskel_bss,
    pub keyring_id: c_int,
}

#[repr(C)]
pub struct fentry_many_args_bss {
    pub test1_result: __u64,
    pub test2_result: __u64,
    pub test3_result: __u64,
}

#[repr(C)]
pub struct fentry_many_args {
    pub bss: *mut fentry_many_args_bss,
}

unsafe extern "C" {
    fn fentry_test_lskel__attach(skel: *mut fentry_test_lskel) -> c_int;
    fn fentry_test_lskel__test1__attach(skel: *mut fentry_test_lskel) -> c_int;
    fn fentry_test_lskel__detach(skel: *mut fentry_test_lskel);
    fn fentry_test_lskel__open() -> *mut fentry_test_lskel;
    fn fentry_test_lskel__load(skel: *mut fentry_test_lskel) -> c_int;
    fn fentry_test_lskel__destroy(skel: *mut fentry_test_lskel);

    fn fentry_many_args__open_and_load() -> *mut fentry_many_args;
    fn fentry_many_args__attach(skel: *mut fentry_many_args) -> c_int;
    fn fentry_many_args__destroy(skel: *mut fentry_many_args);

    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn trigger_module_test_read(arg: c_int) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_LT(res: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ<T: Copy>(res: T, expected: T, name: *const c_char) -> bool;
}

unsafe fn fentry_test_common(fentry_skel: *mut fentry_test_lskel) -> c_int {
    let mut err: c_int;
    let prog_fd: c_int;
    let mut i: usize;
    let link_fd: c_int;
    let result: *mut __u64;
    let mut topts: bpf_test_run_opts = mem::zeroed();

    err = fentry_test_lskel__attach(fentry_skel);
    if !ASSERT_OK(err, c"fentry_attach".as_ptr()) {
        return err;
    }

    /* Check that already linked program can't be attached again. */
    link_fd = fentry_test_lskel__test1__attach(fentry_skel);
    if !ASSERT_LT(link_fd, 0, c"fentry_attach_link".as_ptr()) {
        return -1;
    }

    prog_fd = (*fentry_skel).progs.test1.prog_fd;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval, 0, c"test_run".as_ptr());

    result = (*fentry_skel).bss as *mut __u64;
    i = 0;
    while i < mem::size_of_val(&*(*fentry_skel).bss) / mem::size_of::<__u64>() {
        if !ASSERT_EQ(*result.add(i), 1, c"fentry_result".as_ptr()) {
            return -1;
        }
        i += 1;
    }

    fentry_test_lskel__detach(fentry_skel);

    /* zero results for re-attach test */
    ptr::write_bytes(
        (*fentry_skel).bss as *mut c_void,
        0,
        mem::size_of_val(&*(*fentry_skel).bss),
    );
    0
}

unsafe fn fentry_test() {
    let mut fentry_skel: *mut fentry_test_lskel = ptr::null_mut();
    let mut err: c_int;

    fentry_skel = fentry_test_lskel__open();
    if !ASSERT_OK_PTR(fentry_skel, c"fentry_skel_open".as_ptr()) {
        goto_cleanup_fentry_test(fentry_skel);
        return;
    }

    (*fentry_skel).keyring_id = KEY_SPEC_SESSION_KEYRING;
    err = fentry_test_lskel__load(fentry_skel);
    if !ASSERT_OK(err, c"fentry_skel_load".as_ptr()) {
        goto_cleanup_fentry_test(fentry_skel);
        return;
    }

    err = fentry_test_common(fentry_skel);
    if !ASSERT_OK(err, c"fentry_first_attach".as_ptr()) {
        goto_cleanup_fentry_test(fentry_skel);
        return;
    }

    err = fentry_test_common(fentry_skel);
    ASSERT_OK(err, c"fentry_second_attach".as_ptr());

    goto_cleanup_fentry_test(fentry_skel);
}

unsafe fn goto_cleanup_fentry_test(fentry_skel: *mut fentry_test_lskel) {
    fentry_test_lskel__destroy(fentry_skel);
}

unsafe fn fentry_many_args() {
    let mut fentry_skel: *mut fentry_many_args = ptr::null_mut();
    let mut err: c_int;

    fentry_skel = fentry_many_args__open_and_load();
    if !ASSERT_OK_PTR(fentry_skel, c"fentry_many_args_skel_load".as_ptr()) {
        goto_cleanup_fentry_many_args(fentry_skel);
        return;
    }

    err = fentry_many_args__attach(fentry_skel);
    if !ASSERT_OK(err, c"fentry_many_args_attach".as_ptr()) {
        goto_cleanup_fentry_many_args(fentry_skel);
        return;
    }

    ASSERT_OK(trigger_module_test_read(1), c"trigger_read".as_ptr());

    ASSERT_EQ(
        (*(*fentry_skel).bss).test1_result,
        1,
        c"fentry_many_args_result1".as_ptr(),
    );
    ASSERT_EQ(
        (*(*fentry_skel).bss).test2_result,
        1,
        c"fentry_many_args_result2".as_ptr(),
    );
    ASSERT_EQ(
        (*(*fentry_skel).bss).test3_result,
        1,
        c"fentry_many_args_result3".as_ptr(),
    );

    goto_cleanup_fentry_many_args(fentry_skel);
}

unsafe fn goto_cleanup_fentry_many_args(fentry_skel: *mut fentry_many_args) {
    fentry_many_args__destroy(fentry_skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_fentry_test() {
    if test__start_subtest(c"fentry".as_ptr()) {
        fentry_test();
    }
    if test__start_subtest(c"fentry_many_args".as_ptr()) {
        fentry_many_args();
    }
}
