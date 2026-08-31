// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
/* Depends on test_progs.h and testing_helpers.h declarations. */

use std::ffi::c_char;
use std::os::raw::c_int;

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn finit_module(fd: c_int, param_values: *const c_char, flags: c_int) -> c_int;
    fn delete_module(name: *const c_char, flags: c_int) -> c_int;

    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
}

const O_RDONLY: c_int = 0;

unsafe fn load_bpf_test_no_cfi() {
    let fd: c_int;
    let mut err: c_int;

    fd = open(c"bpf_test_no_cfi.ko".as_ptr(), O_RDONLY);
    if !ASSERT_GE(fd, 0, c"open".as_ptr()) {
        return;
    }

    /* The module will try to register a struct_ops type without
     * cfi_stubs and with cfi_stubs.
     *
     * The one without cfi_stub should fail. The module will be loaded
     * successfully only if the result of the registration is as
     * expected, or it fails.
     */
    err = finit_module(fd, c"".as_ptr(), 0);
    close(fd);
    if !ASSERT_OK(err, c"finit_module".as_ptr()) {
        return;
    }

    err = delete_module(c"bpf_test_no_cfi".as_ptr(), 0);
    ASSERT_OK(err, c"delete_module".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn test_struct_ops_no_cfi() {
    if test__start_subtest(c"load_bpf_test_no_cfi".as_ptr()) {
        load_bpf_test_no_cfi();
    }
}
