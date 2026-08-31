// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

/* Dependencies from:
 * <test_progs.h>
 * <network_helpers.h>
 * "type_cast.skel.h"
 */

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct type_cast_bss {
    pub ifindex: c_int,
    pub ingress_ifindex: c_int,
    pub name: [c_char; 16],
    pub inum: u64,
    pub meta_len: c_uint,
    pub frag0_len: c_uint,
    pub kskb_len: c_uint,
    pub kskb2_len: c_uint,
}

#[repr(C)]
pub struct type_cast_progs {
    pub md_xdp: *mut bpf_program,
    pub md_skb: *mut bpf_program,
}

#[repr(C)]
pub struct type_cast {
    pub obj: *mut bpf_object,
    pub progs: type_cast_progs,
    pub bss: *mut type_cast_bss,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *const c_void,
    pub data_size_in: u32,
    pub data_out: *mut c_void,
    pub data_size_out: u32,
    pub retval: u32,
    pub repeat: u32,
}

unsafe extern "C" {
    static pkt_v4: c_void;
    static XDP_PASS: c_uint;

    fn type_cast__open() -> *mut type_cast;
    fn type_cast__load(skel: *mut type_cast) -> c_int;
    fn type_cast__destroy(skel: *mut type_cast);

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: u64, expected: u64, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: u64, expected: u64, name: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
}

unsafe fn test_xdp() {
    let mut skel: *mut type_cast;
    let mut err: c_int;
    let prog_fd: c_int;
    let mut buf = [0 as c_char; 128];

    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: ptr::addr_of!(pkt_v4),
        data_size_in: size_of::<c_void>() as u32,
        data_out: buf.as_mut_ptr() as *mut c_void,
        data_size_out: size_of_val(&buf) as u32,
        retval: 0,
        repeat: 1,
    };

    skel = type_cast__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    bpf_program__set_autoload((*skel).progs.md_xdp, true);
    err = type_cast__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        type_cast__destroy(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.md_xdp);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval as u64, XDP_PASS as u64, c"xdp test_run retval".as_ptr());

    ASSERT_EQ((*(*skel).bss).ifindex as u64, 1, c"xdp_md ifindex".as_ptr());
    ASSERT_EQ(
        (*(*skel).bss).ifindex as u64,
        (*(*skel).bss).ingress_ifindex as u64,
        c"xdp_md ingress_ifindex".as_ptr(),
    );
    ASSERT_STREQ(
        (*(*skel).bss).name.as_ptr(),
        c"lo".as_ptr(),
        c"xdp_md name".as_ptr(),
    );
    ASSERT_NEQ((*(*skel).bss).inum, 0, c"xdp_md inum".as_ptr());

    type_cast__destroy(skel);
}

unsafe fn test_tc() {
    let mut skel: *mut type_cast;
    let mut err: c_int;
    let prog_fd: c_int;

    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: ptr::addr_of!(pkt_v4),
        data_size_in: size_of::<c_void>() as u32,
        data_out: ptr::null_mut(),
        data_size_out: 0,
        retval: 0,
        repeat: 1,
    };

    skel = type_cast__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    bpf_program__set_autoload((*skel).progs.md_skb, true);
    err = type_cast__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        type_cast__destroy(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.md_skb);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval as u64, 0, c"tc test_run retval".as_ptr());

    ASSERT_EQ((*(*skel).bss).meta_len as u64, 0, c"skb meta_len".as_ptr());
    ASSERT_EQ((*(*skel).bss).frag0_len as u64, 0, c"skb frag0_len".as_ptr());
    ASSERT_NEQ((*(*skel).bss).kskb_len as u64, 0, c"skb len".as_ptr());
    ASSERT_NEQ((*(*skel).bss).kskb2_len as u64, 0, c"skb2 len".as_ptr());
    ASSERT_EQ(
        (*(*skel).bss).kskb_len as u64,
        (*(*skel).bss).kskb2_len as u64,
        c"skb len compare".as_ptr(),
    );

    type_cast__destroy(skel);
}

const NEGATIVE_TESTS: [&[u8]; 2] = [b"untrusted_ptr\0", b"kctx_u64\0"];

unsafe fn test_negative() {
    let mut prog: *mut bpf_program;
    let mut skel: *mut type_cast;
    let mut err: c_int;

    for i in 0..NEGATIVE_TESTS.len() {
        skel = type_cast__open();
        if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
            return;
        }

        prog = bpf_object__find_program_by_name(
            (*skel).obj,
            NEGATIVE_TESTS[i].as_ptr() as *const c_char,
        );
        if !ASSERT_OK_PTR(
            prog as *const c_void,
            c"bpf_object__find_program_by_name".as_ptr(),
        ) {
            type_cast__destroy(skel);
            continue;
        }
        bpf_program__set_autoload(prog, true);
        err = type_cast__load(skel);
        ASSERT_ERR(err, c"skel_load".as_ptr());

        type_cast__destroy(skel);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_type_cast() {
    if test__start_subtest(c"xdp".as_ptr()) {
        test_xdp();
    }
    if test__start_subtest(c"tc".as_ptr()) {
        test_tc();
    }
    if test__start_subtest(c"negative".as_ptr()) {
        test_negative();
    }
}
