// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (c) 2022 Facebook
 * Copyright (C) 2022 Huawei Technologies Duesseldorf GmbH
 *
 * Author: Roberto Sassu <roberto.sassu@huawei.com>
 */

// C dependencies:
// #include <test_progs.h>
// #include "test_kfunc_dynptr_param.skel.h"

use core::ffi::{c_char, c_int};

type bool_ = bool;
type __u32 = u32;
type va_list = *mut core::ffi::c_void;
type libbpf_print_fn_t = Option<
    unsafe extern "C" fn(level: libbpf_print_level, fmt: *const c_char, args: va_list) -> c_int,
>;

const EINVAL: c_int = 22;

#[repr(C)]
pub enum libbpf_print_level {
    /* External enum from libbpf; variants are supplied by included dependencies. */
}

#[repr(C)]
pub struct test_kfunc_dynptr_param {
    pub obj: *mut bpf_object,
    pub bss: *mut test_kfunc_dynptr_param__bss,
}

#[repr(C)]
pub struct test_kfunc_dynptr_param__bss {
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
struct kfunc_dynptr_test {
    prog_name: *const c_char,
    expected_runtime_err: c_int,
}

static KFUNC_DYNPTR_TESTS: [kfunc_dynptr_test; 1] = [kfunc_dynptr_test {
    prog_name: c"dynptr_data_null".as_ptr(),
    expected_runtime_err: -EINVAL,
}];

static mut KFUNC_NOT_SUPPORTED: bool_ = false;

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn va_arg_char_ptr(args: va_list) -> *mut c_char;
    fn fprintf(stream: *mut core::ffi::c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut core::ffi::c_void;
    fn getpid() -> c_int;

    fn libbpf_set_print(cb: libbpf_print_fn_t) -> libbpf_print_fn_t;
    fn test_kfunc_dynptr_param__open() -> *mut test_kfunc_dynptr_param;
    fn test_kfunc_dynptr_param__load(skel: *mut test_kfunc_dynptr_param) -> c_int;
    fn test_kfunc_dynptr_param__destroy(skel: *mut test_kfunc_dynptr_param);
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_prog_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn ASSERT_OK_PTR(ptr: *mut core::ffi::c_void, name: *const c_char) -> bool_;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool_;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char);
    fn test__start_subtest(name: *const c_char) -> bool_;
    fn RUN_TESTS_test_kfunc_dynptr_param();
}

unsafe extern "C" fn libbpf_print_cb(
    _level: libbpf_print_level,
    fmt: *const c_char,
    args: va_list,
) -> c_int {
    if unsafe {
        strcmp(
            fmt,
            c"libbpf: extern (func ksym) '%s': not found in kernel or module BTFs\n".as_ptr(),
        )
    } != 0
    {
        return 0;
    }

    if unsafe { strcmp(va_arg_char_ptr(args), c"bpf_verify_pkcs7_signature".as_ptr()) } != 0 {
        return 0;
    }

    unsafe {
        KFUNC_NOT_SUPPORTED = true;
    }
    0
}

unsafe fn has_pkcs7_kfunc_support() -> bool_ {
    let skel: *mut test_kfunc_dynptr_param;
    let old_print_cb: libbpf_print_fn_t;
    let err: c_int;

    skel = unsafe { test_kfunc_dynptr_param__open() };
    if !unsafe {
        ASSERT_OK_PTR(
            skel as *mut core::ffi::c_void,
            c"test_kfunc_dynptr_param__open".as_ptr(),
        )
    } {
        return false;
    }

    unsafe {
        KFUNC_NOT_SUPPORTED = false;
    }

    old_print_cb = unsafe { libbpf_set_print(Some(libbpf_print_cb)) };
    err = unsafe { test_kfunc_dynptr_param__load(skel) };
    unsafe {
        libbpf_set_print(old_print_cb);
    }

    if err < 0 && unsafe { KFUNC_NOT_SUPPORTED } {
        unsafe {
            fprintf(
                stderr,
                c"%s:SKIP:bpf_verify_pkcs7_signature() kfunc not supported\n".as_ptr(),
                c"has_pkcs7_kfunc_support".as_ptr(),
            );
            test_kfunc_dynptr_param__destroy(skel);
        }
        return false;
    }

    unsafe {
        test_kfunc_dynptr_param__destroy(skel);
    }

    true
}

unsafe fn verify_success(prog_name: *const c_char, expected_runtime_err: c_int) {
    let skel: *mut test_kfunc_dynptr_param;
    let prog: *mut bpf_program;
    let link: *mut bpf_link;
    let mut next_id: __u32 = 0;
    let mut err: c_int;

    skel = unsafe { test_kfunc_dynptr_param__open() };
    if !unsafe {
        ASSERT_OK_PTR(
            skel as *mut core::ffi::c_void,
            c"test_kfunc_dynptr_param__open".as_ptr(),
        )
    } {
        return;
    }

    unsafe {
        (*(*skel).bss).pid = getpid();
    }

    err = unsafe { test_kfunc_dynptr_param__load(skel) };

    if !unsafe { ASSERT_OK(err, c"test_kfunc_dynptr_param__load".as_ptr()) } {
        unsafe {
            test_kfunc_dynptr_param__destroy(skel);
        }
        return;
    }

    prog = unsafe { bpf_object__find_program_by_name((*skel).obj, prog_name) };
    if !unsafe {
        ASSERT_OK_PTR(
            prog as *mut core::ffi::c_void,
            c"bpf_object__find_program_by_name".as_ptr(),
        )
    } {
        unsafe {
            test_kfunc_dynptr_param__destroy(skel);
        }
        return;
    }

    link = unsafe { bpf_program__attach(prog) };
    if !unsafe {
        ASSERT_OK_PTR(
            link as *mut core::ffi::c_void,
            c"bpf_program__attach".as_ptr(),
        )
    } {
        unsafe {
            test_kfunc_dynptr_param__destroy(skel);
        }
        return;
    }

    err = unsafe { bpf_prog_get_next_id(0, &mut next_id) };

    unsafe {
        bpf_link__destroy(link);
    }

    if !unsafe { ASSERT_OK(err, c"bpf_prog_get_next_id".as_ptr()) } {
        unsafe {
            test_kfunc_dynptr_param__destroy(skel);
        }
        return;
    }

    unsafe {
        ASSERT_EQ((*(*skel).bss).err, expected_runtime_err, c"err".as_ptr());
    }

    unsafe {
        test_kfunc_dynptr_param__destroy(skel);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kfunc_dynptr_param() {
    let mut i: c_int;

    if !unsafe { has_pkcs7_kfunc_support() } {
        return;
    }

    i = 0;
    while (i as usize) < KFUNC_DYNPTR_TESTS.len() {
        if !unsafe { test__start_subtest(KFUNC_DYNPTR_TESTS[i as usize].prog_name) } {
            i += 1;
            continue;
        }

        unsafe {
            verify_success(
                KFUNC_DYNPTR_TESTS[i as usize].prog_name,
                KFUNC_DYNPTR_TESTS[i as usize].expected_runtime_err,
            );
        }
        i += 1;
    }
    unsafe {
        RUN_TESTS_test_kfunc_dynptr_param();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
