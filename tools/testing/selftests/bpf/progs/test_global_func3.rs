// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Facebook */
// C includes translated as external dependency intent:
// <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
}

#[inline(never)]
unsafe fn f1(skb: *mut __sk_buff) -> i32 {
    unsafe { (*skb).len as i32 }
}

#[inline(never)]
unsafe fn f2(val: i32, skb: *mut __sk_buff) -> i32 {
    unsafe { f1(skb) + val }
}

#[inline(never)]
unsafe fn f3(val: i32, skb: *mut __sk_buff, var: i32) -> i32 {
    unsafe { f2(var, skb) + val }
}

#[inline(never)]
unsafe fn f4(skb: *mut __sk_buff) -> i32 {
    unsafe { f3(1, skb, 2) }
}

#[inline(never)]
unsafe fn f5(skb: *mut __sk_buff) -> i32 {
    unsafe { f4(skb) }
}

#[inline(never)]
unsafe fn f6(skb: *mut __sk_buff) -> i32 {
    unsafe { f5(skb) }
}

#[inline(never)]
unsafe fn f7(skb: *mut __sk_buff) -> i32 {
    unsafe { f6(skb) }
}

#[inline(never)]
unsafe fn f8(skb: *mut __sk_buff) -> i32 {
    unsafe { f7(skb) }
}

#[inline(never)]
unsafe fn f9(skb: *mut __sk_buff) -> i32 {
    unsafe { f8(skb) }
}

#[inline(never)]
unsafe fn f10(skb: *mut __sk_buff) -> i32 {
    unsafe { f9(skb) }
}

#[inline(never)]
unsafe fn f11(skb: *mut __sk_buff) -> i32 {
    unsafe { f10(skb) }
}

#[inline(never)]
unsafe fn f12(skb: *mut __sk_buff) -> i32 {
    unsafe { f11(skb) }
}

#[inline(never)]
unsafe fn f13(skb: *mut __sk_buff) -> i32 {
    unsafe { f12(skb) }
}

#[inline(never)]
unsafe fn f14(skb: *mut __sk_buff) -> i32 {
    unsafe { f13(skb) }
}

#[inline(never)]
unsafe fn f15(skb: *mut __sk_buff) -> i32 {
    unsafe { f14(skb) }
}

#[inline(never)]
unsafe fn f16(skb: *mut __sk_buff) -> i32 {
    unsafe { f15(skb) }
}

// SEC("tc")
// __failure __msg("the call stack of 17 frames")
#[no_mangle]
pub unsafe extern "C" fn global_func3(skb: *mut __sk_buff) -> i32 {
    unsafe { f16(skb) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
