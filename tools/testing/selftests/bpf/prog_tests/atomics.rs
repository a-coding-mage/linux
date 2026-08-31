// SPDX-License-Identifier: GPL-2.0

// Translated from C includes:
// #include <test_progs.h>
// #include "atomics.lskel.h"

use core::ffi::{c_char, c_int};

const KEY_SPEC_SESSION_KEYRING: c_int = -3;

#[repr(C)]
pub struct bpf_test_run_opts {
    // LIBBPF_OPTS initializes the full C struct, including fields not used here.
    pub retval: c_int,
}

#[repr(C)]
pub struct atomics_lskel {
    pub progs: atomics_lskel_progs,
    pub data: *mut atomics_lskel_data,
    pub bss: *mut atomics_lskel_bss,
    pub keyring_id: c_int,
}

#[repr(C)]
pub struct atomics_lskel_progs {
    pub add: atomics_lskel_prog,
    pub sub: atomics_lskel_prog,
    pub and: atomics_lskel_prog,
    pub or: atomics_lskel_prog,
    pub xor: atomics_lskel_prog,
    pub cmpxchg: atomics_lskel_prog,
    pub xchg: atomics_lskel_prog,
}

#[repr(C)]
pub struct atomics_lskel_prog {
    pub prog_fd: c_int,
}

#[repr(C)]
pub struct atomics_lskel_data {
    pub add64_value: i64,
    pub add32_value: i32,
    pub add_noreturn_value: i64,
    pub sub64_value: i64,
    pub sub32_value: i32,
    pub sub_noreturn_value: i64,
    pub and64_value: u64,
    pub and32_value: u32,
    pub and_noreturn_value: u64,
    pub or64_value: u64,
    pub or32_value: u32,
    pub or_noreturn_value: u64,
    pub xor64_value: u64,
    pub xor32_value: u32,
    pub xor_noreturn_value: u64,
    pub cmpxchg64_value: i64,
    pub cmpxchg32_value: i32,
    pub xchg64_value: i64,
    pub xchg32_value: i32,
    pub skip_tests: bool,
}

#[repr(C)]
pub struct atomics_lskel_bss {
    pub add64_result: i64,
    pub add32_result: i32,
    pub add_stack_value_copy: i64,
    pub add_stack_result: i64,
    pub sub64_result: i64,
    pub sub32_result: i32,
    pub sub_stack_value_copy: i64,
    pub sub_stack_result: i64,
    pub and64_result: u64,
    pub and32_result: u32,
    pub or64_result: u64,
    pub or32_result: u32,
    pub xor64_result: u64,
    pub xor32_result: u32,
    pub cmpxchg64_result_fail: i64,
    pub cmpxchg64_result_succeed: i64,
    pub cmpxchg32_result_fail: i32,
    pub cmpxchg32_result_succeed: i32,
    pub xchg64_result: i64,
    pub xchg32_result: i32,
    pub pid: c_int,
}

unsafe extern "C" {
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn atomics_lskel__open() -> *mut atomics_lskel;
    fn atomics_lskel__load(skel: *mut atomics_lskel) -> c_int;
    fn atomics_lskel__destroy(skel: *mut atomics_lskel);
    fn getpid() -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const atomics_lskel, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
}

unsafe fn test_add(skel: *mut atomics_lskel) {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    /* No need to attach it, just run it directly */
    let prog_fd: c_int = (*skel).progs.add.prog_fd;
    let err: c_int = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).data).add64_value, 3, c"add64_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).add64_result, 1, c"add64_result".as_ptr());

    ASSERT_EQ((*(*skel).data).add32_value, 3, c"add32_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).add32_result, 1, c"add32_result".as_ptr());

    ASSERT_EQ((*(*skel).bss).add_stack_value_copy, 3, c"add_stack_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).add_stack_result, 1, c"add_stack_result".as_ptr());

    ASSERT_EQ((*(*skel).data).add_noreturn_value, 3, c"add_noreturn_value".as_ptr());
}

unsafe fn test_sub(skel: *mut atomics_lskel) {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    /* No need to attach it, just run it directly */
    let prog_fd: c_int = (*skel).progs.sub.prog_fd;
    let err: c_int = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).data).sub64_value, -1, c"sub64_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).sub64_result, 1, c"sub64_result".as_ptr());

    ASSERT_EQ((*(*skel).data).sub32_value, -1, c"sub32_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).sub32_result, 1, c"sub32_result".as_ptr());

    ASSERT_EQ((*(*skel).bss).sub_stack_value_copy, -1, c"sub_stack_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).sub_stack_result, 1, c"sub_stack_result".as_ptr());

    ASSERT_EQ((*(*skel).data).sub_noreturn_value, -1, c"sub_noreturn_value".as_ptr());
}

unsafe fn test_and(skel: *mut atomics_lskel) {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    /* No need to attach it, just run it directly */
    let prog_fd: c_int = (*skel).progs.and.prog_fd;
    let err: c_int = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).data).and64_value, 0x010u64 << 32, c"and64_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).and64_result, 0x110u64 << 32, c"and64_result".as_ptr());

    ASSERT_EQ((*(*skel).data).and32_value, 0x010, c"and32_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).and32_result, 0x110, c"and32_result".as_ptr());

    ASSERT_EQ(
        (*(*skel).data).and_noreturn_value,
        0x010u64 << 32,
        c"and_noreturn_value".as_ptr(),
    );
}

unsafe fn test_or(skel: *mut atomics_lskel) {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    /* No need to attach it, just run it directly */
    let prog_fd: c_int = (*skel).progs.or.prog_fd;
    let err: c_int = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).data).or64_value, 0x111u64 << 32, c"or64_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).or64_result, 0x110u64 << 32, c"or64_result".as_ptr());

    ASSERT_EQ((*(*skel).data).or32_value, 0x111, c"or32_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).or32_result, 0x110, c"or32_result".as_ptr());

    ASSERT_EQ(
        (*(*skel).data).or_noreturn_value,
        0x111u64 << 32,
        c"or_noreturn_value".as_ptr(),
    );
}

unsafe fn test_xor(skel: *mut atomics_lskel) {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    /* No need to attach it, just run it directly */
    let prog_fd: c_int = (*skel).progs.xor.prog_fd;
    let err: c_int = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).data).xor64_value, 0x101u64 << 32, c"xor64_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).xor64_result, 0x110u64 << 32, c"xor64_result".as_ptr());

    ASSERT_EQ((*(*skel).data).xor32_value, 0x101, c"xor32_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).xor32_result, 0x110, c"xor32_result".as_ptr());

    ASSERT_EQ(
        (*(*skel).data).xor_noreturn_value,
        0x101u64 << 32,
        c"xor_nxoreturn_value".as_ptr(),
    );
}

unsafe fn test_cmpxchg(skel: *mut atomics_lskel) {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    /* No need to attach it, just run it directly */
    let prog_fd: c_int = (*skel).progs.cmpxchg.prog_fd;
    let err: c_int = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).data).cmpxchg64_value, 2, c"cmpxchg64_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).cmpxchg64_result_fail, 1, c"cmpxchg_result_fail".as_ptr());
    ASSERT_EQ(
        (*(*skel).bss).cmpxchg64_result_succeed,
        1,
        c"cmpxchg_result_succeed".as_ptr(),
    );

    ASSERT_EQ((*(*skel).data).cmpxchg32_value, 2, c"lcmpxchg32_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).cmpxchg32_result_fail, 1, c"cmpxchg_result_fail".as_ptr());
    ASSERT_EQ(
        (*(*skel).bss).cmpxchg32_result_succeed,
        1,
        c"cmpxchg_result_succeed".as_ptr(),
    );
}

unsafe fn test_xchg(skel: *mut atomics_lskel) {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    /* No need to attach it, just run it directly */
    let prog_fd: c_int = (*skel).progs.xchg.prog_fd;
    let err: c_int = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return;
    }

    ASSERT_EQ((*(*skel).data).xchg64_value, 2, c"xchg64_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).xchg64_result, 1, c"xchg64_result".as_ptr());

    ASSERT_EQ((*(*skel).data).xchg32_value, 2, c"xchg32_value".as_ptr());
    ASSERT_EQ((*(*skel).bss).xchg32_result, 1, c"xchg32_result".as_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_atomics() {
    let skel: *mut atomics_lskel;
    let err: c_int;

    skel = atomics_lskel__open();
    if !ASSERT_OK_PTR(skel, c"atomics skeleton open".as_ptr()) {
        return;
    }

    (*skel).keyring_id = KEY_SPEC_SESSION_KEYRING;
    err = atomics_lskel__load(skel);
    if !ASSERT_OK(err, c"atomics skeleton load".as_ptr()) {
        atomics_lskel__destroy(skel);
        return;
    }

    if (*(*skel).data).skip_tests {
        printf(
            c"%s:SKIP:no ENABLE_ATOMICS_TESTS (missing Clang BPF atomics support)".as_ptr(),
            c"test_atomics".as_ptr(),
        );
        test__skip();
        atomics_lskel__destroy(skel);
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

    atomics_lskel__destroy(skel);
}
