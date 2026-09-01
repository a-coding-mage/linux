/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2021 Hengqi Chen */

/* Translated from:
 * #include <test_progs.h>
 * #include "test_prog_array_init.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct test_prog_array_init {
    pub rodata: *mut test_prog_array_init__rodata,
    pub bss: *mut test_prog_array_init__bss,
    pub progs: test_prog_array_init__progs,
    pub links: test_prog_array_init__links,
}

#[repr(C)]
pub struct test_prog_array_init__rodata {
    pub my_pid: c_int,
}

#[repr(C)]
pub struct test_prog_array_init__bss {
    pub value: c_int,
}

#[repr(C)]
pub struct test_prog_array_init__progs {
    pub entry: *mut bpf_program,
}

#[repr(C)]
pub struct test_prog_array_init__links {
    pub entry: *mut bpf_link,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn test_prog_array_init__open() -> *mut test_prog_array_init;
    fn test_prog_array_init__load(skel: *mut test_prog_array_init) -> c_int;
    fn test_prog_array_init__destroy(skel: *mut test_prog_array_init);
    fn bpf_program__attach_raw_tracepoint(
        prog: *mut bpf_program,
        tp_name: *const c_char,
    ) -> *mut bpf_link;
    fn getpid() -> c_int;
    fn usleep(usec: c_uint) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

const COULD_NOT_OPEN_BPF_OBJECT: &[u8] = b"could not open BPF object\0";
const COULD_NOT_LOAD_BPF_OBJECT: &[u8] = b"could not load BPF object\0";
const COULD_NOT_ATTACH_BPF_PROGRAM: &[u8] = b"could not attach BPF program\0";
const SYS_ENTER: &[u8] = b"sys_enter\0";
const UNEXPECTED_VALUE: &[u8] = b"unexpected value\0";

pub unsafe fn test_prog_array_init() {
    let mut skel: *mut test_prog_array_init;
    let err: c_int;

    skel = test_prog_array_init__open();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        COULD_NOT_OPEN_BPF_OBJECT.as_ptr() as *const c_char,
    ) {
        return;
    }

    (*(*skel).rodata).my_pid = getpid();

    err = test_prog_array_init__load(skel);
    if !ASSERT_OK(
        err,
        COULD_NOT_LOAD_BPF_OBJECT.as_ptr() as *const c_char,
    ) {
        test_prog_array_init__destroy(skel);
        return;
    }

    (*skel).links.entry = bpf_program__attach_raw_tracepoint(
        (*skel).progs.entry,
        SYS_ENTER.as_ptr() as *const c_char,
    );
    if !ASSERT_OK_PTR(
        (*skel).links.entry as *const c_void,
        COULD_NOT_ATTACH_BPF_PROGRAM.as_ptr() as *const c_char,
    ) {
        test_prog_array_init__destroy(skel);
        return;
    }

    usleep(1);

    ASSERT_EQ(
        (*(*skel).bss).value,
        42,
        UNEXPECTED_VALUE.as_ptr() as *const c_char,
    );

    test_prog_array_init__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
