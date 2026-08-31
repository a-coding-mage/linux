// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// Dependencies translated from:
// #include <test_progs.h>
// #include "trace_vprintk.lskel.h"

use core::ffi::{c_char, c_int, c_void};

const SEARCHMSG: &[u8] = b"1,2,3,4,5,6,7,8,9,10\0";

#[repr(C)]
pub struct trace_vprintk_lskel__bss {
    pub trace_vprintk_ran: c_int,
    pub trace_vprintk_ret: c_int,
    pub null_data_vprintk_ret: c_int,
}

#[repr(C)]
pub struct trace_vprintk_lskel {
    pub bss: *mut trace_vprintk_lskel__bss,
}

extern "C" {
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn usleep(usec: u32) -> c_int;

    fn trace_vprintk_lskel__open_and_load() -> *mut trace_vprintk_lskel;
    fn trace_vprintk_lskel__attach(skel: *mut trace_vprintk_lskel) -> c_int;
    fn trace_vprintk_lskel__detach(skel: *mut trace_vprintk_lskel);
    fn trace_vprintk_lskel__destroy(skel: *mut trace_vprintk_lskel);

    fn ASSERT_OK_PTR(ptr: *mut trace_vprintk_lskel, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_LT(actual: c_int, expected: c_int, name: *const c_char) -> bool;

    fn read_trace_pipe_iter(
        cb: Option<unsafe extern "C" fn(str_: *const c_char, data: *mut c_void)>,
        data: *mut c_void,
        timeout_ms: c_int,
    ) -> c_int;
}

unsafe extern "C" fn trace_pipe_cb(str_: *const c_char, data: *mut c_void) {
    if !strstr(str_, SEARCHMSG.as_ptr() as *const c_char).is_null() {
        *(data as *mut c_int) += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_trace_vprintk() {
    let mut bss: *mut trace_vprintk_lskel__bss;
    let mut skel: *mut trace_vprintk_lskel;
    let mut err: c_int = 0;
    let mut found: c_int = 0;

    skel = trace_vprintk_lskel__open_and_load();
    if !ASSERT_OK_PTR(
        skel,
        b"trace_vprintk__open_and_load\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(skel);
        return;
    }

    bss = (*skel).bss;

    err = trace_vprintk_lskel__attach(skel);
    if !ASSERT_OK(err, b"trace_vprintk__attach\0".as_ptr() as *const c_char) {
        goto_cleanup(skel);
        return;
    }

    /* wait for tracepoint to trigger */
    usleep(1);
    trace_vprintk_lskel__detach(skel);

    if !ASSERT_GT(
        (*bss).trace_vprintk_ran,
        0,
        b"bss->trace_vprintk_ran\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(skel);
        return;
    }

    if !ASSERT_GT(
        (*bss).trace_vprintk_ret,
        0,
        b"bss->trace_vprintk_ret\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(skel);
        return;
    }

    /* verify our search string is in the trace buffer */
    ASSERT_OK(
        read_trace_pipe_iter(
            Some(trace_pipe_cb),
            &mut found as *mut c_int as *mut c_void,
            1000,
        ),
        b"read_trace_pipe_iter\0".as_ptr() as *const c_char,
    );

    if !ASSERT_EQ(
        found,
        (*bss).trace_vprintk_ran,
        b"found\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(skel);
        return;
    }

    if !ASSERT_LT(
        (*bss).null_data_vprintk_ret,
        0,
        b"bss->null_data_vprintk_ret\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(skel);
        return;
    }

    goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut trace_vprintk_lskel) {
    trace_vprintk_lskel__destroy(skel);
}
