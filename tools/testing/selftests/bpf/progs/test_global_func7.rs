// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Facebook */
// C dependencies: <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    pub tc_index: u32,
}

#[inline(never)]
pub unsafe extern "C" fn foo(skb: *mut __sk_buff) {
    unsafe {
        (*skb).tc_index = 0;
    }
}

// SEC("tc")
// __success
#[no_mangle]
pub unsafe extern "C" fn global_func7(skb: *mut __sk_buff) -> i32 {
    unsafe {
        foo(skb);
    }
    0
}
