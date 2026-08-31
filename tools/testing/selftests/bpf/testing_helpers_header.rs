/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */
/* Copyright (C) 2020 Facebook, Inc. */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long};

pub type __u32 = u32;
pub type __u64 = u64;
pub type u64 = __u64;
pub type size_t = usize;

macro_rules! __TO_STR {
    ($x:tt) => {
        stringify!($x)
    };
}

macro_rules! TO_STR {
    ($x:tt) => {
        __TO_STR!($x)
    };
}

pub(crate) use __TO_STR;
pub(crate) use TO_STR;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_insn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_filter_set {
    _private: [u8; 0],
}

/* enum bpf_prog_type is provided by <bpf/bpf.h>. */
pub type bpf_prog_type = c_int;

unsafe extern "C" {
    pub fn parse_num_list(s: *const c_char, set: *mut *mut bool, set_len: *mut c_int) -> c_int;
    pub fn link_info_prog_id(link: *const bpf_link, info: *mut bpf_link_info) -> __u32;
    pub fn bpf_prog_test_load(
        file: *const c_char,
        type_: bpf_prog_type,
        pobj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    pub fn bpf_test_load_program(
        type_: bpf_prog_type,
        insns: *const bpf_insn,
        insns_cnt: size_t,
        license: *const c_char,
        kern_version: __u32,
        log_buf: *mut c_char,
        log_buf_sz: size_t,
    ) -> c_int;
}

/*
 * below function is exported for testing in prog_test test
 */
unsafe extern "C" {
    pub fn parse_test_list(
        s: *const c_char,
        test_set: *mut test_filter_set,
        is_glob_pattern: bool,
    ) -> c_int;
    pub fn parse_test_list_file(
        path: *const c_char,
        test_set: *mut test_filter_set,
        is_glob_pattern: bool,
    ) -> c_int;

    pub fn read_perf_max_sample_freq() -> __u64;
    pub fn load_bpf_testmod(verbose: bool) -> c_int;
    pub fn unload_bpf_testmod(verbose: bool) -> c_int;
    pub fn kern_sync_rcu() -> c_int;
    pub fn finit_module(fd: c_int, param_values: *const c_char, flags: c_int) -> c_int;
    pub fn delete_module(name: *const c_char, flags: c_int) -> c_int;
    pub fn load_module(path: *const c_char, verbose: bool) -> c_int;
    pub fn load_module_params(
        path: *const c_char,
        param_values: *const c_char,
        verbose: bool,
    ) -> c_int;
    pub fn try_unload_module(name: *const c_char, retries: c_int, verbose: bool) -> c_int;
    pub fn unload_module(name: *const c_char, verbose: bool) -> c_int;
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

pub const CLOCK_MONOTONIC: c_int = 1;

unsafe extern "C" {
    pub fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
}

#[inline]
pub unsafe fn get_time_ns() -> __u64 {
    let mut t = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut t);
    }

    (t.tv_sec as u64)
        .wrapping_mul(1000000000)
        .wrapping_add(t.tv_nsec as __u64)
}

/* Request BPF program instructions after all rewrites are applied,
 * e.g. verifier.c:convert_ctx_access() is done.
 */
unsafe extern "C" {
    pub fn get_xlated_program(fd_prog: c_int, buf: *mut *mut bpf_insn, cnt: *mut __u32) -> c_int;
    pub fn testing_prog_flags() -> c_int;
    pub fn is_jit_enabled() -> bool;
    pub fn stack_mprotect() -> c_int;
}
