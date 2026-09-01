// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */
// C dependencies: <test_progs.h>, <network_helpers.h>
// C dependency: "jit_probe_mem.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *mut c_void,
    pub data_out: *mut c_void,
    pub data_size_in: c_uint,
    pub data_size_out: c_uint,
    pub ctx_in: *mut c_void,
    pub ctx_out: *mut c_void,
    pub ctx_size_in: c_uint,
    pub ctx_size_out: c_uint,
    pub retval: c_uint,
    pub duration: c_uint,
    pub repeat: c_uint,
    pub flags: c_uint,
    pub cpu: c_uint,
    pub batch_size: c_uint,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct jit_probe_mem {
    pub progs: jit_probe_mem__progs,
    pub data: *mut jit_probe_mem__data,
}

#[repr(C)]
pub struct jit_probe_mem__progs {
    pub test_jit_probe_mem: *mut bpf_program,
}

#[repr(C)]
pub struct jit_probe_mem__data {
    pub total_sum: c_int,
}

unsafe extern "C" {
    static mut pkt_v4: [u8; 0];

    fn jit_probe_mem__open_and_load() -> *mut jit_probe_mem;
    fn jit_probe_mem__destroy(skel: *mut jit_probe_mem);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

pub unsafe extern "C" fn test_jit_probe_mem() {
    let mut opts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: core::ptr::addr_of_mut!(pkt_v4) as *mut c_void,
        data_out: core::ptr::null_mut(),
        data_size_in: core::mem::size_of_val(&pkt_v4) as c_uint,
        data_size_out: 0,
        ctx_in: core::ptr::null_mut(),
        ctx_out: core::ptr::null_mut(),
        ctx_size_in: 0,
        ctx_size_out: 0,
        retval: 0,
        duration: 0,
        repeat: 1,
        flags: 0,
        cpu: 0,
        batch_size: 0,
    };
    let mut skel: *mut jit_probe_mem;
    let ret: c_int;

    skel = jit_probe_mem__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"jit_probe_mem__open_and_load".as_ptr()) {
        return;
    }

    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_jit_probe_mem),
        &mut opts,
    );
    ASSERT_OK(ret, c"jit_probe_mem ret".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"jit_probe_mem opts.retval".as_ptr());
    ASSERT_EQ(
        (*(*skel).data).total_sum,
        192,
        c"jit_probe_mem total_sum".as_ptr(),
    );

    jit_probe_mem__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
