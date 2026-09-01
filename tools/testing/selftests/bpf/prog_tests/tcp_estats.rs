// SPDX-License-Identifier: GPL-2.0
// C dependency: #include <test_progs.h>

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct bpf_object {
    _unused: [u8; 0],
}

unsafe extern "C" {
    static BPF_PROG_TYPE_TRACEPOINT: c_int;

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn bpf_object__close(obj: *mut bpf_object);
}

pub unsafe fn test_tcp_estats() {
    let file: *const c_char = b"./test_tcp_estats.bpf.o\0".as_ptr() as *const c_char;
    let mut err: c_int;
    let mut prog_fd: c_int = 0;
    let mut obj: *mut bpf_object = core::ptr::null_mut();

    err = unsafe {
        bpf_prog_test_load(
            file,
            BPF_PROG_TYPE_TRACEPOINT,
            &mut obj,
            &mut prog_fd,
        )
    };
    if !unsafe { ASSERT_OK(err, b"\0".as_ptr() as *const c_char) } {
        return;
    }

    unsafe {
        bpf_object__close(obj);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
