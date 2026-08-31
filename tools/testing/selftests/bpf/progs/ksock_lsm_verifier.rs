// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Isovalent */

// Dependencies from the original C includes:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, "bpf_misc.h",
// and "ksock_common.h".

#[repr(C)]
pub struct socket {
    _private: [u8; 0],
}

#[repr(C)]
pub struct msghdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_ksock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __ksock_ctx_value {
    pub ctx: *mut bpf_ksock,
}

unsafe extern "C" {
    fn ksock_ctx_value_lookup() -> *mut __ksock_ctx_value;
    fn bpf_kptr_xchg(kptr: *mut *mut bpf_ksock, ptr: *mut bpf_ksock) -> *mut bpf_ksock;
    fn bpf_ksock_send(ks: *mut bpf_ksock, data: *mut u8, size: u64) -> i64;
    fn bpf_ksock_release(ks: *mut bpf_ksock);
}

#[unsafe(no_mangle)]
pub static mut send_data: [u8; 11] = *b"dummy data\0";

// SEC("lsm.s/socket_sendmsg")
// __description("bpf_ksock_send is rejected from socket_sendmsg LSM hook")
// __failure __msg("calling kernel function bpf_ksock_send is not allowed")
#[unsafe(no_mangle)]
#[unsafe(link_section = "lsm.s/socket_sendmsg")]
pub unsafe extern "C" fn ksock_socket_sendmsg(
    sock: *mut socket,
    msg: *mut msghdr,
    size: i32,
    ret: i32,
) -> i32 {
    let mut v: *mut __ksock_ctx_value;
    let mut ks: *mut bpf_ksock;

    let _ = sock;
    let _ = msg;
    let _ = size;

    v = unsafe { ksock_ctx_value_lookup() };
    if v.is_null() {
        return ret;
    }

    ks = unsafe { bpf_kptr_xchg(core::ptr::addr_of_mut!((*v).ctx), core::ptr::null_mut()) };
    if ks.is_null() {
        return ret;
    }

    unsafe {
        bpf_ksock_send(
            ks,
            core::ptr::addr_of_mut!(send_data) as *mut u8,
            core::mem::size_of_val(&send_data) as u64,
        );
        bpf_ksock_release(ks);
    }

    return ret;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static __license: [u8; 4] = *b"GPL\0";
