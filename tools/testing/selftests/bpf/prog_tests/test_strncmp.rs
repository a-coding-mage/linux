// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2021. Huawei Technologies Co., Ltd */

// Dependencies from C includes:
// #include <test_progs.h>
// #include "strncmp_test.skel.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct strncmp_test {
    pub bss: *mut strncmp_test__bss,
    pub rodata: *mut strncmp_test__rodata,
    pub progs: strncmp_test__progs,
}

#[repr(C)]
pub struct strncmp_test__bss {
    pub cmp_ret: c_int,
    pub str: [c_char; 0],
    pub target_pid: c_int,
}

#[repr(C)]
pub struct strncmp_test__rodata {
    pub target: [c_char; 0],
}

#[repr(C)]
pub struct strncmp_test__progs {
    pub do_strncmp: *mut c_void,
    pub strncmp_bad_not_const_str_size: *mut c_void,
    pub strncmp_bad_writable_target: *mut c_void,
    pub strncmp_bad_not_null_term_target: *mut c_void,
}

unsafe extern "C" {
    fn usleep(usec: u32) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn getpid() -> c_int;

    fn strncmp_test__open() -> *mut strncmp_test;
    fn strncmp_test__load(skel: *mut strncmp_test) -> c_int;
    fn strncmp_test__attach(skel: *mut strncmp_test) -> c_int;
    fn strncmp_test__destroy(skel: *mut strncmp_test);

    fn bpf_program__set_autoload(prog: *mut c_void, autoload: bool);

    fn ASSERT_OK_PTR(ptr: *mut strncmp_test, name: *const c_char) -> bool;
    fn ASSERT_EQ(got: c_int, exp: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
}

const STRNCMP_TEST_BSS_STR_SIZE: usize = core::mem::size_of::<[c_char; 0]>();

unsafe fn trigger_strncmp(skel: *const strncmp_test) -> c_int {
    let cmp: c_int;

    unsafe {
        usleep(1);

        cmp = (*(*skel).bss).cmp_ret;
    }
    if cmp > 0 {
        return 1;
    }
    if cmp < 0 {
        return -1;
    }
    return 0;
}

/*
 * Compare str and target after making str[i] != target[i].
 * When exp is -1, make str[i] < target[i] and delta = -1.
 */
unsafe fn strncmp_full_str_cmp(skel: *mut strncmp_test, name: *const c_char, exp: c_int) {
    let nr: usize = STRNCMP_TEST_BSS_STR_SIZE;
    let str_: *mut c_char = unsafe { (*(*skel).bss).str.as_mut_ptr() };
    let delta: c_int = exp;
    let mut got: c_int;
    let mut i: usize;

    unsafe {
        memcpy(
            str_ as *mut c_void,
            (*(*skel).rodata).target.as_ptr() as *const c_void,
            nr,
        );
    }
    i = 0;
    while i < nr - 1 {
        unsafe {
            *str_.add(i) = ((*str_.add(i) as c_int) + delta) as c_char;

            got = trigger_strncmp(skel);
            ASSERT_EQ(got, exp, name);

            *str_.add(i) = ((*str_.add(i) as c_int) - delta) as c_char;
        }
        i += 1;
    }
}

unsafe fn test_strncmp_ret() {
    let skel: *mut strncmp_test;
    let mut err: c_int;
    let mut got: c_int;

    unsafe {
        skel = strncmp_test__open();
        if !ASSERT_OK_PTR(skel, c"strncmp_test open".as_ptr()) {
            return;
        }

        bpf_program__set_autoload((*skel).progs.do_strncmp, true);

        err = strncmp_test__load(skel);
        if !ASSERT_EQ(err, 0, c"strncmp_test load".as_ptr()) {
            strncmp_test__destroy(skel);
            return;
        }

        err = strncmp_test__attach(skel);
        if !ASSERT_EQ(err, 0, c"strncmp_test attach".as_ptr()) {
            strncmp_test__destroy(skel);
            return;
        }

        (*(*skel).bss).target_pid = getpid();

        /* Empty str */
        (*(*skel).bss).str[0] = b'\0' as c_char;
        got = trigger_strncmp(skel);
        ASSERT_EQ(got, -1, c"strncmp: empty str".as_ptr());

        /* Same string */
        memcpy(
            (*(*skel).bss).str.as_mut_ptr() as *mut c_void,
            (*(*skel).rodata).target.as_ptr() as *const c_void,
            STRNCMP_TEST_BSS_STR_SIZE,
        );
        got = trigger_strncmp(skel);
        ASSERT_EQ(got, 0, c"strncmp: same str".as_ptr());

        /* Not-null-terminated string  */
        memcpy(
            (*(*skel).bss).str.as_mut_ptr() as *mut c_void,
            (*(*skel).rodata).target.as_ptr() as *const c_void,
            STRNCMP_TEST_BSS_STR_SIZE,
        );
        (*(*skel).bss).str[STRNCMP_TEST_BSS_STR_SIZE - 1] = b'A' as c_char;
        got = trigger_strncmp(skel);
        ASSERT_EQ(got, 1, c"strncmp: not-null-term str".as_ptr());

        strncmp_full_str_cmp(skel, c"strncmp: less than".as_ptr(), -1);
        strncmp_full_str_cmp(skel, c"strncmp: greater than".as_ptr(), 1);

        strncmp_test__destroy(skel);
    }
}

unsafe fn test_strncmp_bad_not_const_str_size() {
    let skel: *mut strncmp_test;
    let err: c_int;

    unsafe {
        skel = strncmp_test__open();
        if !ASSERT_OK_PTR(skel, c"strncmp_test open".as_ptr()) {
            return;
        }

        bpf_program__set_autoload((*skel).progs.strncmp_bad_not_const_str_size, true);

        err = strncmp_test__load(skel);
        ASSERT_ERR(err, c"strncmp_test load bad_not_const_str_size".as_ptr());

        strncmp_test__destroy(skel);
    }
}

unsafe fn test_strncmp_bad_writable_target() {
    let skel: *mut strncmp_test;
    let err: c_int;

    unsafe {
        skel = strncmp_test__open();
        if !ASSERT_OK_PTR(skel, c"strncmp_test open".as_ptr()) {
            return;
        }

        bpf_program__set_autoload((*skel).progs.strncmp_bad_writable_target, true);

        err = strncmp_test__load(skel);
        ASSERT_ERR(err, c"strncmp_test load bad_writable_target".as_ptr());

        strncmp_test__destroy(skel);
    }
}

unsafe fn test_strncmp_bad_not_null_term_target() {
    let skel: *mut strncmp_test;
    let err: c_int;

    unsafe {
        skel = strncmp_test__open();
        if !ASSERT_OK_PTR(skel, c"strncmp_test open".as_ptr()) {
            return;
        }

        bpf_program__set_autoload((*skel).progs.strncmp_bad_not_null_term_target, true);

        err = strncmp_test__load(skel);
        ASSERT_ERR(err, c"strncmp_test load bad_not_null_term_target".as_ptr());

        strncmp_test__destroy(skel);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_test_strncmp() {
    unsafe {
        if test__start_subtest(c"strncmp_ret".as_ptr()) {
            test_strncmp_ret();
        }
        if test__start_subtest(c"strncmp_bad_not_const_str_size".as_ptr()) {
            test_strncmp_bad_not_const_str_size();
        }
        if test__start_subtest(c"strncmp_bad_writable_target".as_ptr()) {
            test_strncmp_bad_writable_target();
        }
        if test__start_subtest(c"strncmp_bad_not_null_term_target".as_ptr()) {
            test_strncmp_bad_not_null_term_target();
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
