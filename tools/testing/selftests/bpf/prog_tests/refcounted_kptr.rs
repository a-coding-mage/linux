// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external declarations:
// #include <test_progs.h>
// #include <network_helpers.h>
// #include "refcounted_kptr.skel.h"
// #include "refcounted_kptr_fail.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;

type size_t = usize;
type u64 = u64;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const c_void,
    pub data_size_in: size_t,
    pub repeat: c_uint,
    pub retval: c_int,
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
pub struct refcounted_kptr_maps {
    pub percpu_hash: *mut bpf_map,
}

#[repr(C)]
pub struct refcounted_kptr_progs {
    pub rbtree_wrong_owner_remove_fail_a1: *mut bpf_program,
    pub rbtree_wrong_owner_remove_fail_b: *mut bpf_program,
    pub rbtree_wrong_owner_remove_fail_a2: *mut bpf_program,
    pub percpu_hash_refcount_leak: *mut bpf_program,
    pub clear_percpu_hash_kptr: *mut bpf_program,
    pub check_percpu_hash_refcount: *mut bpf_program,
}

#[repr(C)]
pub struct refcounted_kptr {
    pub maps: refcounted_kptr_maps,
    pub progs: refcounted_kptr_progs,
}

unsafe extern "C" {
    static pkt_v4: c_void;

    fn RUN_TESTS(name: *const c_char);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(a: c_int, b: c_int, name: *const c_char) -> bool;

    fn refcounted_kptr__open_and_load() -> *mut refcounted_kptr;
    fn refcounted_kptr__destroy(skel: *mut refcounted_kptr);

    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn libbpf_num_possible_cpus() -> c_int;
    fn bpf_map__update_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: size_t,
        value: *const c_void,
        value_sz: size_t,
        flags: u64,
    ) -> c_int;

    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_refcounted_kptr() {
    unsafe {
        RUN_TESTS(c"refcounted_kptr".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_refcounted_kptr_fail() {
    unsafe {
        RUN_TESTS(c"refcounted_kptr_fail".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_refcounted_kptr_wrong_owner() {
    let mut opts = bpf_test_run_opts {
        data_in: unsafe { &pkt_v4 as *const c_void },
        data_size_in: size_of::<c_void>(),
        repeat: 1,
        retval: 0,
    };
    let skel: *mut refcounted_kptr;
    let mut ret: c_int;

    unsafe {
        skel = refcounted_kptr__open_and_load();
        if !ASSERT_OK_PTR(
            skel as *const c_void,
            c"refcounted_kptr__open_and_load".as_ptr(),
        ) {
            return;
        }

        ret = bpf_prog_test_run_opts(
            bpf_program__fd((*skel).progs.rbtree_wrong_owner_remove_fail_a1),
            &mut opts,
        );
        ASSERT_OK(ret, c"rbtree_wrong_owner_remove_fail_a1".as_ptr());
        ASSERT_OK(opts.retval, c"rbtree_wrong_owner_remove_fail_a1 retval".as_ptr());

        ret = bpf_prog_test_run_opts(
            bpf_program__fd((*skel).progs.rbtree_wrong_owner_remove_fail_b),
            &mut opts,
        );
        ASSERT_OK(ret, c"rbtree_wrong_owner_remove_fail_b".as_ptr());
        ASSERT_OK(opts.retval, c"rbtree_wrong_owner_remove_fail_b retval".as_ptr());

        ret = bpf_prog_test_run_opts(
            bpf_program__fd((*skel).progs.rbtree_wrong_owner_remove_fail_a2),
            &mut opts,
        );
        ASSERT_OK(ret, c"rbtree_wrong_owner_remove_fail_a2".as_ptr());
        ASSERT_OK(opts.retval, c"rbtree_wrong_owner_remove_fail_a2 retval".as_ptr());
        refcounted_kptr__destroy(skel);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_percpu_hash_refcounted_kptr_refcount_leak() {
    let skel: *mut refcounted_kptr;
    let mut cpu_nr: c_int;
    let mut fd: c_int;
    let mut err: c_int;
    let key: c_int = 0;
    let mut map: *mut bpf_map;
    let mut values_sz: size_t;
    let values: *mut u64;
    let mut opts = bpf_test_run_opts {
        data_in: unsafe { &pkt_v4 as *const c_void },
        data_size_in: size_of::<c_void>(),
        repeat: 1,
        retval: 0,
    };
    let mut syscall_opts = bpf_test_run_opts {
        data_in: core::ptr::null(),
        data_size_in: 0,
        repeat: 0,
        retval: 0,
    };

    unsafe {
        cpu_nr = libbpf_num_possible_cpus();
        if !ASSERT_GT(cpu_nr, 0, c"libbpf_num_possible_cpus".as_ptr()) {
            return;
        }

        values = calloc(cpu_nr as size_t, size_of::<u64>()) as *mut u64;
        if !ASSERT_OK_PTR(values as *const c_void, c"calloc values".as_ptr()) {
            return;
        }

        skel = refcounted_kptr__open_and_load();
        if !ASSERT_OK_PTR(
            skel as *const c_void,
            c"refcounted_kptr__open_and_load".as_ptr(),
        ) {
            free(values as *mut c_void);
            return;
        }

        values_sz = cpu_nr as size_t * size_of::<u64>();
        memset(values as *mut c_void, 0, values_sz);

        map = (*skel).maps.percpu_hash;
        err = bpf_map__update_elem(
            map,
            &key as *const c_int as *const c_void,
            size_of::<c_int>(),
            values as *const c_void,
            values_sz,
            0,
        );
        if !ASSERT_OK(err, c"bpf_map__update_elem".as_ptr()) {
            refcounted_kptr__destroy(skel);
            free(values as *mut c_void);
            return;
        }

        fd = bpf_program__fd((*skel).progs.percpu_hash_refcount_leak);
        err = bpf_prog_test_run_opts(fd, &mut opts);
        if !ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr()) {
            refcounted_kptr__destroy(skel);
            free(values as *mut c_void);
            return;
        }
        if !ASSERT_EQ(opts.retval, 2, c"opts.retval".as_ptr()) {
            refcounted_kptr__destroy(skel);
            free(values as *mut c_void);
            return;
        }

        fd = bpf_program__fd((*skel).progs.clear_percpu_hash_kptr);
        err = bpf_prog_test_run_opts(fd, &mut syscall_opts);
        if !ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr()) {
            refcounted_kptr__destroy(skel);
            free(values as *mut c_void);
            return;
        }
        if !ASSERT_EQ(syscall_opts.retval, 1, c"syscall_opts.retval".as_ptr()) {
            refcounted_kptr__destroy(skel);
            free(values as *mut c_void);
            return;
        }

        fd = bpf_program__fd((*skel).progs.check_percpu_hash_refcount);
        err = bpf_prog_test_run_opts(fd, &mut opts);
        ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());
        ASSERT_EQ(opts.retval, 1, c"opts.retval".as_ptr());

        refcounted_kptr__destroy(skel);
        free(values as *mut c_void);
    }
}
