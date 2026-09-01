// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies:
// #include <test_progs.h>
// #include <network_helpers.h>
// #include "test_ksyms_module.lskel.h"
// #include "test_ksyms_module.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *const c_void,
    pub data_out: *mut c_void,
    pub data_size_in: c_uint,
    pub data_size_out: c_uint,
    pub ctx_in: *const c_void,
    pub ctx_out: *mut c_void,
    pub ctx_size_in: c_uint,
    pub ctx_size_out: c_uint,
    pub retval: c_uint,
    pub repeat: c_int,
    pub duration: c_uint,
    pub flags: c_uint,
    pub cpu: c_uint,
    pub batch_size: c_uint,
}

#[repr(C)]
pub struct test_env {
    pub has_testmod: bool,
}

#[repr(C)]
pub struct test_ksyms_module_lskel {
    pub progs: test_ksyms_module_lskel_progs,
    pub bss: *mut test_ksyms_module_lskel_bss,
}

#[repr(C)]
pub struct test_ksyms_module_lskel_progs {
    pub load: test_ksyms_module_lskel_load,
}

#[repr(C)]
pub struct test_ksyms_module_lskel_load {
    pub prog_fd: c_int,
}

#[repr(C)]
pub struct test_ksyms_module_lskel_bss {
    pub out_bpf_testmod_ksym: c_int,
}

#[repr(C)]
pub struct test_ksyms_module {
    pub progs: test_ksyms_module_progs,
    pub bss: *mut test_ksyms_module_bss,
}

#[repr(C)]
pub struct test_ksyms_module_progs {
    pub load: *mut bpf_program,
}

#[repr(C)]
pub struct test_ksyms_module_bss {
    pub out_bpf_testmod_ksym: c_int,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    static env: test_env;
    static pkt_v4: [u8; 0];

    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;

    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;

    fn test_ksyms_module_lskel__open_and_load() -> *mut test_ksyms_module_lskel;
    fn test_ksyms_module_lskel__destroy(skel: *mut test_ksyms_module_lskel);

    fn test_ksyms_module__open_and_load() -> *mut test_ksyms_module;
    fn test_ksyms_module__destroy(skel: *mut test_ksyms_module);
}

unsafe fn test_ksyms_module_lskel() {
    let skel: *mut test_ksyms_module_lskel;
    let mut err: c_int;
    let mut topts = bpf_test_run_opts {
        sz: mem::size_of::<bpf_test_run_opts>(),
        data_in: (&pkt_v4 as *const [u8; 0]).cast::<c_void>(),
        data_out: core::ptr::null_mut(),
        data_size_in: mem::size_of_val(&pkt_v4) as c_uint,
        data_size_out: 0,
        ctx_in: core::ptr::null(),
        ctx_out: core::ptr::null_mut(),
        ctx_size_in: 0,
        ctx_size_out: 0,
        retval: 0,
        repeat: 1,
        duration: 0,
        flags: 0,
        cpu: 0,
        batch_size: 0,
    };

    if !env.has_testmod {
        test__skip();
        return;
    }

    skel = test_ksyms_module_lskel__open_and_load();
    if !ASSERT_OK_PTR(
        skel.cast::<c_void>(),
        c"test_ksyms_module_lskel__open_and_load".as_ptr(),
    ) {
        return;
    }
    err = bpf_prog_test_run_opts((*skel).progs.load.prog_fd, &mut topts);
    if !ASSERT_OK(err, c"bpf_prog_test_run".as_ptr()) {
        test_ksyms_module_lskel__destroy(skel);
        return;
    }
    ASSERT_EQ(topts.retval, 0, c"retval".as_ptr());
    ASSERT_EQ(
        (*(*skel).bss).out_bpf_testmod_ksym as c_uint,
        42,
        c"bpf_testmod_ksym".as_ptr(),
    );
    test_ksyms_module_lskel__destroy(skel);
}

unsafe fn test_ksyms_module_libbpf() {
    let skel: *mut test_ksyms_module;
    let mut err: c_int;
    let mut topts = bpf_test_run_opts {
        sz: mem::size_of::<bpf_test_run_opts>(),
        data_in: (&pkt_v4 as *const [u8; 0]).cast::<c_void>(),
        data_out: core::ptr::null_mut(),
        data_size_in: mem::size_of_val(&pkt_v4) as c_uint,
        data_size_out: 0,
        ctx_in: core::ptr::null(),
        ctx_out: core::ptr::null_mut(),
        ctx_size_in: 0,
        ctx_size_out: 0,
        retval: 0,
        repeat: 1,
        duration: 0,
        flags: 0,
        cpu: 0,
        batch_size: 0,
    };

    if !env.has_testmod {
        test__skip();
        return;
    }

    skel = test_ksyms_module__open_and_load();
    if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"test_ksyms_module__open".as_ptr()) {
        return;
    }
    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.load), &mut topts);
    if !ASSERT_OK(err, c"bpf_prog_test_run".as_ptr()) {
        test_ksyms_module__destroy(skel);
        return;
    }
    ASSERT_EQ(topts.retval, 0, c"retval".as_ptr());
    ASSERT_EQ(
        (*(*skel).bss).out_bpf_testmod_ksym as c_uint,
        42,
        c"bpf_testmod_ksym".as_ptr(),
    );
    test_ksyms_module__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_ksyms_module() {
    if test__start_subtest(c"lskel".as_ptr()) {
        test_ksyms_module_lskel();
    }
    if test__start_subtest(c"libbpf".as_ptr()) {
        test_ksyms_module_libbpf();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
