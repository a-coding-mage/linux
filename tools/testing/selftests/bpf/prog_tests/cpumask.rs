// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <test_progs.h>
// #include "cpumask_failure.skel.h"
// #include "cpumask_success.skel.h"

use core::ffi::{c_char, c_int, c_void};

type pid_t = c_int;

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
pub struct cpumask_success_bss {
    pub pid: pid_t,
    pub nr_cpus: c_int,
    pub err: c_int,
}

#[repr(C)]
pub struct cpumask_success {
    pub obj: *mut bpf_object,
    pub bss: *mut cpumask_success_bss,
}

unsafe extern "C" {
    fn cpumask_success__open() -> *mut cpumask_success;
    fn cpumask_success__load(skel: *mut cpumask_success) -> c_int;
    fn cpumask_success__destroy(skel: *mut cpumask_success);
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn getpid() -> pid_t;
    fn libbpf_num_possible_cpus() -> c_int;
    fn fork() -> pid_t;
    fn _exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;

    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(value: pid_t, threshold: pid_t, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn RUN_TESTS_cpumask_failure();
}

static CPUMASK_SUCCESS_TESTCASES: [*const c_char; 23] = [
    b"test_alloc_free_cpumask\0".as_ptr() as *const c_char,
    b"test_set_clear_cpu\0".as_ptr() as *const c_char,
    b"test_setall_clear_cpu\0".as_ptr() as *const c_char,
    b"test_first_firstzero_cpu\0".as_ptr() as *const c_char,
    b"test_firstand_nocpu\0".as_ptr() as *const c_char,
    b"test_test_and_set_clear\0".as_ptr() as *const c_char,
    b"test_and_or_xor\0".as_ptr() as *const c_char,
    b"test_intersects_subset\0".as_ptr() as *const c_char,
    b"test_copy_any_anyand\0".as_ptr() as *const c_char,
    b"test_insert_leave\0".as_ptr() as *const c_char,
    b"test_insert_remove_release\0".as_ptr() as *const c_char,
    b"test_global_mask_rcu\0".as_ptr() as *const c_char,
    b"test_global_mask_array_one_rcu\0".as_ptr() as *const c_char,
    b"test_global_mask_array_rcu\0".as_ptr() as *const c_char,
    b"test_global_mask_array_l2_rcu\0".as_ptr() as *const c_char,
    b"test_global_mask_nested_rcu\0".as_ptr() as *const c_char,
    b"test_global_mask_nested_deep_rcu\0".as_ptr() as *const c_char,
    b"test_global_mask_nested_deep_array_rcu\0".as_ptr() as *const c_char,
    b"test_cpumask_weight\0".as_ptr() as *const c_char,
    b"test_refcount_null_tracking\0".as_ptr() as *const c_char,
    b"test_populate_reject_small_mask\0".as_ptr() as *const c_char,
    b"test_populate_reject_unaligned\0".as_ptr() as *const c_char,
    b"test_populate\0".as_ptr() as *const c_char,
];

unsafe fn verify_success(prog_name: *const c_char) {
    let skel: *mut cpumask_success;
    let prog: *mut bpf_program;
    let mut link: *mut bpf_link = core::ptr::null_mut();
    let child_pid: pid_t;
    let mut status: c_int = 0;
    let mut err: c_int;

    skel = cpumask_success__open();
    if !ASSERT_OK_PTR(
        skel as *mut c_void,
        b"cpumask_success__open\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    (*(*skel).bss).pid = getpid();
    (*(*skel).bss).nr_cpus = libbpf_num_possible_cpus();

    err = cpumask_success__load(skel);
    if !ASSERT_OK(
        err,
        b"cpumask_success__load\0".as_ptr() as *const c_char,
    ) {
        bpf_link__destroy(link);
        cpumask_success__destroy(skel);
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR(
        prog as *mut c_void,
        b"bpf_object__find_program_by_name\0".as_ptr() as *const c_char,
    ) {
        bpf_link__destroy(link);
        cpumask_success__destroy(skel);
        return;
    }

    link = bpf_program__attach(prog);
    if !ASSERT_OK_PTR(
        link as *mut c_void,
        b"bpf_program__attach\0".as_ptr() as *const c_char,
    ) {
        bpf_link__destroy(link);
        cpumask_success__destroy(skel);
        return;
    }

    child_pid = fork();
    if !ASSERT_GT(child_pid, -1, b"child_pid\0".as_ptr() as *const c_char) {
        bpf_link__destroy(link);
        cpumask_success__destroy(skel);
        return;
    }
    if child_pid == 0 {
        _exit(0);
    }
    waitpid(child_pid, &mut status, 0);
    ASSERT_OK((*(*skel).bss).err, b"post_wait_err\0".as_ptr() as *const c_char);

    bpf_link__destroy(link);
    cpumask_success__destroy(skel);
}

pub unsafe fn test_cpumask() {
    let mut i: c_int;

    i = 0;
    while i < CPUMASK_SUCCESS_TESTCASES.len() as c_int {
        if !test__start_subtest(CPUMASK_SUCCESS_TESTCASES[i as usize]) {
            i += 1;
            continue;
        }

        verify_success(CPUMASK_SUCCESS_TESTCASES[i as usize]);
        i += 1;
    }

    RUN_TESTS_cpumask_failure();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
