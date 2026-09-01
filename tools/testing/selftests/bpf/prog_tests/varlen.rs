// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/*
 * Translated from C source that included:
 * <test_progs.h>, <time.h>, and "test_varlen.skel.h".
 * The CHECK! macro and skeleton symbols are expected to be supplied by the
 * surrounding test harness/bindings.
 */

use core::ffi::{c_char, c_int, c_long, c_void};

const EFAULT: c_int = 14;

macro_rules! CHECK_VAL {
    ($got:expr, $exp:expr) => {
        CHECK!(
            ($got) != ($exp),
            "check",
            "got %ld != exp %ld\n",
            ($got) as c_long,
            ($exp) as c_long
        )
    };
}

#[repr(C)]
pub struct test_varlen {
    pub bss: *mut test_varlen__bss,
    pub data: *mut test_varlen__data,
}

#[repr(C)]
pub struct test_varlen__bss {
    pub test_pid: c_int,
    pub buf_in1: [c_char; 8],
    pub buf_in2: [c_char; 7],
    pub capture: bool,
    pub payload1_len1: c_int,
    pub payload1_len2: c_int,
    pub total1: c_int,
    pub payload1: [c_char; 15],
    pub ret_bad_read: c_int,
}

#[repr(C)]
pub struct test_varlen__data {
    pub payload2_len1: c_int,
    pub payload2_len2: c_int,
    pub total2: c_int,
    pub payload2: [c_char; 15],
    pub payload3_len1: c_int,
    pub payload3_len2: c_int,
    pub total3: c_int,
    pub payload3: [c_char; 15],
    pub payload4_len1: c_int,
    pub payload4_len2: c_int,
    pub total4: c_int,
    pub payload4: [c_char; 15],
    pub payload_bad: [c_char; 5],
}

unsafe extern "C" {
    fn test_varlen__open_and_load() -> *mut test_varlen;
    fn test_varlen__attach(skel: *mut test_varlen) -> c_int;
    fn test_varlen__destroy(skel: *mut test_varlen);

    fn getpid() -> c_int;
    fn usleep(usec: u32) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
}

pub unsafe fn test_varlen() {
    let duration: c_int = 0;
    let mut err: c_int;
    let mut skel: *mut test_varlen;
    let mut bss: *mut test_varlen__bss;
    let mut data: *mut test_varlen__data;
    let str1: &[u8; 8] = b"Hello, \0";
    let str2: &[u8; 7] = b"World!\0";
    let exp_str: &[u8; 15] = b"Hello, \0World!\0";
    let size1: c_int = core::mem::size_of_val(str1) as c_int;
    let size2: c_int = core::mem::size_of_val(str2) as c_int;

    let _ = duration;

    skel = test_varlen__open_and_load();
    if CHECK!(!skel.is_null(), "skel_open", "failed to open skeleton\n") {
        return;
    }
    bss = (*skel).bss;
    data = (*skel).data;

    err = test_varlen__attach(skel);
    if CHECK!(
        err != 0,
        "skel_attach",
        "skeleton attach failed: %d\n",
        err
    ) {
        test_varlen__destroy(skel);
        return;
    }

    (*bss).test_pid = getpid();

    /* trigger everything */
    memcpy(
        (*bss).buf_in1.as_mut_ptr() as *mut c_void,
        str1.as_ptr() as *const c_void,
        size1 as usize,
    );
    memcpy(
        (*bss).buf_in2.as_mut_ptr() as *mut c_void,
        str2.as_ptr() as *const c_void,
        size2 as usize,
    );
    (*bss).capture = true;
    usleep(1);
    (*bss).capture = false;

    CHECK_VAL!((*bss).payload1_len1, size1);
    CHECK_VAL!((*bss).payload1_len2, size2);
    CHECK_VAL!((*bss).total1, size1 + size2);
    CHECK!(
        memcmp(
            (*bss).payload1.as_ptr() as *const c_void,
            exp_str.as_ptr() as *const c_void,
            (size1 + size2) as usize
        ) != 0,
        "content_check",
        "doesn't match!\n"
    );

    CHECK_VAL!((*data).payload2_len1, size1);
    CHECK_VAL!((*data).payload2_len2, size2);
    CHECK_VAL!((*data).total2, size1 + size2);
    CHECK!(
        memcmp(
            (*data).payload2.as_ptr() as *const c_void,
            exp_str.as_ptr() as *const c_void,
            (size1 + size2) as usize
        ) != 0,
        "content_check",
        "doesn't match!\n"
    );

    CHECK_VAL!((*data).payload3_len1, size1);
    CHECK_VAL!((*data).payload3_len2, size2);
    CHECK_VAL!((*data).total3, size1 + size2);
    CHECK!(
        memcmp(
            (*data).payload3.as_ptr() as *const c_void,
            exp_str.as_ptr() as *const c_void,
            (size1 + size2) as usize
        ) != 0,
        "content_check",
        "doesn't match!\n"
    );

    CHECK_VAL!((*data).payload4_len1, size1);
    CHECK_VAL!((*data).payload4_len2, size2);
    CHECK_VAL!((*data).total4, size1 + size2);
    CHECK!(
        memcmp(
            (*data).payload4.as_ptr() as *const c_void,
            exp_str.as_ptr() as *const c_void,
            (size1 + size2) as usize
        ) != 0,
        "content_check",
        "doesn't match!\n"
    );

    CHECK_VAL!((*bss).ret_bad_read, -EFAULT);
    CHECK_VAL!((*data).payload_bad[0], 0x42);
    CHECK_VAL!((*data).payload_bad[1], 0x42);
    CHECK_VAL!((*data).payload_bad[2], 0);
    CHECK_VAL!((*data).payload_bad[3], 0x42);
    CHECK_VAL!((*data).payload_bad[4], 0x42);

    test_varlen__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
