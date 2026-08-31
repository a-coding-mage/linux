// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies removed from executable Rust:
// <vmlinux.h>, "bpf_tracing_net.h", <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>.
// Types, constants, SEC/linkage conventions, and BPF helpers are expected to be
// supplied by the surrounding BPF Rust bindings/build environment.

use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    fn bpf_skc_to_tcp_sock(skc: *mut sock_common) -> *mut tcp_sock;
    fn bpf_getsockopt(
        sk: *mut tcp_sock,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: u32,
    ) -> c_int;
    fn bpf_strncmp(s1: *const c_char, s1_sz: u32, s2: *const c_char) -> c_int;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_setsockopt(
        sk: *mut tcp_sock,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: u32,
    ) -> c_int;
    fn bpf_ntohs(val: u16) -> u16;
}

const fn init_dctcp_cc() -> [c_char; TCP_CA_NAME_MAX as usize] {
    let src = *b"bpf_dctcp\0";
    let mut dst = [0 as c_char; TCP_CA_NAME_MAX as usize];
    let mut i = 0usize;

    while i < src.len() && i < TCP_CA_NAME_MAX as usize {
        dst[i] = src[i] as c_char;
        i += 1;
    }

    dst
}

#[no_mangle]
pub static mut reuse_listen_hport: u16 = 0;

#[no_mangle]
pub static mut listen_hport: u16 = 0;

#[no_mangle]
pub static cubic_cc: [c_char; 10] = [
    b'b' as c_char,
    b'p' as c_char,
    b'f' as c_char,
    b'_' as c_char,
    b'c' as c_char,
    b'u' as c_char,
    b'b' as c_char,
    b'i' as c_char,
    b'c' as c_char,
    0,
];

#[no_mangle]
pub static mut dctcp_cc: [c_char; TCP_CA_NAME_MAX as usize] = init_dctcp_cc();

#[no_mangle]
pub static mut random_retry: bool = false;

#[link_section = "iter/tcp"]
#[no_mangle]
pub unsafe extern "C" fn change_tcp_cc(ctx: *mut bpf_iter__tcp) -> c_int {
    let mut cur_cc = [0 as c_char; TCP_CA_NAME_MAX as usize];
    let mut tp: *mut tcp_sock;
    let mut sk: *mut sock;

    /*
     * C macro bpf_tcp_sk(ctx->sk_common):
     *   struct sock_common *_skc = skc;
     *   sk = NULL;
     *   tp = NULL;
     *   if (_skc) {
     *     tp = bpf_skc_to_tcp_sock(_skc);
     *     sk = (struct sock *)tp;
     *   }
     *   tp;
     */
    {
        let _skc: *mut sock_common = (*ctx).sk_common;
        sk = core::ptr::null_mut();
        tp = core::ptr::null_mut();
        if !_skc.is_null() {
            tp = bpf_skc_to_tcp_sock(_skc);
            sk = tp as *mut sock;
        }
        if tp.is_null() {
            return 0;
        }
    }

    if (*sk).sk_family as c_int != AF_INET6
        || ((*sk).sk_state as c_int != TCP_LISTEN
            && (*sk).sk_state as c_int != TCP_ESTABLISHED)
        || ((*sk).sk_num != reuse_listen_hport
            && (*sk).sk_num != listen_hport
            && bpf_ntohs((*sk).sk_dport) != listen_hport)
    {
        return 0;
    }

    if bpf_getsockopt(
        tp,
        SOL_TCP,
        TCP_CONGESTION,
        cur_cc.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&cur_cc) as u32,
    ) != 0
    {
        return 0;
    }

    if bpf_strncmp(
        cur_cc.as_ptr(),
        TCP_CA_NAME_MAX as u32,
        cubic_cc.as_ptr(),
    ) != 0
    {
        return 0;
    }

    if random_retry && bpf_get_prandom_u32() % 4 == 1 {
        return 1;
    }

    bpf_setsockopt(
        tp,
        SOL_TCP,
        TCP_CONGESTION,
        dctcp_cc.as_ptr() as *const c_void,
        core::mem::size_of_val(&dctcp_cc) as u32,
    );
    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];
