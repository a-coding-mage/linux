// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Christian Brauner */

/*
 * C dependencies:
 * #include <vmlinux.h>
 * #include <bpf/bpf_tracing.h>
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_core_read.h>
 * #include "bpf_experimental.h"
 * #include "bpf_misc.h"
 */

pub type __u32 = u32;

#[repr(C)]
pub struct socket {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_dynptr_from_mem(
        data: *mut core::ffi::c_void,
        size: __u32,
        flags: __u64,
        ptr: *mut bpf_dynptr,
    ) -> core::ffi::c_long;
    fn bpf_sock_read_xattr(
        sock: *mut socket,
        name: *const core::ffi::c_char,
        value_ptr: *mut bpf_dynptr,
    ) -> core::ffi::c_int;
    fn bpf_get_current_pid_tgid() -> __u64;
}

pub type __u64 = u64;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as _, b'P' as _, b'L' as _, 0];

#[no_mangle]
pub static mut value: [core::ffi::c_char; 16] = [0; 16];

#[no_mangle]
pub static mut read_ret: core::ffi::c_int = -1;

#[no_mangle]
pub static mut monitored_pid: __u32 = 0;

#[inline(always)]
unsafe fn read_xattr(sock: *mut socket) {
    let mut value_ptr: bpf_dynptr = core::mem::zeroed();

    bpf_dynptr_from_mem(
        value.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&value) as __u32,
        0,
        &mut value_ptr,
    );
    bpf_sock_read_xattr(sock, b"user.bpf_test\0".as_ptr() as *const core::ffi::c_char, &mut value_ptr);
}

#[no_mangle]
#[link_section = "lsm.s/socket_connect"]
/* __success */
pub unsafe extern "C" fn trusted_sock_ptr_sleepable(sock: *mut socket) -> core::ffi::c_int {
    read_xattr(sock);
    0
}

#[no_mangle]
#[link_section = "lsm/socket_connect"]
/* __success */
pub unsafe extern "C" fn trusted_sock_ptr_non_sleepable(sock: *mut socket) -> core::ffi::c_int {
    read_xattr(sock);
    0
}

#[no_mangle]
#[link_section = "lsm.s/socket_connect"]
/* __success */
pub unsafe extern "C" fn read_sock_xattr(sock: *mut socket) -> core::ffi::c_int {
    let mut value_ptr: bpf_dynptr = core::mem::zeroed();
    let pid: __u32 = (bpf_get_current_pid_tgid() >> 32) as __u32;

    if pid != monitored_pid {
        return 0;
    }

    bpf_dynptr_from_mem(
        value.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&value) as __u32,
        0,
        &mut value_ptr,
    );
    read_ret =
        bpf_sock_read_xattr(sock, b"user.bpf_test\0".as_ptr() as *const core::ffi::c_char, &mut value_ptr);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
