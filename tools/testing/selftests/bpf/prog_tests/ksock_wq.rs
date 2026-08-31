// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Isovalent */

/* C includes translated as external dependencies:
 * <unistd.h>
 * "test_progs.h"
 * "ksock_wq.skel.h"
 */

pub const CALLBACK_WAIT_RETRIES: i32 = 1000;
pub const CALLBACK_WAIT_US: u32 = 1000;

pub type u32 = ::std::os::raw::c_uint;

pub const EOPNOTSUPP: i32 = 95;
pub const __ATOMIC_ACQUIRE: i32 = 2;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: u32,
}

#[repr(C)]
pub struct ksock_wq {
    pub progs: ksock_wq__progs,
    pub bss: *mut ksock_wq__bss,
}

#[repr(C)]
pub struct ksock_wq__progs {
    pub ksock_wq_start: *mut bpf_program,
}

#[repr(C)]
pub struct ksock_wq__bss {
    pub callback_done: u32,
    pub create_err: i32,
}

#[repr(C)]
pub struct bpf_program {
    _unused: [u8; 0],
}

extern "C" {
    pub fn ksock_wq__open_and_load() -> *mut ksock_wq;
    pub fn ksock_wq__destroy(obj: *mut ksock_wq);
    pub fn bpf_program__fd(prog: *mut bpf_program) -> ::std::os::raw::c_int;
    pub fn bpf_prog_test_run_opts(
        prog_fd: ::std::os::raw::c_int,
        opts: *mut bpf_test_run_opts,
    ) -> ::std::os::raw::c_int;
    pub fn usleep(usec: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;

    pub fn ASSERT_OK_PTR(ptr: *mut ksock_wq, name: *const ::std::os::raw::c_char) -> bool;
    pub fn ASSERT_OK(err: ::std::os::raw::c_int, name: *const ::std::os::raw::c_char) -> bool;
    pub fn ASSERT_EQ(
        actual: ::std::os::raw::c_int,
        expected: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> bool;
}

unsafe fn __atomic_load_n_u32(ptr: *const u32, _memorder: i32) -> u32 {
    ::std::ptr::read_volatile(ptr)
}

pub unsafe extern "C" fn test_ksock_wq() {
    let mut opts: bpf_test_run_opts = ::std::mem::zeroed();
    let mut skel: *mut ksock_wq;
    let callback_done: u32;
    let mut err: i32;
    let mut i: i32;

    skel = ksock_wq__open_and_load();
    if !ASSERT_OK_PTR(skel, b"ksock_wq open and load\0".as_ptr() as *const _) {
        return;
    }

    err = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.ksock_wq_start),
        &mut opts,
    );
    if !ASSERT_OK(err, b"run ksock_wq_start\0".as_ptr() as *const _) {
        goto_out(skel);
        return;
    }
    if !ASSERT_OK(opts.retval as i32, b"ksock_wq_start retval\0".as_ptr() as *const _) {
        goto_out(skel);
        return;
    }

    i = 0;
    while i < CALLBACK_WAIT_RETRIES {
        if __atomic_load_n_u32(&(*(*skel).bss).callback_done, __ATOMIC_ACQUIRE) != 0 {
            break;
        }
        usleep(CALLBACK_WAIT_US);
        i += 1;
    }
    callback_done = __atomic_load_n_u32(&(*(*skel).bss).callback_done, __ATOMIC_ACQUIRE);
    if !ASSERT_EQ(
        callback_done as ::std::os::raw::c_int,
        1,
        b"workqueue callback completed\0".as_ptr() as *const _,
    ) {
        goto_out(skel);
        return;
    }

    ASSERT_EQ(
        (*(*skel).bss).create_err,
        -EOPNOTSUPP,
        b"workqueue create rejected\0".as_ptr() as *const _,
    );

    goto_out(skel);
}

unsafe fn goto_out(skel: *mut ksock_wq) {
    ksock_wq__destroy(skel);
}
