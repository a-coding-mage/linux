// SPDX-License-Identifier: GPL-2.0
// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;

const BPF_MAP_TYPE_SK_STORAGE: u32 = 24;
const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_SK_STORAGE_GET_F_CREATE: u64 = 1;
const BPF_SOCK_OPS_TCP_CONNECT_CB: i32 = 2;
const BPF_SOCK_OPS_RTT_CB: i32 = 12;
const BPF_SOCK_OPS_RTT_CB_FLAG: i32 = 1 << 3;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct tcp_rtt_storage {
    pub invoked: __u32,
    pub dsack_dups: __u32,
    pub delivered: __u32,
    pub delivered_ce: __u32,
    pub icsk_retransmits: __u32,

    pub mrtt_us: __u32, /* args[0] */
    pub srtt: __u32,    /* args[1] */
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_tcp_sock {
    _reserved0: [u8; 0],
    pub dsack_dups: __u32,
    pub delivered: __u32,
    pub delivered_ce: __u32,
    pub icsk_retransmits: __u32,
}

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: __u32,
    pub args: [__u32; 4],
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct socket_storage_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut socket_storage_map: socket_storage_map_def = socket_storage_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<tcp_rtt_storage>() as u32,
};

extern "C" {
    fn bpf_sk_storage_get(
        map: *mut socket_storage_map_def,
        sk: *mut bpf_sock,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut tcp_rtt_storage;
    fn bpf_sock_ops_cb_flags_set(ctx: *mut bpf_sock_ops, flags: i32) -> i32;
    fn bpf_tcp_sock(sk: *mut bpf_sock) -> *mut bpf_tcp_sock;
}

#[no_mangle]
#[link_section = "sockops"]
pub unsafe extern "C" fn _sockops(ctx: *mut bpf_sock_ops) -> i32 {
    let storage: *mut tcp_rtt_storage;
    let tcp_sk: *mut bpf_tcp_sock;
    let op: i32 = (*ctx).op as i32;
    let sk: *mut bpf_sock;

    sk = (*ctx).sk;
    if sk.is_null() {
        return 1;
    }

    storage = bpf_sk_storage_get(
        &mut socket_storage_map,
        sk,
        core::ptr::null_mut(),
        BPF_SK_STORAGE_GET_F_CREATE,
    );
    if storage.is_null() {
        return 1;
    }

    if op == BPF_SOCK_OPS_TCP_CONNECT_CB {
        bpf_sock_ops_cb_flags_set(ctx, BPF_SOCK_OPS_RTT_CB_FLAG);
        return 1;
    }

    if op != BPF_SOCK_OPS_RTT_CB {
        return 1;
    }

    tcp_sk = bpf_tcp_sock(sk);
    if tcp_sk.is_null() {
        return 1;
    }

    (*storage).invoked = (*storage).invoked.wrapping_add(1);

    (*storage).dsack_dups = (*tcp_sk).dsack_dups;
    (*storage).delivered = (*tcp_sk).delivered;
    (*storage).delivered_ce = (*tcp_sk).delivered_ce;
    (*storage).icsk_retransmits = (*tcp_sk).icsk_retransmits;

    (*storage).mrtt_us = (*ctx).args[0];
    (*storage).srtt = (*ctx).args[1];

    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
