// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/*
 * Translated from C implementation source.
 * Original includes supplied errno values, syscall numbers, the generated
 * test_map_ops skeleton, and the test_progs assertion/subtest helpers.
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

const EEXIST: c_int = 17;
const ENOENT: c_int = 2;
const E2BIG: c_int = 7;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct test_map_ops_rodata {
    pub pid: c_int,
}

#[repr(C)]
pub struct test_map_ops_bss {
    pub err: c_int,
}

#[repr(C)]
pub struct test_map_ops {
    pub rodata: *mut test_map_ops_rodata,
    pub bss: *mut test_map_ops_bss,
}

unsafe extern "C" {
    fn syscall(number: c_long, ...) -> c_long;
    fn getpid() -> c_int;

    fn test_map_ops__open() -> *mut test_map_ops;
    fn test_map_ops__load(skel: *mut test_map_ops) -> c_int;
    fn test_map_ops__attach(skel: *mut test_map_ops) -> c_int;
    fn test_map_ops__destroy(skel: *mut test_map_ops);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_LT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
}

#[inline]
unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn map_update() {
    // __NR_getpid is supplied by the platform syscall headers.
    let _ = syscall(libc::__NR_getpid as c_long);
}

unsafe fn map_delete() {
    // __NR_getppid is supplied by the platform syscall headers.
    let _ = syscall(libc::__NR_getppid as c_long);
}

unsafe fn map_push() {
    // __NR_getuid is supplied by the platform syscall headers.
    let _ = syscall(libc::__NR_getuid as c_long);
}

unsafe fn map_pop() {
    // __NR_geteuid is supplied by the platform syscall headers.
    let _ = syscall(libc::__NR_geteuid as c_long);
}

unsafe fn map_peek() {
    // __NR_getgid is supplied by the platform syscall headers.
    let _ = syscall(libc::__NR_getgid as c_long);
}

unsafe fn map_for_each_pass() {
    // __NR_gettid is supplied by the platform syscall headers.
    let _ = syscall(libc::__NR_gettid as c_long);
}

unsafe fn map_for_each_fail() {
    // __NR_getpgid is supplied by the platform syscall headers.
    let _ = syscall(libc::__NR_getpgid as c_long);
}

unsafe fn setup(skel: *mut *mut test_map_ops) -> c_int {
    let mut err: c_int = 0;

    if skel.is_null() {
        return -1;
    }

    *skel = test_map_ops__open();
    if !ASSERT_OK_PTR(*skel as *const c_void, cstr(b"test_map_ops__open\0")) {
        return -1;
    }

    (*(*skel)).rodata.as_mut().unwrap().pid = getpid();

    err = test_map_ops__load(*skel);
    if !ASSERT_OK(err, cstr(b"test_map_ops__load\0")) {
        return err;
    }

    err = test_map_ops__attach(*skel);
    if !ASSERT_OK(err, cstr(b"test_map_ops__attach\0")) {
        return err;
    }

    err
}

unsafe fn teardown(skel: *mut *mut test_map_ops) {
    if !skel.is_null() && !(*skel).is_null() {
        test_map_ops__destroy(*skel);
    }
}

unsafe fn map_ops_update_delete_subtest() {
    let mut skel: *mut test_map_ops = ptr::null_mut();

    if setup(&mut skel) == 0 {
        map_update();
        ASSERT_OK((*(*skel).bss).err, cstr(b"map_update_initial\0"));

        map_update();
        ASSERT_LT((*(*skel).bss).err, 0, cstr(b"map_update_existing\0"));
        ASSERT_EQ((*(*skel).bss).err, -EEXIST, cstr(b"map_update_existing\0"));

        map_delete();
        ASSERT_OK((*(*skel).bss).err, cstr(b"map_delete_existing\0"));

        map_delete();
        ASSERT_LT((*(*skel).bss).err, 0, cstr(b"map_delete_non_existing\0"));
        ASSERT_EQ((*(*skel).bss).err, -ENOENT, cstr(b"map_delete_non_existing\0"));
    }

    teardown(&mut skel);
}

unsafe fn map_ops_push_peek_pop_subtest() {
    let mut skel: *mut test_map_ops = ptr::null_mut();

    if setup(&mut skel) == 0 {
        map_push();
        ASSERT_OK((*(*skel).bss).err, cstr(b"map_push_initial\0"));

        map_push();
        ASSERT_LT((*(*skel).bss).err, 0, cstr(b"map_push_when_full\0"));
        ASSERT_EQ((*(*skel).bss).err, -E2BIG, cstr(b"map_push_when_full\0"));

        map_peek();
        ASSERT_OK((*(*skel).bss).err, cstr(b"map_peek\0"));

        map_pop();
        ASSERT_OK((*(*skel).bss).err, cstr(b"map_pop\0"));

        map_peek();
        ASSERT_LT((*(*skel).bss).err, 0, cstr(b"map_peek_when_empty\0"));
        ASSERT_EQ((*(*skel).bss).err, -ENOENT, cstr(b"map_peek_when_empty\0"));

        map_pop();
        ASSERT_LT((*(*skel).bss).err, 0, cstr(b"map_pop_when_empty\0"));
        ASSERT_EQ((*(*skel).bss).err, -ENOENT, cstr(b"map_pop_when_empty\0"));
    }

    teardown(&mut skel);
}

unsafe fn map_ops_for_each_subtest() {
    let mut skel: *mut test_map_ops = ptr::null_mut();

    if setup(&mut skel) == 0 {
        map_for_each_pass();
        /* expect to iterate over 1 element */
        ASSERT_EQ((*(*skel).bss).err, 1, cstr(b"map_for_each_no_flags\0"));

        map_for_each_fail();
        ASSERT_LT((*(*skel).bss).err, 0, cstr(b"map_for_each_with_flags\0"));
        ASSERT_EQ((*(*skel).bss).err, -EINVAL, cstr(b"map_for_each_with_flags\0"));
    }

    teardown(&mut skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_map_ops() {
    if test__start_subtest(cstr(b"map_ops_update_delete\0")) {
        map_ops_update_delete_subtest();
    }

    if test__start_subtest(cstr(b"map_ops_push_peek_pop\0")) {
        map_ops_push_peek_pop_subtest();
    }

    if test__start_subtest(cstr(b"map_ops_for_each\0")) {
        map_ops_for_each_subtest();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
