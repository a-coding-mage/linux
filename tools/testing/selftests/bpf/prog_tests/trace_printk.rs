// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020, Oracle and/or its affiliates. */

// C dependencies translated from:
// #include <test_progs.h>
// #include "trace_printk.lskel.h"

use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

const SEARCHMSG: &[u8] = b"testing,testing\0";
const SEARCHMSG_UTF8: &[u8] = "中文,测试\0".as_bytes();

#[repr(C)]
pub struct trace_printk_lskel__rodata {
    pub fmt: [c_char; 0],
}

#[repr(C)]
pub struct trace_printk_lskel__bss {
    pub trace_printk_ran: c_int,
    pub trace_printk_ret: c_int,
    pub trace_printk_utf8_ran: c_int,
    pub trace_printk_utf8_ret: c_int,
    pub trace_printk_invalid_spec_ret: c_int,
}

#[repr(C)]
pub struct trace_printk_lskel {
    pub rodata: *mut trace_printk_lskel__rodata,
    pub bss: *mut trace_printk_lskel__bss,
}

extern "C" {
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn usleep(usec: u32) -> c_int;

    fn trace_printk_lskel__open() -> *mut trace_printk_lskel;
    fn trace_printk_lskel__load(skel: *mut trace_printk_lskel) -> c_int;
    fn trace_printk_lskel__attach(skel: *mut trace_printk_lskel) -> c_int;
    fn trace_printk_lskel__detach(skel: *mut trace_printk_lskel);
    fn trace_printk_lskel__destroy(skel: *mut trace_printk_lskel);

    fn read_trace_pipe_iter(
        cb: unsafe extern "C" fn(str_: *const c_char, data: *mut c_void),
        data: *mut c_void,
        timeout_ms: c_int,
    ) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: i64, right: i64, name: *const c_char) -> bool;
    fn ASSERT_GT(left: i64, right: i64, name: *const c_char) -> bool;
    fn ASSERT_LT(left: i64, right: i64, name: *const c_char) -> bool;
}

unsafe extern "C" fn trace_pipe_cb(str_: *const c_char, data: *mut c_void) {
    if !strstr(str_, SEARCHMSG.as_ptr() as *const c_char).is_null() {
        *(data as *mut c_int).add(0) += 1;
    }
    if !strstr(str_, SEARCHMSG_UTF8.as_ptr() as *const c_char).is_null() {
        *(data as *mut c_int).add(1) += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_trace_printk() {
    let mut bss: *mut trace_printk_lskel__bss;
    let mut skel: *mut trace_printk_lskel;
    let mut err: c_int = 0;
    let mut found: [c_int; 2] = [0; 2];

    skel = trace_printk_lskel__open();
    if !ASSERT_OK_PTR(skel as *const c_void, b"trace_printk__open\0".as_ptr() as *const c_char) {
        return;
    }

    let rodata = (*skel).rodata;
    ASSERT_EQ(
        *((*rodata).fmt.as_mut_ptr().add(0)) as i64,
        b'T' as i64,
        b"skel->rodata->fmt[0]\0".as_ptr() as *const c_char,
    );
    *((*rodata).fmt.as_mut_ptr().add(0)) = b't' as c_char;

    loop {
        err = trace_printk_lskel__load(skel);
        if !ASSERT_OK(err, b"trace_printk__load\0".as_ptr() as *const c_char) {
            break;
        }

        bss = (*skel).bss;

        err = trace_printk_lskel__attach(skel);
        if !ASSERT_OK(err, b"trace_printk__attach\0".as_ptr() as *const c_char) {
            break;
        }

        /* wait for tracepoint to trigger */
        usleep(1);
        trace_printk_lskel__detach(skel);

        if !ASSERT_GT(
            (*bss).trace_printk_ran as i64,
            0,
            b"bss->trace_printk_ran\0".as_ptr() as *const c_char,
        ) {
            break;
        }

        if !ASSERT_GT(
            (*bss).trace_printk_ret as i64,
            0,
            b"bss->trace_printk_ret\0".as_ptr() as *const c_char,
        ) {
            break;
        }

        if !ASSERT_GT(
            (*bss).trace_printk_utf8_ran as i64,
            0,
            b"bss->trace_printk_utf8_ran\0".as_ptr() as *const c_char,
        ) {
            break;
        }

        if !ASSERT_GT(
            (*bss).trace_printk_utf8_ret as i64,
            0,
            b"bss->trace_printk_utf8_ret\0".as_ptr() as *const c_char,
        ) {
            break;
        }

        if !ASSERT_LT(
            (*bss).trace_printk_invalid_spec_ret as i64,
            0,
            b"bss->trace_printk_invalid_spec_ret\0".as_ptr() as *const c_char,
        ) {
            break;
        }

        /* verify our search strings are in the trace buffer */
        ASSERT_OK(
            read_trace_pipe_iter(
                trace_pipe_cb,
                found.as_mut_ptr() as *mut c_void,
                1000,
            ),
            b"read_trace_pipe_iter\0".as_ptr() as *const c_char,
        );

        if !ASSERT_EQ(
            found[0] as i64,
            (*bss).trace_printk_ran as i64,
            b"found\0".as_ptr() as *const c_char,
        ) {
            break;
        }

        if !ASSERT_EQ(
            found[1] as i64,
            (*bss).trace_printk_utf8_ran as i64,
            b"found_utf8\0".as_ptr() as *const c_char,
        ) {
            break;
        }

        break;
    }

    trace_printk_lskel__destroy(skel);
}
