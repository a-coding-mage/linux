// SPDX-License-Identifier: GPL-2.0

// Dependency intent from C source:
// #include "bpf_tracing_net.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type __u32 = u32;

pub const BPF_MAP_TYPE_SOCKMAP: __u32 = 15;
pub const BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB: i32 = 4;
pub const BPF_NOEXIST: u64 = 1;

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: i32,
    pub local_port: __u32,
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

unsafe extern "C" {
    pub fn bpf_sock_map_update(
        skops: *mut bpf_sock_ops,
        map: *mut bpf_map_def,
        key: *mut i32,
        flags: u64,
    ) -> i32;

    pub fn bpf_sk_redirect_map(
        skb: *mut __sk_buff,
        map: *mut bpf_map_def,
        key: i32,
        flags: u64,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut sk_index: i32 = 0;
#[unsafe(no_mangle)]
pub static mut redirect_idx: i32 = 0;
#[unsafe(no_mangle)]
pub static mut trace_port: i32 = 0;
#[unsafe(no_mangle)]
pub static mut helper_ret: i32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut sock_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
    max_entries: 100,
};

#[unsafe(no_mangle)]
#[unsafe(link_section = "sockops")]
pub unsafe extern "C" fn mptcp_sockmap_inject(skops: *mut bpf_sock_ops) -> i32 {
    let sk: *mut bpf_sock;

    /* only accept specified connection */
    if (*skops).local_port != trace_port as __u32
        || (*skops).op != BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB
    {
        return 1;
    }

    sk = (*skops).sk;
    if sk.is_null() {
        return 1;
    }

    /* update sk handler */
    helper_ret = bpf_sock_map_update(
        skops,
        core::ptr::addr_of_mut!(sock_map),
        core::ptr::addr_of_mut!(sk_index),
        BPF_NOEXIST,
    );

    return 1;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "sk_skb/stream_verdict")]
pub unsafe extern "C" fn mptcp_sockmap_redirect(skb: *mut __sk_buff) -> i32 {
    /* redirect skb to the sk under sock_map[redirect_idx] */
    return bpf_sk_redirect_map(
        skb,
        core::ptr::addr_of_mut!(sock_map),
        redirect_idx,
        0,
    );
}
