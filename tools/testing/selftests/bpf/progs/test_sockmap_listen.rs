// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Cloudflare

// C dependencies removed from executable Rust:
// <errno.h>, <stdbool.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_SOCKMAP: u32 = 15;
pub const BPF_MAP_TYPE_SOCKHASH: u32 = 18;
pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_F_INGRESS: u64 = 1;
pub const SK_DROP: i32 = 0;
pub const SK_PASS: i32 = 1;

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
}

#[repr(C)]
pub struct sk_msg_md {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_reuseport_md {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static sock_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 2,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static nop_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 2,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static sock_hash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKHASH,
    max_entries: 2,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static verdict_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 2,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<u32>() as u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static parser_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

#[no_mangle]
pub static mut test_sockmap: bool = false; /* toggled by user-space */
#[no_mangle]
pub static mut test_ingress: bool = false; /* toggled by user-space */

extern "C" {
    fn bpf_map_lookup_elem(map: *const bpf_map_def, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_sk_redirect_map(
        skb: *mut __sk_buff,
        map: *const bpf_map_def,
        key: __u32,
        flags: __u64,
    ) -> i32;
    fn bpf_sk_redirect_hash(
        skb: *mut __sk_buff,
        map: *const bpf_map_def,
        key: *const core::ffi::c_void,
        flags: __u64,
    ) -> i32;
    fn bpf_msg_redirect_map(
        msg: *mut sk_msg_md,
        map: *const bpf_map_def,
        key: __u32,
        flags: __u64,
    ) -> i32;
    fn bpf_msg_redirect_hash(
        msg: *mut sk_msg_md,
        map: *const bpf_map_def,
        key: *const core::ffi::c_void,
        flags: __u64,
    ) -> i32;
    fn bpf_sk_select_reuseport(
        reuse: *mut sk_reuseport_md,
        map: *const bpf_map_def,
        key: *const core::ffi::c_void,
        flags: __u64,
    ) -> i32;
}

#[link_section = "sk_skb/stream_parser"]
#[no_mangle]
pub unsafe extern "C" fn prog_stream_parser(skb: *mut __sk_buff) -> i32 {
    let mut value: *mut i32;
    let key: __u32 = 0;

    value = bpf_map_lookup_elem(
        &parser_map as *const bpf_map_def,
        &key as *const __u32 as *const core::ffi::c_void,
    ) as *mut i32;
    if !value.is_null() && *value != 0 {
        return *value;
    }

    (*skb).len as i32
}

#[link_section = "sk_skb/stream_verdict"]
#[no_mangle]
pub unsafe extern "C" fn prog_stream_verdict(skb: *mut __sk_buff) -> i32 {
    let mut count: *mut u32;
    let zero: __u32 = 0;
    let verdict: i32;

    if test_sockmap {
        verdict = bpf_sk_redirect_map(skb, &sock_map as *const bpf_map_def, zero, 0);
    } else {
        verdict = bpf_sk_redirect_hash(
            skb,
            &sock_hash as *const bpf_map_def,
            &zero as *const __u32 as *const core::ffi::c_void,
            0,
        );
    }

    count = bpf_map_lookup_elem(
        &verdict_map as *const bpf_map_def,
        &verdict as *const i32 as *const core::ffi::c_void,
    ) as *mut u32;
    if !count.is_null() {
        *count = (*count).wrapping_add(1);
    }

    verdict
}

#[link_section = "sk_skb"]
#[no_mangle]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> i32 {
    let mut count: *mut u32;
    let zero: __u32 = 0;
    let verdict: i32;

    if test_sockmap {
        verdict = bpf_sk_redirect_map(
            skb,
            &sock_map as *const bpf_map_def,
            zero,
            if test_ingress { BPF_F_INGRESS } else { 0 },
        );
    } else {
        verdict = bpf_sk_redirect_hash(
            skb,
            &sock_hash as *const bpf_map_def,
            &zero as *const __u32 as *const core::ffi::c_void,
            if test_ingress { BPF_F_INGRESS } else { 0 },
        );
    }

    count = bpf_map_lookup_elem(
        &verdict_map as *const bpf_map_def,
        &verdict as *const i32 as *const core::ffi::c_void,
    ) as *mut u32;
    if !count.is_null() {
        *count = (*count).wrapping_add(1);
    }

    verdict
}

#[link_section = "sk_msg"]
#[no_mangle]
pub unsafe extern "C" fn prog_msg_verdict(msg: *mut sk_msg_md) -> i32 {
    let mut count: *mut u32;
    let zero: __u32 = 0;
    let verdict: i32;

    if test_sockmap {
        verdict = bpf_msg_redirect_map(msg, &sock_map as *const bpf_map_def, zero, 0);
    } else {
        verdict = bpf_msg_redirect_hash(
            msg,
            &sock_hash as *const bpf_map_def,
            &zero as *const __u32 as *const core::ffi::c_void,
            0,
        );
    }

    count = bpf_map_lookup_elem(
        &verdict_map as *const bpf_map_def,
        &verdict as *const i32 as *const core::ffi::c_void,
    ) as *mut u32;
    if !count.is_null() {
        *count = (*count).wrapping_add(1);
    }

    verdict
}

#[link_section = "sk_reuseport"]
#[no_mangle]
pub unsafe extern "C" fn prog_reuseport(reuse: *mut sk_reuseport_md) -> i32 {
    let mut count: *mut u32;
    let err: i32;
    let verdict: i32;
    let zero: __u32 = 0;

    if test_sockmap {
        err = bpf_sk_select_reuseport(
            reuse,
            &sock_map as *const bpf_map_def,
            &zero as *const __u32 as *const core::ffi::c_void,
            0,
        );
    } else {
        err = bpf_sk_select_reuseport(
            reuse,
            &sock_hash as *const bpf_map_def,
            &zero as *const __u32 as *const core::ffi::c_void,
            0,
        );
    }
    verdict = if err != 0 { SK_DROP } else { SK_PASS };

    count = bpf_map_lookup_elem(
        &verdict_map as *const bpf_map_def,
        &verdict as *const i32 as *const core::ffi::c_void,
    ) as *mut u32;
    if !count.is_null() {
        *count = (*count).wrapping_add(1);
    }

    verdict
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
