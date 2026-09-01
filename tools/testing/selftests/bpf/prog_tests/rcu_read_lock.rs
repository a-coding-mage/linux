// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates.*/

// C dependencies translated as external declarations:
// <unistd.h>, <sys/syscall.h>, <sys/types.h>, <test_progs.h>, <bpf/btf.h>,
// "rcu_read_lock.skel.h", "cgroup_helpers.h"

use core::ffi::{c_char, c_int, c_long, c_ulonglong, c_void};

const SYS_GETPGID: c_long = 121;

#[repr(C)]
pub struct rcu_read_lock_bss {
    pub target_pid: c_int,
    pub task_storage_val: c_int,
    pub cgroup_id: c_ulonglong,
}

#[repr(C)]
pub struct rcu_read_lock_progs {
    pub get_cgroup_id: *mut bpf_program,
    pub task_succ: *mut bpf_program,
    pub two_regions: *mut bpf_program,
    pub non_sleepable_1: *mut bpf_program,
    pub non_sleepable_2: *mut bpf_program,
    pub nested_rcu_region: *mut bpf_program,
    pub task_trusted_non_rcuptr: *mut bpf_program,
    pub rcu_read_lock_subprog: *mut bpf_program,
    pub rcu_read_lock_global_subprog: *mut bpf_program,
    pub rcu_read_lock_subprog_lock: *mut bpf_program,
    pub rcu_read_lock_subprog_unlock: *mut bpf_program,
    pub non_own_ref_untrusted_ld: *mut bpf_program,
    pub rcu_untrusted_union_ld: *mut bpf_program,
    pub task_acquire: *mut bpf_program,
}

#[repr(C)]
pub struct rcu_read_lock {
    pub obj: *mut bpf_object,
    pub progs: rcu_read_lock_progs,
    pub bss: *mut rcu_read_lock_bss,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn rcu_read_lock__open() -> *mut rcu_read_lock;
    fn rcu_read_lock__load(skel: *mut rcu_read_lock) -> c_int;
    fn rcu_read_lock__attach(skel: *mut rcu_read_lock) -> c_int;
    fn rcu_read_lock__destroy(skel: *mut rcu_read_lock);

    fn sys_gettid() -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn get_cgroup_id(path: *const c_char) -> c_ulonglong;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_ulonglong, expected: c_ulonglong, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

static mut cgroup_id: c_ulonglong = 0;

unsafe fn test_success() {
    let skel: *mut rcu_read_lock;
    let mut err: c_int;

    skel = rcu_read_lock__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    (*(*skel).bss).target_pid = sys_gettid();

    bpf_program__set_autoload((*skel).progs.get_cgroup_id, true);
    bpf_program__set_autoload((*skel).progs.task_succ, true);
    bpf_program__set_autoload((*skel).progs.two_regions, true);
    bpf_program__set_autoload((*skel).progs.non_sleepable_1, true);
    bpf_program__set_autoload((*skel).progs.non_sleepable_2, true);
    bpf_program__set_autoload((*skel).progs.nested_rcu_region, true);
    bpf_program__set_autoload((*skel).progs.task_trusted_non_rcuptr, true);
    bpf_program__set_autoload((*skel).progs.rcu_read_lock_subprog, true);
    bpf_program__set_autoload((*skel).progs.rcu_read_lock_global_subprog, true);
    bpf_program__set_autoload((*skel).progs.rcu_read_lock_subprog_lock, true);
    bpf_program__set_autoload((*skel).progs.rcu_read_lock_subprog_unlock, true);
    bpf_program__set_autoload((*skel).progs.non_own_ref_untrusted_ld, true);
    bpf_program__set_autoload((*skel).progs.rcu_untrusted_union_ld, true);
    err = rcu_read_lock__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        rcu_read_lock__destroy(skel);
        return;
    }

    err = rcu_read_lock__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        rcu_read_lock__destroy(skel);
        return;
    }

    syscall(SYS_GETPGID);

    ASSERT_EQ(
        (*(*skel).bss).task_storage_val as c_ulonglong,
        2,
        c"task_storage_val".as_ptr(),
    );
    ASSERT_EQ(
        (*(*skel).bss).cgroup_id,
        cgroup_id,
        c"cgroup_id".as_ptr(),
    );

    rcu_read_lock__destroy(skel);
}

unsafe fn test_rcuptr_acquire() {
    let skel: *mut rcu_read_lock;
    let mut err: c_int;

    skel = rcu_read_lock__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    (*(*skel).bss).target_pid = sys_gettid();

    bpf_program__set_autoload((*skel).progs.task_acquire, true);
    err = rcu_read_lock__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        rcu_read_lock__destroy(skel);
        return;
    }

    err = rcu_read_lock__attach(skel);
    ASSERT_OK(err, c"skel_attach".as_ptr());

    rcu_read_lock__destroy(skel);
}

static inproper_region_tests: [*const c_char; 13] = [
    c"miss_lock".as_ptr(),
    c"no_lock".as_ptr(),
    c"miss_unlock".as_ptr(),
    c"non_sleepable_rcu_mismatch".as_ptr(),
    c"inproper_sleepable_helper".as_ptr(),
    c"inproper_sleepable_kfunc".as_ptr(),
    c"nested_rcu_region_unbalanced_1".as_ptr(),
    c"nested_rcu_region_unbalanced_2".as_ptr(),
    c"rcu_read_lock_global_subprog_lock".as_ptr(),
    c"rcu_read_lock_global_subprog_unlock".as_ptr(),
    c"rcu_read_lock_sleepable_helper_global_subprog".as_ptr(),
    c"rcu_read_lock_sleepable_kfunc_global_subprog".as_ptr(),
    c"rcu_read_lock_sleepable_global_subprog_indirect".as_ptr(),
];

unsafe fn test_inproper_region() {
    let mut skel: *mut rcu_read_lock;
    let mut prog: *mut bpf_program;
    let mut err: c_int;

    for i in 0..inproper_region_tests.len() {
        skel = rcu_read_lock__open();
        if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
            return;
        }

        prog = bpf_object__find_program_by_name((*skel).obj, inproper_region_tests[i]);
        if !ASSERT_OK_PTR(prog as *const c_void, c"bpf_object__find_program_by_name".as_ptr()) {
            rcu_read_lock__destroy(skel);
            continue;
        }
        bpf_program__set_autoload(prog, true);
        err = rcu_read_lock__load(skel);
        ASSERT_ERR(err, c"skel_load".as_ptr());

        rcu_read_lock__destroy(skel);
    }
}

static rcuptr_misuse_tests: [*const c_char; 2] = [
    c"task_untrusted_rcuptr".as_ptr(),
    c"cross_rcu_region".as_ptr(),
];

unsafe fn test_rcuptr_misuse() {
    let mut skel: *mut rcu_read_lock;
    let mut prog: *mut bpf_program;
    let mut err: c_int;

    for i in 0..rcuptr_misuse_tests.len() {
        skel = rcu_read_lock__open();
        if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
            return;
        }

        prog = bpf_object__find_program_by_name((*skel).obj, rcuptr_misuse_tests[i]);
        if !ASSERT_OK_PTR(prog as *const c_void, c"bpf_object__find_program_by_name".as_ptr()) {
            rcu_read_lock__destroy(skel);
            continue;
        }
        bpf_program__set_autoload(prog, true);
        err = rcu_read_lock__load(skel);
        ASSERT_ERR(err, c"skel_load".as_ptr());

        rcu_read_lock__destroy(skel);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_rcu_read_lock() {
    let cgroup_fd: c_int;

    cgroup_fd = test__join_cgroup(c"/rcu_read_lock".as_ptr());
    if !ASSERT_GE(cgroup_fd, 0, c"join_cgroup /rcu_read_lock".as_ptr()) {
        return;
    }

    cgroup_id = get_cgroup_id(c"/rcu_read_lock".as_ptr());
    if test__start_subtest(c"success".as_ptr()) {
        test_success();
    }
    if test__start_subtest(c"rcuptr_acquire".as_ptr()) {
        test_rcuptr_acquire();
    }
    if test__start_subtest(c"negative_tests_inproper_region".as_ptr()) {
        test_inproper_region();
    }
    if test__start_subtest(c"negative_tests_rcuptr_misuse".as_ptr()) {
        test_rcuptr_misuse();
    }
    close(cgroup_fd);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
