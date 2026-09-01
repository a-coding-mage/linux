// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C dependencies from:
// #include <test_progs.h>
// #include <network_helpers.h>
// #include "skb_pkt_end.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u32 = c_uint;

const BPF_F_TEST_SKB_CHECKSUM_COMPLETE: c_uint = 1 << 0;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct skb_pkt_end_progs {
    pub main_prog: *mut bpf_program,
}

#[repr(C)]
pub struct skb_pkt_end {
    pub progs: skb_pkt_end_progs,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *const c_void,
    pub data_out: *mut c_void,
    pub data_size_in: c_uint,
    pub data_size_out: c_uint,
    pub retval: c_uint,
    pub repeat: c_uint,
    pub duration: c_uint,
    pub ctx_in: *const c_void,
    pub ctx_out: *mut c_void,
    pub ctx_size_in: c_uint,
    pub ctx_size_out: c_uint,
    pub flags: c_uint,
    pub cpu: c_uint,
    pub batch_size: c_uint,
}

extern "C" {
    static pkt_v4: [u8; 0];

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn skb_pkt_end__open_and_load() -> *mut skb_pkt_end;
    fn skb_pkt_end__attach(skel: *mut skb_pkt_end) -> c_int;
    fn skb_pkt_end__destroy(skel: *mut skb_pkt_end);

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;
    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
}

unsafe fn sanity_run(prog: *mut bpf_program) -> c_int {
    let mut err: c_int;
    let prog_fd: c_int;
    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: pkt_v4.as_ptr() as *const c_void,
        data_out: core::ptr::null_mut(),
        data_size_in: core::mem::size_of_val(&pkt_v4) as c_uint,
        data_size_out: 0,
        retval: 0,
        repeat: 1,
        duration: 0,
        ctx_in: core::ptr::null(),
        ctx_out: core::ptr::null_mut(),
        ctx_size_in: 0,
        ctx_size_out: 0,
        flags: BPF_F_TEST_SKB_CHECKSUM_COMPLETE,
        cpu: 0,
        batch_size: 0,
    };

    prog_fd = bpf_program__fd(prog);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, b"test_run\0".as_ptr() as *const c_char) {
        return -1;
    }
    if !ASSERT_EQ(topts.retval, 123, b"test_run retval\0".as_ptr() as *const c_char) {
        return -1;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn test_test_skb_pkt_end() {
    let mut skb_pkt_end_skel: *mut skb_pkt_end = core::ptr::null_mut();
    let mut _duration: __u32 = 0;
    let mut err: c_int;

    skb_pkt_end_skel = skb_pkt_end__open_and_load();
    if CHECK(
        skb_pkt_end_skel.is_null(),
        b"skb_pkt_end_skel_load\0".as_ptr() as *const c_char,
        b"skb_pkt_end skeleton failed\n\0".as_ptr() as *const c_char,
    ) {
        skb_pkt_end__destroy(skb_pkt_end_skel);
        return;
    }

    err = skb_pkt_end__attach(skb_pkt_end_skel);
    if CHECK(
        err != 0,
        b"skb_pkt_end_attach\0".as_ptr() as *const c_char,
        b"skb_pkt_end attach failed: %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        skb_pkt_end__destroy(skb_pkt_end_skel);
        return;
    }

    if sanity_run((*skb_pkt_end_skel).progs.main_prog) != 0 {
        skb_pkt_end__destroy(skb_pkt_end_skel);
        return;
    }

    skb_pkt_end__destroy(skb_pkt_end_skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
