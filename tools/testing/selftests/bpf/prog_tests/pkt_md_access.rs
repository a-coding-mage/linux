// SPDX-License-Identifier: GPL-2.0
// C dependencies translated from:
// #include <test_progs.h>
// #include <network_helpers.h>

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *const c_void,
    pub data_size_in: u32,
    pub repeat: u32,
    pub retval: u32,
}

unsafe extern "C" {
    static pkt_v4: [u8; 0];

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        pobj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn CHECK_FAIL(cond: c_int) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char);
}

const BPF_PROG_TYPE_SCHED_CLS: c_int = 3;

#[no_mangle]
pub unsafe extern "C" fn test_pkt_md_access() {
    let file = b"./test_pkt_md_access.bpf.o\0";
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut prog_fd: c_int = 0;
    let mut err: c_int;

    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: (&pkt_v4 as *const [u8; 0]).cast::<c_void>(),
        data_size_in: core::mem::size_of_val(&pkt_v4) as u32,
        repeat: 10,
        retval: 0,
    };

    err = bpf_prog_test_load(
        file.as_ptr().cast::<c_char>(),
        BPF_PROG_TYPE_SCHED_CLS,
        &mut obj,
        &mut prog_fd,
    );
    if CHECK_FAIL(err) {
        return;
    }

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run_opts err".as_ptr());
    ASSERT_OK(topts.retval as c_int, c"test_run_opts retval".as_ptr());

    bpf_object__close(obj);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
