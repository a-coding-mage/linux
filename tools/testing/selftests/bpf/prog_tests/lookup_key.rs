// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2022 Huawei Technologies Duesseldorf GmbH
 *
 * Author: Roberto Sassu <roberto.sassu@huawei.com>
 */

// C dependencies:
// #include <linux/keyctl.h>
// #include <test_progs.h>
// #include "test_lookup_key.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

const KEY_LOOKUP_CREATE: u64 = 0x01;
const KEY_LOOKUP_PARTIAL: u64 = 0x02;

static mut KFUNC_NOT_SUPPORTED: bool = false;

type __u32 = u32;
type LibbpfPrintFnT = Option<
    unsafe extern "C" fn(level: LibbpfPrintLevel, fmt: *const c_char, args: VaList) -> c_int,
>;

type VaList = *mut c_void;

#[repr(C)]
pub enum LibbpfPrintLevel {
    /* external enum values are supplied by libbpf */
}

#[repr(C)]
pub struct TestLookupKey {
    pub bss: *mut TestLookupKeyBss,
}

#[repr(C)]
pub struct TestLookupKeyBss {
    pub monitored_pid: c_int,
    pub key_serial: i32,
    pub flags: u64,
    pub key_id: u32,
}

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;

    fn libbpf_set_print(cb: LibbpfPrintFnT) -> LibbpfPrintFnT;
    fn test_lookup_key__open() -> *mut TestLookupKey;
    fn test_lookup_key__load(skel: *mut TestLookupKey) -> c_int;
    fn test_lookup_key__attach(skel: *mut TestLookupKey) -> c_int;
    fn test_lookup_key__destroy(skel: *mut TestLookupKey);
    fn bpf_prog_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;

    fn ASSERT_OK_PTR(ptr: *mut TestLookupKey, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_LT(ret: c_int, val: c_int, name: *const c_char) -> bool;
    fn test__skip();
}

const KEY_SPEC_THREAD_KEYRING: i32 = -1;
const UINT64_MAX: u64 = u64::MAX;
const UINT32_MAX: u32 = u32::MAX;

// Rust has no stable equivalent for directly consuming a C va_list in this
// translation unit. This declaration preserves the source-level dependency on
// retrieving the next char * argument from args.
unsafe extern "C" {
    fn va_arg_char_ptr(args: VaList) -> *mut c_char;
}

unsafe extern "C" fn libbpf_print_cb(
    _level: LibbpfPrintLevel,
    fmt: *const c_char,
    args: VaList,
) -> c_int {
    let func: *mut c_char;

    if strcmp(
        fmt,
        c"libbpf: extern (func ksym) '%s': not found in kernel or module BTFs\n".as_ptr(),
    ) != 0
    {
        return 0;
    }

    func = va_arg_char_ptr(args);

    if strcmp(func, c"bpf_lookup_user_key".as_ptr()) != 0
        && strcmp(func, c"bpf_key_put".as_ptr()) != 0
        && strcmp(func, c"bpf_lookup_system_key".as_ptr()) != 0
    {
        return 0;
    }

    KFUNC_NOT_SUPPORTED = true;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_lookup_key() {
    let old_print_cb: LibbpfPrintFnT;
    let skel: *mut TestLookupKey;
    let mut next_id: __u32 = 0;
    let mut ret: c_int;

    skel = test_lookup_key__open();
    if !ASSERT_OK_PTR(skel, c"test_lookup_key__open".as_ptr()) {
        return;
    }

    old_print_cb = libbpf_set_print(Some(libbpf_print_cb));
    ret = test_lookup_key__load(skel);
    libbpf_set_print(old_print_cb);

    if ret < 0 && KFUNC_NOT_SUPPORTED {
        printf(
            c"%s:SKIP:bpf_lookup_*_key(), bpf_key_put() kfuncs not supported\n".as_ptr(),
            c"test_lookup_key".as_ptr(),
        );
        test__skip();
        (*(*skel).bss).monitored_pid = 0;
        test_lookup_key__destroy(skel);
        return;
    }

    if !ASSERT_OK(ret, c"test_lookup_key__load".as_ptr()) {
        (*(*skel).bss).monitored_pid = 0;
        test_lookup_key__destroy(skel);
        return;
    }

    ret = test_lookup_key__attach(skel);
    if !ASSERT_OK(ret, c"test_lookup_key__attach".as_ptr()) {
        (*(*skel).bss).monitored_pid = 0;
        test_lookup_key__destroy(skel);
        return;
    }

    (*(*skel).bss).monitored_pid = getpid();
    (*(*skel).bss).key_serial = KEY_SPEC_THREAD_KEYRING;

    /* The thread-specific keyring does not exist, this test fails. */
    (*(*skel).bss).flags = 0;

    ret = bpf_prog_get_next_id(0, &mut next_id);
    if !ASSERT_LT(ret, 0, c"bpf_prog_get_next_id".as_ptr()) {
        (*(*skel).bss).monitored_pid = 0;
        test_lookup_key__destroy(skel);
        return;
    }

    /* Force creation of the thread-specific keyring, this test succeeds. */
    (*(*skel).bss).flags = KEY_LOOKUP_CREATE;

    ret = bpf_prog_get_next_id(0, &mut next_id);
    if !ASSERT_OK(ret, c"bpf_prog_get_next_id".as_ptr()) {
        (*(*skel).bss).monitored_pid = 0;
        test_lookup_key__destroy(skel);
        return;
    }

    /* Pass both lookup flags for parameter validation. */
    (*(*skel).bss).flags = KEY_LOOKUP_CREATE | KEY_LOOKUP_PARTIAL;

    ret = bpf_prog_get_next_id(0, &mut next_id);
    if !ASSERT_OK(ret, c"bpf_prog_get_next_id".as_ptr()) {
        (*(*skel).bss).monitored_pid = 0;
        test_lookup_key__destroy(skel);
        return;
    }

    /* Pass invalid flags. */
    (*(*skel).bss).flags = UINT64_MAX;

    ret = bpf_prog_get_next_id(0, &mut next_id);
    if !ASSERT_LT(ret, 0, c"bpf_prog_get_next_id".as_ptr()) {
        (*(*skel).bss).monitored_pid = 0;
        test_lookup_key__destroy(skel);
        return;
    }

    (*(*skel).bss).key_serial = 0;
    (*(*skel).bss).key_id = 1;

    ret = bpf_prog_get_next_id(0, &mut next_id);
    if !ASSERT_OK(ret, c"bpf_prog_get_next_id".as_ptr()) {
        (*(*skel).bss).monitored_pid = 0;
        test_lookup_key__destroy(skel);
        return;
    }

    (*(*skel).bss).key_id = UINT32_MAX;

    ret = bpf_prog_get_next_id(0, &mut next_id);
    ASSERT_LT(ret, 0, c"bpf_prog_get_next_id".as_ptr());

    (*(*skel).bss).monitored_pid = 0;
    test_lookup_key__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
