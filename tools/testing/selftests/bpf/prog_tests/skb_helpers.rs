// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, <network_helpers.h>

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, size_of_val, zeroed};

unsafe extern "C" {
    static pkt_v4: c_void;

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        pobj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

unsafe extern "C" {
    static BPF_PROG_TYPE_SCHED_CLS: c_int;
}

pub unsafe fn test_skb_helpers() {
    let mut skb: __sk_buff = unsafe { zeroed() };
    skb.wire_len = 100;
    skb.gso_segs = 8;
    skb.gso_size = 10;

    // LIBBPF_OPTS(bpf_test_run_opts, topts, ...)
    let mut topts: bpf_test_run_opts = unsafe { zeroed() };
    topts.sz = size_of::<bpf_test_run_opts>() as _;
    topts.data_in = unsafe { &pkt_v4 as *const _ as *const c_void };
    topts.data_size_in = unsafe { size_of_val(&pkt_v4) as _ };
    topts.ctx_in = &mut skb as *mut _ as *mut c_void;
    topts.ctx_size_in = size_of::<__sk_buff>() as _;
    topts.ctx_out = &mut skb as *mut _ as *mut c_void;
    topts.ctx_size_out = size_of::<__sk_buff>() as _;

    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut err: c_int;
    let mut prog_fd: c_int = 0;

    err = unsafe {
        bpf_prog_test_load(
            b"./test_skb_helpers.bpf.o\0".as_ptr() as *const c_char,
            BPF_PROG_TYPE_SCHED_CLS,
            &mut obj,
            &mut prog_fd,
        )
    };
    if !unsafe { ASSERT_OK(err, b"load\0".as_ptr() as *const c_char) } {
        return;
    }
    err = unsafe { bpf_prog_test_run_opts(prog_fd, &mut topts) };
    unsafe {
        ASSERT_OK(err, b"test_run\0".as_ptr() as *const c_char);
        bpf_object__close(obj);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
