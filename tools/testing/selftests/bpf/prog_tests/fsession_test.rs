// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 ChinaTelecom */

/* Dependencies from the original C file:
 * #include <test_progs.h>
 * #include "fsession_test.skel.h"
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

type __u64 = u64;

const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: c_int,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fsession_test__progs {
    pub test1: *mut bpf_program,
    pub test6: *mut bpf_program,
}

#[repr(C)]
pub struct fsession_test__bss {
    pub test6_entry_result: __u64,
    pub test6_exit_result: __u64,
}

#[repr(C)]
pub struct fsession_test {
    pub progs: fsession_test__progs,
    pub bss: *mut fsession_test__bss,
}

unsafe extern "C" {
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);

    fn fsession_test__open() -> *mut fsession_test;
    fn fsession_test__load(skel: *mut fsession_test) -> c_int;
    fn fsession_test__attach(skel: *mut fsession_test) -> c_int;
    fn fsession_test__detach(skel: *mut fsession_test);
    fn fsession_test__destroy(skel: *mut fsession_test);

    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: __u64, expected: __u64, name: *const c_char) -> bool;
}

unsafe fn check_result(skel: *mut fsession_test) -> c_int {
    let mut topts: bpf_test_run_opts = mem::zeroed();
    let err: c_int;
    let prog_fd: c_int;

    /* Trigger test function calls */
    prog_fd = bpf_program__fd((*skel).progs.test1);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return err;
    }
    if !ASSERT_OK(topts.retval, c"test_run_opts retval".as_ptr()) {
        return topts.retval;
    }

    for i in 0..(mem::size_of_val(&*(*skel).bss) / mem::size_of::<__u64>()) {
        if !ASSERT_EQ(*((*skel).bss as *mut __u64).add(i), 1, c"test_result".as_ptr()) {
            return -EINVAL;
        }
    }

    0
}

unsafe fn test_fsession_basic() {
    let mut skel: *mut fsession_test = ptr::null_mut();
    let mut err: c_int;

    skel = fsession_test__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"fsession_test__open".as_ptr()) {
        return;
    }

    err = fsession_test__load(skel);
    if err == -EOPNOTSUPP {
        test__skip();
        fsession_test__destroy(skel);
        return;
    }
    if !ASSERT_OK(err, c"fsession_test__load".as_ptr()) {
        fsession_test__destroy(skel);
        return;
    }

    err = fsession_test__attach(skel);
    if !ASSERT_OK(err, c"fsession_attach".as_ptr()) {
        fsession_test__destroy(skel);
        return;
    }

    check_result(skel);
    fsession_test__destroy(skel);
}

unsafe fn test_fsession_reattach() {
    let mut skel: *mut fsession_test = ptr::null_mut();
    let mut err: c_int;

    skel = fsession_test__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"fsession_test__open".as_ptr()) {
        return;
    }

    err = fsession_test__load(skel);
    if err == -EOPNOTSUPP {
        test__skip();
        fsession_test__destroy(skel);
        return;
    }
    if !ASSERT_OK(err, c"fsession_test__load".as_ptr()) {
        fsession_test__destroy(skel);
        return;
    }

    /* first attach */
    err = fsession_test__attach(skel);
    if !ASSERT_OK(err, c"fsession_first_attach".as_ptr()) {
        fsession_test__destroy(skel);
        return;
    }

    if check_result(skel) != 0 {
        fsession_test__destroy(skel);
        return;
    }

    /* detach */
    fsession_test__detach(skel);

    /* reset counters */
    ptr::write_bytes((*skel).bss as *mut u8, 0, mem::size_of_val(&*(*skel).bss));

    /* second attach */
    err = fsession_test__attach(skel);
    if !ASSERT_OK(err, c"fsession_second_attach".as_ptr()) {
        fsession_test__destroy(skel);
        return;
    }

    if check_result(skel) != 0 {
        fsession_test__destroy(skel);
        return;
    }

    fsession_test__destroy(skel);
}

unsafe fn test_fsession_cookie() {
    let mut skel: *mut fsession_test = ptr::null_mut();
    let mut err: c_int;

    skel = fsession_test__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"fsession_test__open".as_ptr()) {
        fsession_test__destroy(skel);
        return;
    }

    /*
     * The test_fsession_basic() will test the session cookie with
     * bpf_get_func_ip() case, so we need only check
     * the cookie without bpf_get_func_ip() case here
     */
    bpf_program__set_autoload((*skel).progs.test6, false);

    err = fsession_test__load(skel);
    if err == -EOPNOTSUPP {
        test__skip();
        fsession_test__destroy(skel);
        return;
    }
    if !ASSERT_OK(err, c"fsession_test__load".as_ptr()) {
        fsession_test__destroy(skel);
        return;
    }

    err = fsession_test__attach(skel);
    if !ASSERT_OK(err, c"fsession_attach".as_ptr()) {
        fsession_test__destroy(skel);
        return;
    }

    (*(*skel).bss).test6_entry_result = 1;
    (*(*skel).bss).test6_exit_result = 1;

    check_result(skel);
    fsession_test__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_fsession_test() {
    if test__start_subtest(c"fsession_test".as_ptr()) {
        test_fsession_basic();
    }
    if test__start_subtest(c"fsession_reattach".as_ptr()) {
        test_fsession_reattach();
    }
    if test__start_subtest(c"fsession_cookie".as_ptr()) {
        test_fsession_cookie();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
