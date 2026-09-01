// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

use core::ffi::{c_char, c_int};

// C dependencies:
// #include <test_progs.h>
// #include "arena_atomics.skel.h"

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: c_int,
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        Self { retval: 0 }
    }
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct arena_atomics_progs {
    pub add: *mut bpf_program,
    pub sub: *mut bpf_program,
    pub and: *mut bpf_program,
    pub or: *mut bpf_program,
    pub xor: *mut bpf_program,
    pub cmpxchg: *mut bpf_program,
    pub xchg: *mut bpf_program,
    pub uaf: *mut bpf_program,
    pub load_acquire: *mut bpf_program,
    pub store_release: *mut bpf_program,
}

#[repr(C)]
pub struct arena_atomics_arena {
    pub add64_value: i64,
    pub add64_result: i64,
    pub add32_value: i32,
    pub add32_result: i32,
    pub add_stack_value_copy: i64,
    pub add_stack_result: i64,
    pub add_noreturn_value: i64,
    pub sub64_value: i64,
    pub sub64_result: i64,
    pub sub32_value: i32,
    pub sub32_result: i32,
    pub sub_stack_value_copy: i64,
    pub sub_stack_result: i64,
    pub sub_noreturn_value: i64,
    pub and64_value: u64,
    pub and32_value: u32,
    pub or64_value: u64,
    pub or32_value: u32,
    pub xor64_value: u64,
    pub xor32_value: u32,
    pub cmpxchg64_value: i64,
    pub cmpxchg64_result_fail: i64,
    pub cmpxchg64_result_succeed: i64,
    pub cmpxchg32_value: i32,
    pub cmpxchg32_result_fail: i32,
    pub cmpxchg32_result_succeed: i32,
    pub xchg64_value: i64,
    pub xchg64_result: i64,
    pub xchg32_value: i32,
    pub xchg32_result: i32,
    pub uaf_recovery_fails: i32,
    pub load_acquire8_result: u8,
    pub load_acquire16_result: u16,
    pub load_acquire32_result: u32,
    pub load_acquire64_result: u64,
    pub store_release8_result: u8,
    pub store_release16_result: u16,
    pub store_release32_result: u32,
    pub store_release64_result: u64,
}

#[repr(C)]
pub struct arena_atomics_data {
    pub skip_lacq_srel_tests: bool,
    pub skip_all_tests: bool,
}

#[repr(C)]
pub struct arena_atomics_bss {
    pub pid: c_int,
}

#[repr(C)]
pub struct arena_atomics {
    pub progs: arena_atomics_progs,
    pub arena: *mut arena_atomics_arena,
    pub data: *mut arena_atomics_data,
    pub bss: *mut arena_atomics_bss,
}

unsafe extern "C" {
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const arena_atomics, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char);

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn getpid() -> c_int;

    fn arena_atomics__open() -> *mut arena_atomics;
    fn arena_atomics__load(skel: *mut arena_atomics) -> c_int;
    fn arena_atomics__destroy(skel: *mut arena_atomics);
}

unsafe fn test_add(skel: *mut arena_atomics) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    /* No need to attach it, just run it directly */
    prog_fd = bpf_program__fd((*skel).progs.add);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).arena).add64_value, 3, c"add64_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).add64_result, 1, c"add64_result".as_ptr());

    ASSERT_EQ((*(*skel).arena).add32_value, 3, c"add32_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).add32_result, 1, c"add32_result".as_ptr());

    ASSERT_EQ((*(*skel).arena).add_stack_value_copy, 3, c"add_stack_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).add_stack_result, 1, c"add_stack_result".as_ptr());

    ASSERT_EQ((*(*skel).arena).add_noreturn_value, 3, c"add_noreturn_value".as_ptr());
}

unsafe fn test_sub(skel: *mut arena_atomics) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    /* No need to attach it, just run it directly */
    prog_fd = bpf_program__fd((*skel).progs.sub);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).arena).sub64_value, -1, c"sub64_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).sub64_result, 1, c"sub64_result".as_ptr());

    ASSERT_EQ((*(*skel).arena).sub32_value, -1, c"sub32_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).sub32_result, 1, c"sub32_result".as_ptr());

    ASSERT_EQ((*(*skel).arena).sub_stack_value_copy, -1, c"sub_stack_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).sub_stack_result, 1, c"sub_stack_result".as_ptr());

    ASSERT_EQ((*(*skel).arena).sub_noreturn_value, -1, c"sub_noreturn_value".as_ptr());
}

unsafe fn test_and(skel: *mut arena_atomics) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    /* No need to attach it, just run it directly */
    prog_fd = bpf_program__fd((*skel).progs.and);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).arena).and64_value, 0x010u64 << 32, c"and64_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).and32_value, 0x010u32, c"and32_value".as_ptr());
}

unsafe fn test_or(skel: *mut arena_atomics) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    /* No need to attach it, just run it directly */
    prog_fd = bpf_program__fd((*skel).progs.or);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).arena).or64_value, 0x111u64 << 32, c"or64_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).or32_value, 0x111u32, c"or32_value".as_ptr());
}

unsafe fn test_xor(skel: *mut arena_atomics) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    /* No need to attach it, just run it directly */
    prog_fd = bpf_program__fd((*skel).progs.xor);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).arena).xor64_value, 0x101u64 << 32, c"xor64_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).xor32_value, 0x101u32, c"xor32_value".as_ptr());
}

unsafe fn test_cmpxchg(skel: *mut arena_atomics) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    /* No need to attach it, just run it directly */
    prog_fd = bpf_program__fd((*skel).progs.cmpxchg);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).arena).cmpxchg64_value, 2, c"cmpxchg64_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).cmpxchg64_result_fail, 1, c"cmpxchg_result_fail".as_ptr());
    ASSERT_EQ((*(*skel).arena).cmpxchg64_result_succeed, 1, c"cmpxchg_result_succeed".as_ptr());

    ASSERT_EQ((*(*skel).arena).cmpxchg32_value, 2, c"lcmpxchg32_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).cmpxchg32_result_fail, 1, c"cmpxchg_result_fail".as_ptr());
    ASSERT_EQ((*(*skel).arena).cmpxchg32_result_succeed, 1, c"cmpxchg_result_succeed".as_ptr());
}

unsafe fn test_xchg(skel: *mut arena_atomics) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    /* No need to attach it, just run it directly */
    prog_fd = bpf_program__fd((*skel).progs.xchg);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).arena).xchg64_value, 2, c"xchg64_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).xchg64_result, 1, c"xchg64_result".as_ptr());

    ASSERT_EQ((*(*skel).arena).xchg32_value, 2, c"xchg32_value".as_ptr());
    ASSERT_EQ((*(*skel).arena).xchg32_result, 1, c"xchg32_result".as_ptr());
}

unsafe fn test_uaf(skel: *mut arena_atomics) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    /* No need to attach it, just run it directly */
    prog_fd = bpf_program__fd((*skel).progs.uaf);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).arena).uaf_recovery_fails, 0, c"uaf_recovery_fails".as_ptr());
}

unsafe fn test_load_acquire(skel: *mut arena_atomics) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    if (*(*skel).data).skip_lacq_srel_tests {
        printf(
            c"%s:SKIP: ENABLE_ATOMICS_TESTS not defined, Clang doesn't support addr_space_cast, and/or JIT doesn't support load-acquire\n".as_ptr(),
            c"test_load_acquire".as_ptr(),
        );
        test__skip();
        return;
    }

    /* No need to attach it, just run it directly */
    prog_fd = bpf_program__fd((*skel).progs.load_acquire);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).arena).load_acquire8_result, 0x12, c"load_acquire8_result".as_ptr());
    ASSERT_EQ((*(*skel).arena).load_acquire16_result, 0x1234, c"load_acquire16_result".as_ptr());
    ASSERT_EQ((*(*skel).arena).load_acquire32_result, 0x12345678, c"load_acquire32_result".as_ptr());
    ASSERT_EQ(
        (*(*skel).arena).load_acquire64_result,
        0x1234567890abcdef,
        c"load_acquire64_result".as_ptr(),
    );
}

unsafe fn test_store_release(skel: *mut arena_atomics) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    if (*(*skel).data).skip_lacq_srel_tests {
        printf(
            c"%s:SKIP: ENABLE_ATOMICS_TESTS not defined, Clang doesn't support addr_space_cast, and/or JIT doesn't support store-release\n".as_ptr(),
            c"test_store_release".as_ptr(),
        );
        test__skip();
        return;
    }

    /* No need to attach it, just run it directly */
    prog_fd = bpf_program__fd((*skel).progs.store_release);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).arena).store_release8_result, 0x12, c"store_release8_result".as_ptr());
    ASSERT_EQ((*(*skel).arena).store_release16_result, 0x1234, c"store_release16_result".as_ptr());
    ASSERT_EQ((*(*skel).arena).store_release32_result, 0x12345678, c"store_release32_result".as_ptr());
    ASSERT_EQ(
        (*(*skel).arena).store_release64_result,
        0x1234567890abcdef,
        c"store_release64_result".as_ptr(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_arena_atomics() {
    let skel: *mut arena_atomics;
    let err: c_int;

    skel = arena_atomics__open();
    if !ASSERT_OK_PTR(skel, c"arena atomics skeleton open".as_ptr()) {
        return;
    }

    if (*(*skel).data).skip_all_tests {
        printf(
            c"%s:SKIP:no ENABLE_ATOMICS_TESTS or no addr_space_cast support in clang".as_ptr(),
            c"serial_test_arena_atomics".as_ptr(),
        );
        test__skip();
        arena_atomics__destroy(skel);
        return;
    }
    err = arena_atomics__load(skel);
    if !ASSERT_OK(err, c"arena atomics skeleton load".as_ptr()) {
        return;
    }
    (*(*skel).bss).pid = getpid();

    if test__start_subtest(c"add".as_ptr()) {
        test_add(skel);
    }
    if test__start_subtest(c"sub".as_ptr()) {
        test_sub(skel);
    }
    if test__start_subtest(c"and".as_ptr()) {
        test_and(skel);
    }
    if test__start_subtest(c"or".as_ptr()) {
        test_or(skel);
    }
    if test__start_subtest(c"xor".as_ptr()) {
        test_xor(skel);
    }
    if test__start_subtest(c"cmpxchg".as_ptr()) {
        test_cmpxchg(skel);
    }
    if test__start_subtest(c"xchg".as_ptr()) {
        test_xchg(skel);
    }
    if test__start_subtest(c"uaf".as_ptr()) {
        test_uaf(skel);
    }
    if test__start_subtest(c"load_acquire".as_ptr()) {
        test_load_acquire(skel);
    }
    if test__start_subtest(c"store_release".as_ptr()) {
        test_store_release(skel);
    }

    arena_atomics__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
