// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <test_progs.h>

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *mut c_char,
    pub data_size_in: c_uint,
    pub data_out: *mut c_char,
    pub data_size_out: c_uint,
    pub repeat: c_uint,
    pub retval: c_uint,
}

unsafe extern "C" {
    static BPF_PROG_TYPE_XDP: c_int;
    static XDP_PASS: c_uint;

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
    fn ASSERT_EQ(left: c_uint, right: c_uint, name: *const c_char);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp_perf() {
    let file: *const c_char = c"./xdp_dummy.bpf.o".as_ptr();
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut in_: [c_char; 128] = [0; 128];
    let mut out: [c_char; 128] = [0; 128];
    let mut err: c_int;
    let mut prog_fd: c_int = 0;

    // C used LIBBPF_OPTS(bpf_test_run_opts, topts, ...).
    let mut topts = bpf_test_run_opts {
        data_in: in_.as_mut_ptr(),
        data_size_in: core::mem::size_of_val(&in_) as c_uint,
        data_out: out.as_mut_ptr(),
        data_size_out: core::mem::size_of_val(&out) as c_uint,
        repeat: 1000000,
        retval: 0,
    };

    err = bpf_prog_test_load(file, BPF_PROG_TYPE_XDP, &mut obj, &mut prog_fd);
    if CHECK_FAIL(err) {
        return;
    }

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval, XDP_PASS, c"test_run retval".as_ptr());
    ASSERT_EQ(topts.data_size_out, 128, c"test_run data_size_out".as_ptr());

    bpf_object__close(obj);
}
