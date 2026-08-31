// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// Translated from C implementation source. C includes:
// <sys/wait.h>, <test_progs.h>, <unistd.h>
// "task_kfunc_failure.skel.h", "task_kfunc_success.skel.h"

use core::ffi::{c_char, c_int, c_void, CStr};
use core::ptr;

const CLONE_NEWPID: c_int = 0x20000000;
const SIGCHLD: c_int = 17;

#[repr(C)]
pub struct task_kfunc_success {
    pub obj: *mut bpf_object,
    pub bss: *mut task_kfunc_success__bss,
}

#[repr(C)]
pub struct task_kfunc_success__bss {
    pub pid: c_int,
    pub err: c_int,
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub retval: u32,
}

unsafe extern "C" {
    fn task_kfunc_success__open() -> *mut task_kfunc_success;
    fn task_kfunc_success__load(skel: *mut task_kfunc_success) -> c_int;
    fn task_kfunc_success__destroy(skel: *mut task_kfunc_success);

    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn getpid() -> c_int;
    fn fork() -> c_int;
    fn _exit(status: c_int) -> !;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn clone(
        func: unsafe extern "C" fn(*mut c_void) -> c_int,
        stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn test__start_subtest(name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: u32, expected: u32, name: *const c_char) -> bool;

    fn RUN_TESTS_task_kfunc_failure();
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

unsafe fn open_load_task_kfunc_skel() -> *mut task_kfunc_success {
    let skel: *mut task_kfunc_success;
    let err: c_int;

    skel = task_kfunc_success__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return ptr::null_mut();
    }

    (*(*skel).bss).pid = getpid();

    err = task_kfunc_success__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        task_kfunc_success__destroy(skel);
        return ptr::null_mut();
    }

    skel
}

unsafe fn run_success_test(prog_name: *const c_char) {
    let skel: *mut task_kfunc_success;
    let mut status: c_int = 0;
    let child_pid: c_int;
    let prog: *mut bpf_program;
    let mut link: *mut bpf_link = ptr::null_mut();

    skel = open_load_task_kfunc_skel();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_load_skel".as_ptr()) {
        return;
    }

    if !ASSERT_OK((*(*skel).bss).err, c"pre_spawn_err".as_ptr()) {
        bpf_link__destroy(link);
        task_kfunc_success__destroy(skel);
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR(prog as *const c_void, c"bpf_object__find_program_by_name".as_ptr()) {
        bpf_link__destroy(link);
        task_kfunc_success__destroy(skel);
        return;
    }

    link = bpf_program__attach(prog);
    if !ASSERT_OK_PTR(link as *const c_void, c"attached_link".as_ptr()) {
        bpf_link__destroy(link);
        task_kfunc_success__destroy(skel);
        return;
    }

    child_pid = fork();
    if !ASSERT_GT(child_pid, -1, c"child_pid".as_ptr()) {
        bpf_link__destroy(link);
        task_kfunc_success__destroy(skel);
        return;
    }
    if child_pid == 0 {
        _exit(0);
    }
    waitpid(child_pid, &mut status, 0);

    ASSERT_OK((*(*skel).bss).err, c"post_wait_err".as_ptr());

    bpf_link__destroy(link);
    task_kfunc_success__destroy(skel);
}

unsafe fn run_syscall_success_test(prog_name: *const c_char) {
    let mut opts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        retval: 0,
    };
    let skel: *mut task_kfunc_success;
    let prog: *mut bpf_program;
    let err: c_int;

    skel = open_load_task_kfunc_skel();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_load_skel".as_ptr()) {
        return;
    }

    if !ASSERT_OK((*(*skel).bss).err, c"pre_run_err".as_ptr()) {
        task_kfunc_success__destroy(skel);
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR(prog as *const c_void, c"bpf_object__find_program_by_name".as_ptr()) {
        task_kfunc_success__destroy(skel);
        return;
    }

    err = bpf_prog_test_run_opts(bpf_program__fd(prog), &mut opts);
    if !ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr()) {
        task_kfunc_success__destroy(skel);
        return;
    }
    if !ASSERT_EQ(opts.retval, 0, c"retval".as_ptr()) {
        task_kfunc_success__destroy(skel);
        return;
    }

    ASSERT_OK((*(*skel).bss).err, c"post_run_err".as_ptr());

    task_kfunc_success__destroy(skel);
}

unsafe extern "C" fn run_vpid_test(prog_name: *mut c_void) -> c_int {
    let skel: *mut task_kfunc_success;
    let prog: *mut bpf_program;
    let prog_fd: c_int;
    let mut err: c_int = 0;

    if getpid() != 1 {
        return 1;
    }

    skel = open_load_task_kfunc_skel();
    if skel.is_null() {
        return 2;
    }

    if (*(*skel).bss).err != 0 {
        err = 3;
        task_kfunc_success__destroy(skel);
        return err;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name as *const c_char);
    if prog.is_null() {
        err = 4;
        task_kfunc_success__destroy(skel);
        return err;
    }

    prog_fd = bpf_program__fd(prog);
    if prog_fd < 0 {
        err = 5;
        task_kfunc_success__destroy(skel);
        return err;
    }

    if bpf_prog_test_run_opts(prog_fd, ptr::null_mut()) != 0 {
        err = 6;
        task_kfunc_success__destroy(skel);
        return err;
    }

    if (*(*skel).bss).err != 0 {
        err = 7 + (*(*skel).bss).err;
    }
    task_kfunc_success__destroy(skel);
    err
}

unsafe fn run_vpid_success_test(prog_name: *const c_char) {
    let stack_size: c_int = 1024 * 1024;
    let child_pid: c_int;
    let mut wstatus: c_int = 0;
    let stack: *mut c_char;

    stack = malloc(stack_size as usize) as *mut c_char;
    if !ASSERT_OK_PTR(stack as *const c_void, c"clone_stack".as_ptr()) {
        return;
    }

    child_pid = clone(
        run_vpid_test,
        stack.add(stack_size as usize) as *mut c_void,
        CLONE_NEWPID | SIGCHLD,
        prog_name as *mut c_void,
    );
    if !ASSERT_GT(child_pid, -1, c"child_pid".as_ptr()) {
        free(stack as *mut c_void);
        return;
    }

    if !ASSERT_GT(waitpid(child_pid, &mut wstatus, 0), -1, c"waitpid".as_ptr()) {
        free(stack as *mut c_void);
        return;
    }

    if WEXITSTATUS(wstatus) > 7 {
        ASSERT_OK(WEXITSTATUS(wstatus) - 7, c"vpid_test_failure".as_ptr());
    } else {
        ASSERT_OK(WEXITSTATUS(wstatus), c"run_vpid_test_err".as_ptr());
    }
    free(stack as *mut c_void);
}

static SUCCESS_TESTS: [&CStr; 19] = [
    c"test_task_acquire_release_argument",
    c"test_task_acquire_release_current",
    c"test_task_acquire_leave_in_map",
    c"test_task_map_acquire_release",
    c"test_task_current_acquire_release",
    c"test_task_from_pid_arg",
    c"test_task_from_pid_current",
    c"test_task_from_pid_invalid",
    c"task_kfunc_acquire_trusted_walked",
    c"task_kfunc_acquire_after_spin_unlock_non_sleepable",
    c"task_kfunc_acquire_after_spin_unlock_explicit_rcu",
    c"task_kfunc_acquire_after_spin_unlock_preempt_disabled",
    c"task_kfunc_acquire_after_spin_unlock_irq_disabled",
    c"task_kfunc_acquire_after_rcu_unlock_preempt_disabled",
    c"task_kfunc_acquire_after_rcu_unlock_irq_disabled",
    c"task_kfunc_acquire_after_preempt_enable_explicit_rcu",
    c"task_kfunc_acquire_after_irq_restore_explicit_rcu",
    c"test_task_kfunc_flavor_relo",
    c"test_task_kfunc_flavor_relo_not_found",
];

static SYSCALL_SUCCESS_TESTS: [&CStr; 1] = [
    c"test_task_xchg_release",
];

static VPID_SUCCESS_TESTS: [&CStr; 2] = [
    c"test_task_from_vpid_current",
    c"test_task_from_vpid_invalid",
];

#[no_mangle]
pub unsafe extern "C" fn test_task_kfunc() {
    let mut i: usize;

    i = 0;
    while i < SUCCESS_TESTS.len() {
        if !test__start_subtest(SUCCESS_TESTS[i].as_ptr()) {
            i += 1;
            continue;
        }

        run_success_test(SUCCESS_TESTS[i].as_ptr());
        i += 1;
    }

    i = 0;
    while i < SYSCALL_SUCCESS_TESTS.len() {
        if !test__start_subtest(SYSCALL_SUCCESS_TESTS[i].as_ptr()) {
            i += 1;
            continue;
        }

        run_syscall_success_test(SYSCALL_SUCCESS_TESTS[i].as_ptr());
        i += 1;
    }

    i = 0;
    while i < VPID_SUCCESS_TESTS.len() {
        if !test__start_subtest(VPID_SUCCESS_TESTS[i].as_ptr()) {
            i += 1;
            continue;
        }

        run_vpid_success_test(VPID_SUCCESS_TESTS[i].as_ptr());
        i += 1;
    }

    RUN_TESTS_task_kfunc_failure();
}
