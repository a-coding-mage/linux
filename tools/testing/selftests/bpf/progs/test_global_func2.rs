// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Facebook */
// C dependencies: <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

use core::ptr;

const MAX_STACK: usize = 512 - 3 * 32;

// Provided by the BPF/Linux bindings included by the original C source.
use crate::__sk_buff;

extern "C" {
    fn __sink(arg: i8);
}

#[inline(never)]
unsafe fn f0(_var: i32, skb: *mut __sk_buff) -> i32 {
    (*skb).len as i32
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn f1(skb: *mut __sk_buff) -> i32 {
    let buf: [i8; MAX_STACK] = [0; MAX_STACK];

    __sink(ptr::read_volatile(buf.as_ptr().add(MAX_STACK - 1)));

    f0(0, skb) + (*skb).len as i32
}

// C forward declaration: int f3(int, struct __sk_buff *skb, int);

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn f2(val: i32, skb: *mut __sk_buff) -> i32 {
    f1(skb) + f3(val, skb, 1)
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn f3(val: i32, skb: *mut __sk_buff, var: i32) -> i32 {
    let buf: [i8; MAX_STACK] = [0; MAX_STACK];

    __sink(ptr::read_volatile(buf.as_ptr().add(MAX_STACK - 1)));

    ((*skb).ifindex as i32) * val * var
}

// SEC("tc")
// __success
#[no_mangle]
pub unsafe extern "C" fn global_func2(skb: *mut __sk_buff) -> i32 {
    f0(1, skb) + f1(skb) + f2(2, skb) + f3(3, skb, 4)
}
