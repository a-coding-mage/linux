// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies: <test_progs.h>, <sys/syscall.h>, "linked_vars.skel.h"

use core::ffi::{c_char, c_int, c_long};

#[repr(C)]
pub struct linked_vars_bss {
    pub input_bss1: c_int,
    pub input_bss2: c_int,
    pub input_bss_weak: c_int,
    pub output_bss1: c_int,
    pub output_bss2: c_int,
    pub output_data1: c_int,
    pub output_data2: c_int,
    pub output_rodata1: c_int,
    pub output_rodata2: c_int,
}

#[repr(C)]
pub struct linked_vars {
    pub bss: *mut linked_vars_bss,
}

unsafe extern "C" {
    fn linked_vars__open() -> *mut linked_vars;
    fn linked_vars__load(skel: *mut linked_vars) -> c_int;
    fn linked_vars__attach(skel: *mut linked_vars) -> c_int;
    fn linked_vars__destroy(skel: *mut linked_vars);

    fn ASSERT_OK_PTR(ptr: *const linked_vars, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;

    fn syscall(number: c_long) -> c_long;
}

// From <sys/syscall.h>; actual value is target-architecture dependent.
const SYS_getpgid: c_long = 121;

pub unsafe fn test_linked_vars() {
    let mut err: c_int;
    let skel: *mut linked_vars;

    skel = linked_vars__open();
    if !ASSERT_OK_PTR(skel, c"skel_open".as_ptr()) {
        return;
    }

    (*(*skel).bss).input_bss1 = 1000;
    (*(*skel).bss).input_bss2 = 2000;
    (*(*skel).bss).input_bss_weak = 3000;

    err = linked_vars__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        linked_vars__destroy(skel);
        return;
    }

    err = linked_vars__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        linked_vars__destroy(skel);
        return;
    }

    /* trigger */
    syscall(SYS_getpgid);

    ASSERT_EQ(
        (*(*skel).bss).output_bss1,
        1000 + 2000 + 3000,
        c"output_bss1".as_ptr(),
    );
    ASSERT_EQ(
        (*(*skel).bss).output_bss2,
        1000 + 2000 + 3000,
        c"output_bss2".as_ptr(),
    );
    /* 10 comes from "winner" input_data_weak in first obj file */
    ASSERT_EQ(
        (*(*skel).bss).output_data1,
        1 + 2 + 10,
        c"output_bss1".as_ptr(),
    );
    ASSERT_EQ(
        (*(*skel).bss).output_data2,
        1 + 2 + 10,
        c"output_bss2".as_ptr(),
    );
    /* 100 comes from "winner" input_rodata_weak in first obj file */
    ASSERT_EQ(
        (*(*skel).bss).output_rodata1,
        11 + 22 + 100,
        c"output_weak1".as_ptr(),
    );
    ASSERT_EQ(
        (*(*skel).bss).output_rodata2,
        11 + 22 + 100,
        c"output_weak2".as_ptr(),
    );

    linked_vars__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
