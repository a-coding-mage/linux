// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Isovalent */

/*
 * Dependencies from the original C source:
 * "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
 * <bpf/bpf_endian.h>, "bpf_tracing_net.h", and "ksock_common.h".
 */

type __be32 = u32;
type __u16 = u16;

const AF_INET: i32 = 2;
const SOCK_DGRAM: i32 = 2;
const IPPROTO_UDP: i32 = 17;
const EEXIST: i32 = 17;
const ENOENT: i32 = 2;

#[repr(C)]
pub struct bpf_ksock_create_opts {
    pub family: i32,
    pub type_: i32,
    pub protocol: i32,
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: __be32,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: i32,
    pub sin_port: __u16,
    pub sin_addr: in_addr,
}

#[repr(C)]
pub union bpf_ksock_addr {
    pub sin: sockaddr_in,
}

#[repr(C)]
pub struct bpf_ksock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct socket {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_ksock_create(
        create_opts: *mut bpf_ksock_create_opts,
        create_opts__sz: u32,
        err: *mut i32,
    ) -> *mut bpf_ksock;
    fn bpf_ksock_connect(ks: *mut bpf_ksock, addr: *mut bpf_ksock_addr, addr__sz: u32) -> i32;
    fn bpf_ksock_release(ks: *mut bpf_ksock);
    fn ksock_ctx_insert(ks: *mut bpf_ksock) -> i32;
    fn ksock_ctx_get() -> *mut bpf_ksock;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_ksock_send(ks: *mut bpf_ksock, data: *mut u8, len: u32) -> i32;
    fn bpf_htons(port: __u16) -> __u16;
}

#[no_mangle]
pub static mut send_data: [u8; 32] = *b"hello from bpf ksock\0\0\0\0\0\0\0\0\0\0\0";

#[no_mangle]
pub static mut ipv4_remote: __be32 = 0;

#[no_mangle]
pub static mut remote_port: __u16 = 0;

#[no_mangle]
pub static mut target_pid: i32 = 0;

#[no_mangle]
pub static mut send_ret: i32 = -1;

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn ksock_setup(ctx: *mut core::ffi::c_void) -> i32 {
    let mut create_opts: bpf_ksock_create_opts = core::mem::zeroed();
    let mut addr: bpf_ksock_addr = core::mem::zeroed();
    let mut err: i32 = 0;

    create_opts.family = AF_INET;
    create_opts.type_ = SOCK_DGRAM;
    create_opts.protocol = IPPROTO_UDP;

    let ks = bpf_ksock_create(
        &mut create_opts,
        core::mem::size_of_val(&create_opts) as u32,
        &mut err,
    );
    if ks.is_null() {
        return err;
    }

    addr.sin.sin_family = AF_INET;
    addr.sin.sin_port = bpf_htons(remote_port);
    addr.sin.sin_addr.s_addr = ipv4_remote;

    err = bpf_ksock_connect(ks, &mut addr, core::mem::size_of_val(&addr) as u32);
    if err != 0 {
        bpf_ksock_release(ks);
        return err;
    }

    err = ksock_ctx_insert(ks);
    if err != 0 && err != -EEXIST {
        return err;
    }
    return 0;
}

#[no_mangle]
#[link_section = "lsm.s/socket_bind"]
pub unsafe extern "C" fn ksock_socket_bind(
    sock: *mut socket,
    address: *mut sockaddr,
    addrlen: i32,
    ret: i32,
) -> i32 {
    let mut ks: *mut bpf_ksock;
    let pid: u32 = (bpf_get_current_pid_tgid() >> 32) as u32;

    if ret != 0 || pid != target_pid as u32 {
        return ret;
    }

    ks = ksock_ctx_get();
    if ks.is_null() {
        send_ret = -ENOENT;
        return ret;
    }

    send_ret = bpf_ksock_send(ks, send_data.as_mut_ptr(), core::mem::size_of_val(&send_data) as u32);
    bpf_ksock_release(ks);

    return ret;
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
