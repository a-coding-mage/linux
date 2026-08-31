// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::ffi::c_void;

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_SOCKMAP: i32 = 15;
const BPF_MAP_TYPE_SOCKHASH: i32 = 18;
const BPF_MAP_TYPE_ARRAY: i32 = 2;
const __MAX_BPF_MAP_TYPE: i32 = 36;

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sk_msg_md {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def {
    type_: __u32,
    max_entries: __u32,
    key_size: __u32,
    value_size: __u32,
}

// SEC(".maps") struct { ... } nop_map, sock_map;
#[no_mangle]
#[link_section = ".maps"]
pub static mut nop_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP as __u32,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP as __u32,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

// SEC(".maps") struct { ... } nop_hash, sock_hash;
#[no_mangle]
#[link_section = ".maps"]
pub static mut nop_hash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKHASH as __u32,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_hash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKHASH as __u32,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

// SEC(".maps") struct { ... } verdict_map;
#[no_mangle]
#[link_section = ".maps"]
pub static mut verdict_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY as __u32,
    max_entries: 2,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<u32>() as __u32,
};

/* Set by user space */
#[no_mangle]
pub static mut redirect_type: i32 = 0;

#[no_mangle]
pub static mut redirect_flags: i32 = 0;

extern "C" {
    fn bpf_sk_redirect_map(
        skb: *mut __sk_buff,
        map: *mut bpf_map_def,
        key: __u32,
        flags: i32,
    ) -> i32;
    fn bpf_msg_redirect_map(
        msg: *mut sk_msg_md,
        map: *mut bpf_map_def,
        key: __u32,
        flags: i32,
    ) -> i32;
    fn bpf_sk_redirect_hash(
        skb: *mut __sk_buff,
        map: *mut bpf_map_def,
        key: *const __u32,
        flags: i32,
    ) -> i32;
    fn bpf_msg_redirect_hash(
        msg: *mut sk_msg_md,
        map: *mut bpf_map_def,
        key: *const __u32,
        flags: i32,
    ) -> i32;
    fn bpf_map_lookup_elem(map: *mut bpf_map_def, key: *const c_void) -> *mut c_void;
}

unsafe fn redirect_map_skb(data: *mut __sk_buff) -> i32 {
    bpf_sk_redirect_map(data, &raw mut sock_map, 0u32, redirect_flags)
}

unsafe fn redirect_map_msg(data: *mut sk_msg_md) -> i32 {
    bpf_msg_redirect_map(data, &raw mut sock_map, 0u32, redirect_flags)
}

unsafe fn redirect_hash_skb(data: *mut __sk_buff) -> i32 {
    let key: __u32 = 0;

    bpf_sk_redirect_hash(data, &raw mut sock_hash, &key, redirect_flags)
}

unsafe fn redirect_hash_msg(data: *mut sk_msg_md) -> i32 {
    let key: __u32 = 0;

    bpf_msg_redirect_hash(data, &raw mut sock_hash, &key, redirect_flags)
}

// DEFINE_PROG(skb, struct __sk_buff *);
#[no_mangle]
#[link_section = "sk_skb"]
pub unsafe extern "C" fn prog_skb_verdict(data: *mut __sk_buff) -> i32 {
    let count: *mut u32;
    let verdict: i32;

    if redirect_type == BPF_MAP_TYPE_SOCKMAP {
        verdict = redirect_map_skb(data);
    } else if redirect_type == BPF_MAP_TYPE_SOCKHASH {
        verdict = redirect_hash_skb(data);
    } else {
        verdict = redirect_type - __MAX_BPF_MAP_TYPE;
    }

    count = bpf_map_lookup_elem(
        &raw mut verdict_map,
        &verdict as *const i32 as *const c_void,
    ) as *mut u32;
    if !count.is_null() {
        *count = (*count).wrapping_add(1);
    }

    verdict
}

// DEFINE_PROG(msg, struct sk_msg_md *);
#[no_mangle]
#[link_section = "sk_msg"]
pub unsafe extern "C" fn prog_msg_verdict(data: *mut sk_msg_md) -> i32 {
    let count: *mut u32;
    let verdict: i32;

    if redirect_type == BPF_MAP_TYPE_SOCKMAP {
        verdict = redirect_map_msg(data);
    } else if redirect_type == BPF_MAP_TYPE_SOCKHASH {
        verdict = redirect_hash_msg(data);
    } else {
        verdict = redirect_type - __MAX_BPF_MAP_TYPE;
    }

    count = bpf_map_lookup_elem(
        &raw mut verdict_map,
        &verdict as *const i32 as *const c_void,
    ) as *mut u32;
    if !count.is_null() {
        *count = (*count).wrapping_add(1);
    }

    verdict
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
