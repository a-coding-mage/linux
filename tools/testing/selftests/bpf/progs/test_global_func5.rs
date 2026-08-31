// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Facebook */
// Dependencies from C source:
// #include <stddef.h>
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    len: u32,
    pkt_type: u32,
    mark: u32,
    queue_mapping: u32,
    protocol: u32,
    vlan_present: u32,
    vlan_tci: u32,
    vlan_proto: u32,
    priority: u32,
    ingress_ifindex: u32,
    ifindex: u32,
}

#[inline(never)]
pub unsafe extern "C" fn f1(skb: *mut __sk_buff) -> i32 {
    (*skb).len as i32
}

// int f3(int, struct __sk_buff *skb);

#[inline(never)]
pub unsafe extern "C" fn f2(val: i32, skb: *mut __sk_buff) -> i32 {
    f1(skb) + f3(val, (&val as *const i32 as *mut core::ffi::c_void) as *mut __sk_buff) /* type mismatch */
}

#[inline(never)]
pub unsafe extern "C" fn f3(val: i32, skb: *mut __sk_buff) -> i32 {
    ((*skb).ifindex as i32) * val
}

// SEC("tc")
// __failure __msg("expects pointer to ctx")
pub unsafe extern "C" fn global_func5(skb: *mut __sk_buff) -> i32 {
    f1(skb) + f2(2, skb) + f3(3, skb)
}
