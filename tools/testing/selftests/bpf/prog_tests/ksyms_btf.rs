// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Google */

// C dependencies:
// <test_progs.h>
// <bpf/libbpf.h>
// <bpf/btf.h>
// "test_ksyms_btf.skel.h"
// "test_ksyms_btf_null_check.skel.h"
// "test_ksyms_weak.skel.h"
// "test_ksyms_weak.lskel.h"
// "test_ksyms_btf_write_check.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u32 = u32;
type __u64 = u64;

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const BTF_KIND_DATASEC: c_uint = 15;

static mut duration: c_int = 0;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_ksyms_btf {
    pub data: *mut test_ksyms_btf__data,
}

#[repr(C)]
pub struct test_ksyms_btf__data {
    pub out__runqueues_addr: __u64,
    pub out__bpf_prog_active_addr: __u64,
    pub out__rq_cpu: __u32,
    pub out__bpf_prog_active: c_int,
    pub out__cpu_0_rq_cpu: __u32,
    pub out__this_rq_cpu: __u32,
    pub out__this_bpf_prog_active: c_int,
}

#[repr(C)]
pub struct test_ksyms_btf_null_check {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_ksyms_weak {
    pub data: *mut test_ksyms_weak__data,
}

#[repr(C)]
pub struct test_ksyms_weak__data {
    pub out__existing_typed: c_int,
    pub out__existing_typeless: c_int,
    pub out__non_existent_typeless: c_int,
    pub out__non_existent_typed: c_int,
}

#[repr(C)]
pub struct test_ksyms_weak_lskel {
    pub data: *mut test_ksyms_weak_lskel__data,
}

#[repr(C)]
pub struct test_ksyms_weak_lskel__data {
    pub out__existing_typed: c_int,
    pub out__existing_typeless: c_int,
    pub out__non_existent_typeless: c_int,
    pub out__non_existent_typed: c_int,
}

#[repr(C)]
pub struct test_ksyms_btf_write_check {
    pub progs: test_ksyms_btf_write_check__progs,
}

#[repr(C)]
pub struct test_ksyms_btf_write_check__progs {
    pub handler1: *mut bpf_program,
    pub handler2: *mut bpf_program,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn kallsyms_find(sym: *const c_char, addr: *mut __u64) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn libbpf_find_kernel_btf() -> *mut btf;
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: c_uint) -> c_int;
    fn btf__free(btf: *mut btf);
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);

    fn test_ksyms_btf__open_and_load() -> *mut test_ksyms_btf;
    fn test_ksyms_btf__attach(skel: *mut test_ksyms_btf) -> c_int;
    fn test_ksyms_btf__destroy(skel: *mut test_ksyms_btf);

    fn test_ksyms_btf_null_check__open_and_load() -> *mut test_ksyms_btf_null_check;
    fn test_ksyms_btf_null_check__destroy(skel: *mut test_ksyms_btf_null_check);

    fn test_ksyms_weak__open_and_load() -> *mut test_ksyms_weak;
    fn test_ksyms_weak__attach(skel: *mut test_ksyms_weak) -> c_int;
    fn test_ksyms_weak__destroy(skel: *mut test_ksyms_weak);

    fn test_ksyms_weak_lskel__open_and_load() -> *mut test_ksyms_weak_lskel;
    fn test_ksyms_weak_lskel__attach(skel: *mut test_ksyms_weak_lskel) -> c_int;
    fn test_ksyms_weak_lskel__destroy(skel: *mut test_ksyms_weak_lskel);

    fn test_ksyms_btf_write_check__open() -> *mut test_ksyms_btf_write_check;
    fn test_ksyms_btf_write_check__load(skel: *mut test_ksyms_btf_write_check) -> c_int;
    fn test_ksyms_btf_write_check__destroy(skel: *mut test_ksyms_btf_write_check);

    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

unsafe fn test_basic() {
    let mut runqueues_addr: __u64 = 0;
    let mut bpf_prog_active_addr: __u64 = 0;
    let this_rq_cpu: __u32;
    let this_bpf_prog_active: c_int;
    let mut skel: *mut test_ksyms_btf = core::ptr::null_mut();
    let data: *mut test_ksyms_btf__data;
    let mut err: c_int;

    err = kallsyms_find(c"runqueues".as_ptr(), &mut runqueues_addr);
    if CHECK(
        err == -EINVAL,
        c"kallsyms_fopen".as_ptr(),
        c"failed to open: %d\n".as_ptr(),
        errno,
    ) {
        return;
    }
    if CHECK(
        err == -ENOENT,
        c"ksym_find".as_ptr(),
        c"symbol 'runqueues' not found\n".as_ptr(),
    ) {
        return;
    }

    err = kallsyms_find(c"bpf_prog_active".as_ptr(), &mut bpf_prog_active_addr);
    if CHECK(
        err == -EINVAL,
        c"kallsyms_fopen".as_ptr(),
        c"failed to open: %d\n".as_ptr(),
        errno,
    ) {
        return;
    }
    if CHECK(
        err == -ENOENT,
        c"ksym_find".as_ptr(),
        c"symbol 'bpf_prog_active' not found\n".as_ptr(),
    ) {
        return;
    }

    skel = test_ksyms_btf__open_and_load();
    if CHECK(
        skel.is_null(),
        c"skel_open".as_ptr(),
        c"failed to open and load skeleton\n".as_ptr(),
    ) {
        test_ksyms_btf__destroy(skel);
        return;
    }

    err = test_ksyms_btf__attach(skel);
    if CHECK(
        err != 0,
        c"skel_attach".as_ptr(),
        c"skeleton attach failed: %d\n".as_ptr(),
        err,
    ) {
        test_ksyms_btf__destroy(skel);
        return;
    }

    /* trigger tracepoint */
    usleep(1);

    data = (*skel).data;
    CHECK(
        (*data).out__runqueues_addr != runqueues_addr,
        c"runqueues_addr".as_ptr(),
        c"got %llu, exp %llu\n".as_ptr(),
        (*data).out__runqueues_addr as u64,
        runqueues_addr as u64,
    );
    CHECK(
        (*data).out__bpf_prog_active_addr != bpf_prog_active_addr,
        c"bpf_prog_active_addr".as_ptr(),
        c"got %llu, exp %llu\n".as_ptr(),
        (*data).out__bpf_prog_active_addr as u64,
        bpf_prog_active_addr as u64,
    );

    CHECK(
        (*data).out__rq_cpu == (-1i32 as __u32),
        c"rq_cpu".as_ptr(),
        c"got %u, exp != -1\n".as_ptr(),
        (*data).out__rq_cpu,
    );
    CHECK(
        (*data).out__bpf_prog_active < 0,
        c"bpf_prog_active".as_ptr(),
        c"got %d, exp >= 0\n".as_ptr(),
        (*data).out__bpf_prog_active,
    );
    CHECK(
        (*data).out__cpu_0_rq_cpu != 0,
        c"cpu_rq(0)->cpu".as_ptr(),
        c"got %u, exp 0\n".as_ptr(),
        (*data).out__cpu_0_rq_cpu,
    );

    this_rq_cpu = (*data).out__this_rq_cpu;
    CHECK(
        this_rq_cpu != (*data).out__rq_cpu,
        c"this_rq_cpu".as_ptr(),
        c"got %u, exp %u\n".as_ptr(),
        this_rq_cpu,
        (*data).out__rq_cpu,
    );

    this_bpf_prog_active = (*data).out__this_bpf_prog_active;
    CHECK(
        this_bpf_prog_active != (*data).out__bpf_prog_active,
        c"this_bpf_prog_active".as_ptr(),
        c"got %d, exp %d\n".as_ptr(),
        this_bpf_prog_active,
        (*data).out__bpf_prog_active,
    );

    test_ksyms_btf__destroy(skel);
}

unsafe fn test_null_check() {
    let skel: *mut test_ksyms_btf_null_check;

    skel = test_ksyms_btf_null_check__open_and_load();
    CHECK(
        !skel.is_null(),
        c"skel_open".as_ptr(),
        c"unexpected load of a prog missing null check\n".as_ptr(),
    );

    test_ksyms_btf_null_check__destroy(skel);
}

unsafe fn test_weak_syms() {
    let skel: *mut test_ksyms_weak;
    let data: *mut test_ksyms_weak__data;
    let err: c_int;

    skel = test_ksyms_weak__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *mut c_void,
        c"test_ksyms_weak__open_and_load".as_ptr(),
    ) {
        return;
    }

    err = test_ksyms_weak__attach(skel);
    if !ASSERT_OK(err, c"test_ksyms_weak__attach".as_ptr()) {
        test_ksyms_weak__destroy(skel);
        return;
    }

    /* trigger tracepoint */
    usleep(1);

    data = (*skel).data;
    ASSERT_EQ((*data).out__existing_typed, 0, c"existing typed ksym".as_ptr());
    ASSERT_NEQ(
        (*data).out__existing_typeless,
        -1,
        c"existing typeless ksym".as_ptr(),
    );
    ASSERT_EQ(
        (*data).out__non_existent_typeless,
        0,
        c"nonexistent typeless ksym".as_ptr(),
    );
    ASSERT_EQ(
        (*data).out__non_existent_typed,
        0,
        c"nonexistent typed ksym".as_ptr(),
    );

    test_ksyms_weak__destroy(skel);
}

unsafe fn test_weak_syms_lskel() {
    let skel: *mut test_ksyms_weak_lskel;
    let data: *mut test_ksyms_weak_lskel__data;
    let err: c_int;

    skel = test_ksyms_weak_lskel__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *mut c_void,
        c"test_ksyms_weak_lskel__open_and_load".as_ptr(),
    ) {
        return;
    }

    err = test_ksyms_weak_lskel__attach(skel);
    if !ASSERT_OK(err, c"test_ksyms_weak_lskel__attach".as_ptr()) {
        test_ksyms_weak_lskel__destroy(skel);
        return;
    }

    /* trigger tracepoint */
    usleep(1);

    data = (*skel).data;
    ASSERT_EQ((*data).out__existing_typed, 0, c"existing typed ksym".as_ptr());
    ASSERT_NEQ(
        (*data).out__existing_typeless,
        -1,
        c"existing typeless ksym".as_ptr(),
    );
    ASSERT_EQ(
        (*data).out__non_existent_typeless,
        0,
        c"nonexistent typeless ksym".as_ptr(),
    );
    ASSERT_EQ(
        (*data).out__non_existent_typed,
        0,
        c"nonexistent typed ksym".as_ptr(),
    );

    test_ksyms_weak_lskel__destroy(skel);
}

unsafe fn test_write_check(test_handler1: bool) {
    let skel: *mut test_ksyms_btf_write_check;

    skel = test_ksyms_btf_write_check__open();
    if !ASSERT_OK_PTR(
        skel as *mut c_void,
        c"test_ksyms_btf_write_check__open".as_ptr(),
    ) {
        return;
    }
    bpf_program__set_autoload(
        if test_handler1 {
            (*skel).progs.handler2
        } else {
            (*skel).progs.handler1
        },
        false,
    );
    ASSERT_ERR(
        test_ksyms_btf_write_check__load(skel),
        c"unexpected load of a prog writing to ksym memory\n".as_ptr(),
    );

    test_ksyms_btf_write_check__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_ksyms_btf() {
    let percpu_datasec: c_int;
    let btf: *mut btf;

    btf = libbpf_find_kernel_btf();
    if !ASSERT_OK_PTR(btf as *mut c_void, c"btf_exists".as_ptr()) {
        return;
    }

    percpu_datasec = btf__find_by_name_kind(
        btf,
        c".data..percpu".as_ptr(),
        BTF_KIND_DATASEC,
    );
    btf__free(btf);
    if percpu_datasec < 0 {
        printf(
            c"%s:SKIP:no PERCPU DATASEC in kernel btf\n".as_ptr(),
            c"test_ksyms_btf".as_ptr(),
        );
        test__skip();
        return;
    }

    if test__start_subtest(c"basic".as_ptr()) {
        test_basic();
    }

    if test__start_subtest(c"null_check".as_ptr()) {
        test_null_check();
    }

    if test__start_subtest(c"weak_ksyms".as_ptr()) {
        test_weak_syms();
    }

    if test__start_subtest(c"weak_ksyms_lskel".as_ptr()) {
        test_weak_syms_lskel();
    }

    if test__start_subtest(c"write_check1".as_ptr()) {
        test_write_check(true);
    }

    if test__start_subtest(c"write_check2".as_ptr()) {
        test_write_check(false);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
