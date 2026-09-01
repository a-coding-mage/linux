// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Facebook */
// C dependencies: <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub ifindex: u32,
}

#[inline(never)]
pub unsafe extern "C" fn f1(skb: *mut __sk_buff) -> i32 {
    unsafe { (*skb).len as i32 }
}

// C forward declaration: int f3(int, struct __sk_buff *skb);

#[inline(never)]
pub unsafe extern "C" fn f2(val: i32, skb: *mut __sk_buff) -> i32 {
    unsafe { f1(skb) + f3(val, skb.add(1)) } /* type mismatch */
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn f3(val: i32, skb: *mut __sk_buff) -> i32 {
    unsafe { ((*skb).ifindex as i32).wrapping_mul(val) }
}

// SEC("tc")
// __failure __msg("modified ctx ptr R2")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_func6(skb: *mut __sk_buff) -> i32 {
    unsafe { f1(skb) + f2(2, skb) + f3(3, skb) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
