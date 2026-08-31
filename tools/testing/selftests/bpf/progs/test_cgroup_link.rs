// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[no_mangle]
pub static mut calls: i32 = 0;

#[no_mangle]
pub static mut alt_calls: i32 = 0;

#[no_mangle]
#[link_section = "cgroup_skb/egress"]
pub unsafe extern "C" fn egress(_skb: *mut __sk_buff) -> i32 {
    unsafe {
        core::intrinsics::atomic_xadd_seqcst(&raw mut calls, 1);
    }
    1
}

#[no_mangle]
#[link_section = "cgroup_skb/egress"]
pub unsafe extern "C" fn egress_alt(_skb: *mut __sk_buff) -> i32 {
    unsafe {
        core::intrinsics::atomic_xadd_seqcst(&raw mut alt_calls, 1);
    }
    1
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
