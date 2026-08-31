// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external Rust dependencies:
// <test_progs.h>
// <network_helpers.h>

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of_val;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const c_void,
    pub data_size_in: u32,
    pub data_size_out: u32,
    pub retval: u32,
    pub repeat: u32,
}

pub const BPF_PROG_TYPE_SCHED_CLS: c_int = 3;

unsafe extern "C" {
    static pkt_v4: [u8; 0];
    static pkt_v6: [u8; 0];

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn CHECK_FAIL(condition: c_int) -> bool;
    fn ASSERT_OK(condition: c_int, name: *const c_char);
}

pub unsafe fn test_pkt_access() {
    let file = b"./test_pkt_access.bpf.o\0".as_ptr() as *const c_char;
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut err: c_int;
    let mut prog_fd: c_int = 0;
    let mut topts = bpf_test_run_opts {
        data_in: (&raw const pkt_v4) as *const c_void,
        data_size_in: size_of_val(&pkt_v4) as u32,
        data_size_out: 0,
        retval: 0,
        repeat: 100000,
    };

    err = bpf_prog_test_load(file, BPF_PROG_TYPE_SCHED_CLS, &mut obj, &mut prog_fd);
    if CHECK_FAIL(err) {
        return;
    }

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, b"ipv4 test_run_opts err\0".as_ptr() as *const c_char);
    ASSERT_OK(
        topts.retval as c_int,
        b"ipv4 test_run_opts retval\0".as_ptr() as *const c_char,
    );

    topts.data_in = (&raw const pkt_v6) as *const c_void;
    topts.data_size_in = size_of_val(&pkt_v6) as u32;
    topts.data_size_out = 0; /* reset from last call */
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, b"ipv6 test_run_opts err\0".as_ptr() as *const c_char);
    ASSERT_OK(
        topts.retval as c_int,
        b"ipv6 test_run_opts retval\0".as_ptr() as *const c_char,
    );

    bpf_object__close(obj);
}
