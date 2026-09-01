// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 Yafang Shao <laoar.shao@gmail.com> */

// C dependencies translated as external declarations:
// #include <string.h>
// #include <linux/bpf.h>
// #include <test_progs.h>
// #include "test_ptr_untrusted.skel.h"

use core::ffi::{c_char, c_int, c_void};

const TP_NAME: &[u8] = b"sched_switch\0";

#[repr(C)]
pub struct test_ptr_untrusted {
    pub links: test_ptr_untrusted__links,
    pub progs: test_ptr_untrusted__progs,
    pub bss: *mut test_ptr_untrusted__bss,
}

#[repr(C)]
pub struct test_ptr_untrusted__links {
    pub lsm_run: *mut c_void,
    pub raw_tp_run: *mut c_void,
}

#[repr(C)]
pub struct test_ptr_untrusted__progs {
    pub lsm_run: *mut c_void,
    pub raw_tp_run: *mut c_void,
}

#[repr(C)]
pub struct test_ptr_untrusted__bss {
    pub tp_name: *const c_char,
}

extern "C" {
    fn test_ptr_untrusted__open_and_load() -> *mut test_ptr_untrusted;
    fn test_ptr_untrusted__destroy(skel: *mut test_ptr_untrusted);
    fn bpf_program__attach_lsm(prog: *mut c_void) -> *mut c_void;
    fn bpf_program__attach_raw_tracepoint(prog: *mut c_void, tp_name: *const c_char) -> *mut c_void;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char);
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_ptr_untrusted() {
    let mut skel: *mut test_ptr_untrusted;
    let err: c_int;

    skel = test_ptr_untrusted__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, b"skel_open\0".as_ptr() as *const c_char) {
        goto_cleanup(skel);
        return;
    }

    /* First, attach lsm prog */
    (*skel).links.lsm_run = bpf_program__attach_lsm((*skel).progs.lsm_run);
    if !ASSERT_OK_PTR(
        (*skel).links.lsm_run,
        b"lsm_attach\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(skel);
        return;
    }

    /* Second, attach raw_tp prog. The lsm prog will be triggered. */
    (*skel).links.raw_tp_run = bpf_program__attach_raw_tracepoint(
        (*skel).progs.raw_tp_run,
        TP_NAME.as_ptr() as *const c_char,
    );
    if !ASSERT_OK_PTR(
        (*skel).links.raw_tp_run,
        b"raw_tp_attach\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(skel);
        return;
    }

    err = strncmp(
        (*(*skel).bss).tp_name,
        TP_NAME.as_ptr() as *const c_char,
        strlen(TP_NAME.as_ptr() as *const c_char),
    );
    ASSERT_EQ(err, 0, b"cmp_tp_name\0".as_ptr() as *const c_char);

    goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut test_ptr_untrusted) {
    test_ptr_untrusted__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
