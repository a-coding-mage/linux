// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/* C dependencies translated from:
 * #include <test_progs.h>
 * #include <unistd.h>
 * #include "test_sleepable_tracepoints.skel.h"
 * #include "test_sleepable_tracepoints_fail.skel.h"
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type __u64 = u64;

const PATH_MAX: usize = 4096;
const __NR_getcwd: c_long = 79;
const BPF_F_TEST_RUN_ON_CPU: u32 = 1 << 0;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_sleepable_tracepoints__bss {
    pub target_pid: c_int,
    pub prog_triggered: c_int,
    pub err: c_int,
    pub copied_byte: u8,
}

#[repr(C)]
pub struct test_sleepable_tracepoints__progs {
    pub handle_raw_tp_bare: *mut bpf_program,
    pub handle_tp_bare: *mut bpf_program,
    pub handle_test_run: *mut bpf_program,
    pub handle_sys_enter_tp_btf: *mut bpf_program,
    pub handle_sys_enter_raw_tp: *mut bpf_program,
    pub handle_sys_enter_tp: *mut bpf_program,
    pub handle_sys_exit_tp: *mut bpf_program,
    pub handle_sys_enter_tp_alias: *mut bpf_program,
    pub handle_sys_enter_raw_tp_alias: *mut bpf_program,
    pub handle_raw_tp_non_faultable: *mut bpf_program,
    pub handle_tp_non_syscall: *mut bpf_program,
}

#[repr(C)]
pub struct test_sleepable_tracepoints {
    pub bss: *mut test_sleepable_tracepoints__bss,
    pub progs: test_sleepable_tracepoints__progs,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub retval: u32,
    pub ctx_in: *mut c_void,
    pub ctx_size_in: u32,
    pub flags: u32,
}

unsafe extern "C" {
    fn getpid() -> c_int;
    fn syscall(number: c_long, ...) -> c_long;

    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_program__attach_raw_tracepoint(
        prog: *mut bpf_program,
        tp_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_program__attach_tracepoint(
        prog: *mut bpf_program,
        tp_category: *const c_char,
        tp_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn test_sleepable_tracepoints__open_and_load() -> *mut test_sleepable_tracepoints;
    fn test_sleepable_tracepoints__destroy(skel: *mut test_sleepable_tracepoints);

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_EQ(actual: c_ulong, expected: c_ulong, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *mut c_void, name: *const c_char) -> bool;

    fn RUN_TESTS_test_sleepable_tracepoints_fail();
}

unsafe fn run_test(skel: *mut test_sleepable_tracepoints) {
    let mut buf = [0 as c_char; PATH_MAX];
    buf[0] = b'/' as c_char;

    (*(*skel).bss).target_pid = getpid();
    (*(*skel).bss).prog_triggered = 0;
    (*(*skel).bss).err = 0;
    (*(*skel).bss).copied_byte = 0;

    syscall(__NR_getcwd, buf.as_mut_ptr(), buf.len());

    ASSERT_EQ(
        (*(*skel).bss).prog_triggered as c_ulong,
        1,
        c"prog_triggered".as_ptr(),
    );
    ASSERT_EQ((*(*skel).bss).err as c_ulong, 0, c"err".as_ptr());
    ASSERT_EQ(
        (*(*skel).bss).copied_byte as c_ulong,
        b'/' as c_ulong,
        c"copied_byte".as_ptr(),
    );
}

unsafe fn run_auto_attach_test(
    prog: *mut bpf_program,
    skel: *mut test_sleepable_tracepoints,
) {
    let link: *mut bpf_link;

    link = bpf_program__attach(prog);
    if !ASSERT_OK_PTR(link as *mut c_void, c"prog_attach".as_ptr()) {
        return;
    }

    run_test(skel);
    bpf_link__destroy(link);
}

unsafe fn test_attach_only(prog: *mut bpf_program) {
    let link: *mut bpf_link;

    link = bpf_program__attach(prog);
    if ASSERT_OK_PTR(link as *mut c_void, c"attach".as_ptr()) {
        bpf_link__destroy(link);
    }
}

unsafe fn test_attach_reject(prog: *mut bpf_program) {
    let link: *mut bpf_link;

    link = bpf_program__attach(prog);
    if !ASSERT_ERR_PTR(link as *mut c_void, c"attach_should_fail".as_ptr()) {
        bpf_link__destroy(link);
    }
}

unsafe fn test_raw_tp_bare(skel: *mut test_sleepable_tracepoints) {
    let link: *mut bpf_link;

    link = bpf_program__attach_raw_tracepoint(
        (*skel).progs.handle_raw_tp_bare,
        c"sys_enter".as_ptr(),
    );
    if ASSERT_OK_PTR(link as *mut c_void, c"attach".as_ptr()) {
        bpf_link__destroy(link);
    }
}

unsafe fn test_tp_bare(skel: *mut test_sleepable_tracepoints) {
    let link: *mut bpf_link;

    link = bpf_program__attach_tracepoint(
        (*skel).progs.handle_tp_bare,
        c"syscalls".as_ptr(),
        c"sys_enter_getcwd".as_ptr(),
    );
    if ASSERT_OK_PTR(link as *mut c_void, c"attach".as_ptr()) {
        bpf_link__destroy(link);
    }
}

unsafe fn test_test_run(skel: *mut test_sleepable_tracepoints) {
    let mut args: [__u64; 2] = [0x1234_u64, 0x5678_u64];
    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        retval: 0,
        ctx_in: args.as_mut_ptr() as *mut c_void,
        ctx_size_in: core::mem::size_of_val(&args) as u32,
        flags: 0,
    };
    let fd: c_int;
    let err: c_int;

    fd = bpf_program__fd((*skel).progs.handle_test_run);
    err = bpf_prog_test_run_opts(fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(
        topts.retval as c_ulong,
        args[0].wrapping_add(args[1]) as c_ulong,
        c"test_run_retval".as_ptr(),
    );
}

unsafe fn test_test_run_on_cpu_reject(skel: *mut test_sleepable_tracepoints) {
    let mut args: [__u64; 2] = [0; 2];
    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        retval: 0,
        ctx_in: args.as_mut_ptr() as *mut c_void,
        ctx_size_in: core::mem::size_of_val(&args) as u32,
        flags: BPF_F_TEST_RUN_ON_CPU,
    };
    let fd: c_int;
    let err: c_int;

    fd = bpf_program__fd((*skel).progs.handle_test_run);
    err = bpf_prog_test_run_opts(fd, &mut topts);
    ASSERT_ERR(err, c"test_run_on_cpu_reject".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn test_sleepable_tracepoints() {
    let skel: *mut test_sleepable_tracepoints;

    skel = test_sleepable_tracepoints__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"open_and_load".as_ptr()) {
        return;
    }

    if test__start_subtest(c"tp_btf".as_ptr()) {
        run_auto_attach_test((*skel).progs.handle_sys_enter_tp_btf, skel);
    }
    if test__start_subtest(c"raw_tp".as_ptr()) {
        run_auto_attach_test((*skel).progs.handle_sys_enter_raw_tp, skel);
    }
    if test__start_subtest(c"tracepoint".as_ptr()) {
        run_auto_attach_test((*skel).progs.handle_sys_enter_tp, skel);
    }
    if test__start_subtest(c"sys_exit".as_ptr()) {
        run_auto_attach_test((*skel).progs.handle_sys_exit_tp, skel);
    }
    if test__start_subtest(c"tracepoint_alias".as_ptr()) {
        test_attach_only((*skel).progs.handle_sys_enter_tp_alias);
    }
    if test__start_subtest(c"raw_tracepoint_alias".as_ptr()) {
        test_attach_only((*skel).progs.handle_sys_enter_raw_tp_alias);
    }
    if test__start_subtest(c"raw_tp_bare".as_ptr()) {
        test_raw_tp_bare(skel);
    }
    if test__start_subtest(c"tp_bare".as_ptr()) {
        test_tp_bare(skel);
    }
    if test__start_subtest(c"test_run".as_ptr()) {
        test_test_run(skel);
    }
    if test__start_subtest(c"test_run_on_cpu_reject".as_ptr()) {
        test_test_run_on_cpu_reject(skel);
    }
    if test__start_subtest(c"raw_tp_non_faultable".as_ptr()) {
        test_attach_reject((*skel).progs.handle_raw_tp_non_faultable);
    }
    if test__start_subtest(c"tp_non_syscall".as_ptr()) {
        test_attach_reject((*skel).progs.handle_tp_non_syscall);
    }
    if test__start_subtest(c"tp_btf_non_faultable_reject".as_ptr()) {
        RUN_TESTS_test_sleepable_tracepoints_fail();
    }

    test_sleepable_tracepoints__destroy(skel);
}
