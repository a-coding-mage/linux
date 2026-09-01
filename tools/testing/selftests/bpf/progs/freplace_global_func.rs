// SPDX-License-Identifier: GPL-2.0
// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[inline(never)]
pub unsafe extern "C" fn test_ctx_global_func(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let retval: i32 = 1;
    unsafe { core::ptr::read_volatile(&retval) }
}

#[unsafe(link_section = "freplace/test_pkt_access")]
pub unsafe extern "C" fn new_test_pkt_access(skb: *mut __sk_buff) -> i32 {
    unsafe { test_ctx_global_func(skb) }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
