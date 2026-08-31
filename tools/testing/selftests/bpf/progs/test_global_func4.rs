// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Facebook */
// C dependencies: <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
}

#[inline(never)]
pub unsafe extern "C" fn f1(skb: *mut __sk_buff) -> i32 {
    unsafe { (*skb).len as i32 }
}

#[inline(never)]
pub unsafe extern "C" fn f2(val: i32, skb: *mut __sk_buff) -> i32 {
    unsafe { f1(skb) + val }
}

#[inline(never)]
pub unsafe extern "C" fn f3(val: i32, skb: *mut __sk_buff, var: i32) -> i32 {
    unsafe { f2(var, skb) + val }
}

#[inline(never)]
pub unsafe extern "C" fn f4(skb: *mut __sk_buff) -> i32 {
    unsafe { f3(1, skb, 2) }
}

#[inline(never)]
pub unsafe extern "C" fn f5(skb: *mut __sk_buff) -> i32 {
    unsafe { f4(skb) }
}

#[inline(never)]
pub unsafe extern "C" fn f6(skb: *mut __sk_buff) -> i32 {
    unsafe { f5(skb) }
}

#[inline(never)]
pub unsafe extern "C" fn f7(skb: *mut __sk_buff) -> i32 {
    unsafe { f6(skb) }
}

// SEC("tc")
// __success
#[no_mangle]
pub unsafe extern "C" fn global_func4(skb: *mut __sk_buff) -> i32 {
    unsafe { f7(skb) }
}
