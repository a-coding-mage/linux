// SPDX-License-Identifier: GPL-2.0
// Dependencies in the original C source:
// #include <test_progs.h>
// #include <network_helpers.h>
// #include "skb_load_bytes.skel.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct skb_load_bytes {
    pub progs: skb_load_bytes__progs,
    pub bss: *mut skb_load_bytes__bss,
}

#[repr(C)]
pub struct skb_load_bytes__progs {
    pub skb_process: *mut bpf_program,
}

#[repr(C)]
pub struct skb_load_bytes__bss {
    pub load_offset: u32,
    pub test_result: c_int,
}

#[repr(C)]
pub struct bpf_program {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *mut c_void,
    pub data_size_in: u32,
    pub ctx_in: *mut c_void,
    pub ctx_size_in: u32,
}

const EFAULT: c_int = 14;

extern "C" {
    static mut pkt_v4: [u8; 0];

    fn skb_load_bytes__open_and_load() -> *mut skb_load_bytes;
    fn skb_load_bytes__destroy(skel: *mut skb_load_bytes);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

pub unsafe extern "C" fn test_skb_load_bytes() {
    let mut skel: *mut skb_load_bytes;
    let mut err: c_int;
    let prog_fd: c_int;
    let mut test_result: c_int;
    let mut skb: __sk_buff = core::mem::zeroed();

    let mut tattr = bpf_test_run_opts {
        data_in: pkt_v4.as_mut_ptr() as *mut c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as u32,
        ctx_in: (&mut skb as *mut __sk_buff).cast::<c_void>(),
        ctx_size_in: core::mem::size_of_val(&skb) as u32,
    };

    skel = skb_load_bytes__open_and_load();
    if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"skel_open_and_load".as_ptr()) {
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.skb_process);
    if !ASSERT_GE(prog_fd, 0, c"prog_fd".as_ptr()) {
        goto_out(skel);
        return;
    }

    (*(*skel).bss).load_offset = (-1i32) as u32;
    err = bpf_prog_test_run_opts(prog_fd, &mut tattr);
    if !ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr()) {
        goto_out(skel);
        return;
    }
    test_result = (*(*skel).bss).test_result;
    if !ASSERT_EQ(test_result, -EFAULT, c"offset -1".as_ptr()) {
        goto_out(skel);
        return;
    }

    (*(*skel).bss).load_offset = 10u32;
    err = bpf_prog_test_run_opts(prog_fd, &mut tattr);
    if !ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr()) {
        goto_out(skel);
        return;
    }
    test_result = (*(*skel).bss).test_result;
    if !ASSERT_EQ(test_result, 0, c"offset 10".as_ptr()) {
        goto_out(skel);
        return;
    }

    goto_out(skel);
}

unsafe fn goto_out(skel: *mut skb_load_bytes) {
    skb_load_bytes__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
