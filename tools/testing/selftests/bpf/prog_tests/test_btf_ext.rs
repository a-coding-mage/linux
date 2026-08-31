// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms Inc. */
// C dependencies: <test_progs.h>, "test_btf_ext.skel.h", "btf_helpers.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem;

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_btf_ext__progs {
    pub global_func: *mut bpf_program,
}

#[repr(C)]
pub struct test_btf_ext {
    pub progs: test_btf_ext__progs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_line_info {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_func_info {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_info {
    pub line_info: __u64,
    pub nr_line_info: __u32,
    pub line_info_rec_size: __u32,
    pub func_info: __u64,
    pub nr_func_info: __u32,
    pub func_info_rec_size: __u32,
}

unsafe extern "C" {
    fn test_btf_ext__open_and_load() -> *mut test_btf_ext;
    fn test_btf_ext__destroy(skel: *mut test_btf_ext);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__line_info(prog: *mut bpf_program) -> *mut bpf_line_info;
    fn bpf_program__line_info_cnt(prog: *mut bpf_program) -> __u32;
    fn bpf_program__func_info(prog: *mut bpf_program) -> *mut bpf_func_info;
    fn bpf_program__func_info_cnt(prog: *mut bpf_program) -> __u32;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;

    fn ptr_to_u64(ptr: *const c_void) -> __u64;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> *const c_void;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: __u32, expected: __u32, name: *const c_char) -> bool;
    fn ASSERT_MEMEQ(actual: *const c_void, expected: *const c_void, sz: usize, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
}

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn subtest_line_func_info() {
    let mut skel: *mut test_btf_ext;
    let mut info: bpf_prog_info;
    let mut line_info: [bpf_line_info; 128];
    let mut libbpf_line_info: *mut bpf_line_info;
    let mut func_info: [bpf_func_info; 128];
    let mut libbpf_func_info: *mut bpf_func_info;
    let mut info_len: __u32 = mem::size_of::<bpf_prog_info>() as __u32;
    let libbbpf_line_info_cnt: __u32;
    let libbbpf_func_info_cnt: __u32;
    let mut err: c_int;
    let fd: c_int;

    skel = test_btf_ext__open_and_load();
    if ASSERT_OK_PTR(skel as *const c_void, cstr(b"skel_open_and_load\0")).is_null() {
        return;
    }

    fd = bpf_program__fd((*skel).progs.global_func);

    info = mem::zeroed();
    line_info = mem::zeroed();
    info.line_info = ptr_to_u64(line_info.as_mut_ptr() as *const c_void);
    info.nr_line_info = mem::size_of_val(&line_info) as __u32;
    info.line_info_rec_size = mem::size_of::<bpf_line_info>() as __u32;
    err = bpf_prog_get_info_by_fd(fd, &mut info, &mut info_len);
    if !ASSERT_OK(err, cstr(b"prog_line_info\0")) {
        goto_out(skel);
        return;
    }

    libbpf_line_info = bpf_program__line_info((*skel).progs.global_func);
    libbbpf_line_info_cnt = bpf_program__line_info_cnt((*skel).progs.global_func);

    info = mem::zeroed();
    func_info = mem::zeroed();
    info.func_info = ptr_to_u64(func_info.as_mut_ptr() as *const c_void);
    info.nr_func_info = mem::size_of_val(&func_info) as __u32;
    info.func_info_rec_size = mem::size_of::<bpf_func_info>() as __u32;
    err = bpf_prog_get_info_by_fd(fd, &mut info, &mut info_len);
    if !ASSERT_OK(err, cstr(b"prog_func_info\0")) {
        goto_out(skel);
        return;
    }

    libbpf_func_info = bpf_program__func_info((*skel).progs.global_func);
    libbbpf_func_info_cnt = bpf_program__func_info_cnt((*skel).progs.global_func);

    if ASSERT_OK_PTR(libbpf_line_info as *const c_void, cstr(b"bpf_program__line_info\0")).is_null() {
        goto_out(skel);
        return;
    }
    if !ASSERT_EQ(libbbpf_line_info_cnt, info.nr_line_info, cstr(b"line_info_cnt\0")) {
        goto_out(skel);
        return;
    }
    if ASSERT_OK_PTR(libbpf_func_info as *const c_void, cstr(b"bpf_program__func_info\0")).is_null() {
        goto_out(skel);
        return;
    }
    if !ASSERT_EQ(libbbpf_func_info_cnt, info.nr_func_info, cstr(b"func_info_cnt\0")) {
        goto_out(skel);
        return;
    }
    ASSERT_MEMEQ(
        libbpf_line_info as *const c_void,
        line_info.as_ptr() as *const c_void,
        libbbpf_line_info_cnt as usize * mem::size_of::<bpf_line_info>(),
        cstr(b"line_info\0"),
    );
    ASSERT_MEMEQ(
        libbpf_func_info as *const c_void,
        func_info.as_ptr() as *const c_void,
        libbbpf_func_info_cnt as usize * mem::size_of::<bpf_func_info>(),
        cstr(b"func_info\0"),
    );

    goto_out(skel);
}

unsafe fn goto_out(skel: *mut test_btf_ext) {
    test_btf_ext__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_btf_ext() {
    if test__start_subtest(cstr(b"line_func_info\0")) {
        subtest_line_func_info();
    }
}
