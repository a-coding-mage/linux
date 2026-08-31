// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020, Tessares SA. */
/* Copyright (c) 2022, SUSE. */

/* Rust translation of includes:
 * #include "bpf_tracing_net.h"
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 */

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcp_sock {
    pub is_mptcp: bool,
}

#[repr(C)]
pub struct mptcp_sock {
    pub token: __u32,
    pub ca_name: [::core::ffi::c_char; TCP_CA_NAME_MAX],
    pub first: *mut sock,
}

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: __u32,
    pub sk: *mut bpf_sock,
}

pub type __u32 = u32;

extern "C" {
    fn bpf_skc_to_tcp_sock(sk: *mut bpf_sock) -> *mut tcp_sock;
    fn bpf_skc_to_mptcp_sock(sk: *mut bpf_sock) -> *mut mptcp_sock;
    fn bpf_sk_storage_get(
        map: *mut socket_storage_map_type,
        sk: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        flags: __u64,
    ) -> *mut mptcp_storage;
}

pub type __u64 = u64;

extern "C" {
    static BPF_SOCK_OPS_TCP_CONNECT_CB: i32;
    static BPF_MAP_TYPE_SK_STORAGE: u32;
    static BPF_F_NO_PREALLOC: u32;
    static BPF_SK_STORAGE_GET_F_CREATE: __u64;
    static TCP_CA_NAME_MAX: usize;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

#[no_mangle]
pub static mut token: __u32 = 0;

#[repr(C)]
pub struct mptcp_storage {
    pub invoked: __u32,
    pub is_mptcp: __u32,
    pub sk: *mut sock,
    pub token: __u32,
    pub first: *mut sock,
    pub ca_name: [::core::ffi::c_char; TCP_CA_NAME_MAX],
}

#[repr(C)]
pub struct socket_storage_map_type {
    _private: [u8; 0],
}

/* BPF map definition translated from:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
 *     __uint(map_flags, BPF_F_NO_PREALLOC);
 *     __type(key, int);
 *     __type(value, struct mptcp_storage);
 * } socket_storage_map SEC(".maps");
 */
#[no_mangle]
#[link_section = ".maps"]
pub static mut socket_storage_map: socket_storage_map_type = socket_storage_map_type { _private: [] };

#[no_mangle]
#[link_section = "sockops"]
pub unsafe extern "C" fn _sockops(ctx: *mut bpf_sock_ops) -> i32 {
    let mut storage: *mut mptcp_storage;
    let mut msk: *mut mptcp_sock;
    let op: i32 = (*ctx).op as i32;
    let mut tsk: *mut tcp_sock;
    let mut sk: *mut bpf_sock;
    let is_mptcp: bool;

    if op != BPF_SOCK_OPS_TCP_CONNECT_CB {
        return 1;
    }

    sk = (*ctx).sk;
    if sk.is_null() {
        return 1;
    }

    tsk = bpf_skc_to_tcp_sock(sk);
    if tsk.is_null() {
        return 1;
    }

    /* bpf_core_field_exists(tsk->is_mptcp) is a CO-RE build-time field test.
     * When the field exists, use tsk->is_mptcp; otherwise use 0.
     */
    is_mptcp = (*tsk).is_mptcp;
    if !is_mptcp {
        storage = bpf_sk_storage_get(
            &mut socket_storage_map,
            sk as *mut ::core::ffi::c_void,
            ::core::ptr::null_mut(),
            BPF_SK_STORAGE_GET_F_CREATE,
        );
        if storage.is_null() {
            return 1;
        }

        (*storage).token = 0;
        ::core::ptr::write_bytes((*storage).ca_name.as_mut_ptr(), 0, TCP_CA_NAME_MAX);
        (*storage).first = ::core::ptr::null_mut();
    } else {
        msk = bpf_skc_to_mptcp_sock(sk);
        if msk.is_null() {
            return 1;
        }

        storage = bpf_sk_storage_get(
            &mut socket_storage_map,
            msk as *mut ::core::ffi::c_void,
            ::core::ptr::null_mut(),
            BPF_SK_STORAGE_GET_F_CREATE,
        );
        if storage.is_null() {
            return 1;
        }

        (*storage).token = (*msk).token;
        ::core::ptr::copy_nonoverlapping(
            (*msk).ca_name.as_ptr(),
            (*storage).ca_name.as_mut_ptr(),
            TCP_CA_NAME_MAX,
        );
        (*storage).first = (*msk).first;
    }
    (*storage).invoked = (*storage).invoked.wrapping_add(1);
    (*storage).is_mptcp = is_mptcp as __u32;
    (*storage).sk = sk as *mut sock;

    1
}

#[no_mangle]
#[link_section = "fentry/mptcp_pm_new_connection"]
pub unsafe extern "C" fn trace_mptcp_pm_new_connection(
    msk: *mut mptcp_sock,
    _ssk: *const sock,
    server_side: i32,
) -> i32 {
    if server_side == 0 {
        token = (*msk).token;
    }

    0
}
