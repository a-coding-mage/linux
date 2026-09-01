// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta */

// Translated from:
// #include <test_progs.h>
// #include <network_helpers.h>
// #include "prepare.skel.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const c_void,
    pub data_size_in: u32,
    pub retval: u32,
}

#[repr(C)]
pub struct prepare__progs {
    pub program: *mut bpf_program,
}

#[repr(C)]
pub struct prepare__bss {
    pub err: c_int,
}

#[repr(C)]
pub struct prepare {
    pub obj: *mut bpf_object,
    pub progs: prepare__progs,
    pub bss: *mut prepare__bss,
}

unsafe extern "C" {
    static pkt_v4: [u8; 0];

    fn bpf_object__next_map(
        obj: *const bpf_object,
        prev: *const bpf_map,
    ) -> *const bpf_map;
    fn bpf_map__fd(map: *const bpf_map) -> c_int;
    fn bpf_object__prepare(obj: *mut bpf_object) -> c_int;
    fn bpf_program__fd(prog: *const bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn prepare__open() -> *mut prepare;
    fn prepare__load(skel: *mut prepare) -> c_int;
    fn prepare__destroy(skel: *mut prepare);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_FALSE(cond: bool, name: *const c_char) -> bool;
    fn ASSERT_TRUE(cond: bool, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn check_prepared(obj: *mut bpf_object) -> bool {
    let mut is_prepared = true;
    let mut map: *const bpf_map = core::ptr::null();

    // Equivalent to bpf_object__for_each_map(map, obj).
    loop {
        map = bpf_object__next_map(obj, map);
        if map.is_null() {
            break;
        }

        if bpf_map__fd(map) < 0 {
            is_prepared = false;
        }
    }

    is_prepared
}

unsafe fn test_prepare_no_load() {
    let skel: *mut prepare;
    let err: c_int;
    let mut topts = bpf_test_run_opts {
        data_in: (&raw const pkt_v4) as *const c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as u32,
        retval: 0,
    };

    skel = prepare__open();
    if !ASSERT_OK_PTR(skel as *const c_void, b"prepare__open\0".as_ptr() as *const c_char) {
        return;
    }

    'cleanup: loop {
        if !ASSERT_FALSE(
            check_prepared((*skel).obj),
            b"not check_prepared\0".as_ptr() as *const c_char,
        ) {
            break 'cleanup;
        }

        err = bpf_object__prepare((*skel).obj);

        if !ASSERT_TRUE(
            check_prepared((*skel).obj),
            b"check_prepared\0".as_ptr() as *const c_char,
        ) {
            break 'cleanup;
        }

        if !ASSERT_OK(err, b"bpf_object__prepare\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        break 'cleanup;
    }

    prepare__destroy(skel);
}

unsafe fn test_prepare_load() {
    let skel: *mut prepare;
    let mut err: c_int;
    let prog_fd: c_int;
    let mut topts = bpf_test_run_opts {
        data_in: (&raw const pkt_v4) as *const c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as u32,
        retval: 0,
    };

    skel = prepare__open();
    if !ASSERT_OK_PTR(skel as *const c_void, b"prepare__open\0".as_ptr() as *const c_char) {
        return;
    }

    'cleanup: loop {
        if !ASSERT_FALSE(
            check_prepared((*skel).obj),
            b"not check_prepared\0".as_ptr() as *const c_char,
        ) {
            break 'cleanup;
        }

        err = bpf_object__prepare((*skel).obj);
        if !ASSERT_OK(err, b"bpf_object__prepare\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        err = prepare__load(skel);
        if !ASSERT_OK(err, b"prepare__load\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        if !ASSERT_TRUE(
            check_prepared((*skel).obj),
            b"check_prepared\0".as_ptr() as *const c_char,
        ) {
            break 'cleanup;
        }

        prog_fd = bpf_program__fd((*skel).progs.program);
        if !ASSERT_GE(prog_fd, 0, b"prog_fd\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        if !ASSERT_OK(err, b"test_run_opts err\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        if !ASSERT_OK(topts.retval as c_int, b"test_run_opts retval\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        ASSERT_EQ((*(*skel).bss).err, 0, b"err\0".as_ptr() as *const c_char);

        break 'cleanup;
    }

    prepare__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_prepare() {
    if test__start_subtest(b"prepare_load\0".as_ptr() as *const c_char) {
        test_prepare_load();
    }
    if test__start_subtest(b"prepare_no_load\0".as_ptr() as *const c_char) {
        test_prepare_no_load();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
