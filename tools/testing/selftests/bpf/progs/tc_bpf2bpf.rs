// SPDX-License-Identifier: GPL-2.0

// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

extern "C" {
    fn __sink<T>(arg: T);
    fn bpf_skb_change_proto(skb: *mut __sk_buff, proto: u32, flags: u64) -> i64;
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[inline(never)]
pub unsafe extern "C" fn subprog_tc(skb: *mut __sk_buff) -> i32 {
    let ret: i32 = 1;

    __sink(skb);
    __sink(ret);
    /* let verifier know that 'subprog_tc' can change pointers to skb->data */
    bpf_skb_change_proto(skb, 0, 0);
    return ret;
}

#[link_section = "tc"]
pub unsafe extern "C" fn entry_tc(skb: *mut __sk_buff) -> i32 {
    return subprog_tc(skb);
}

#[link_section = "license"]
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";
