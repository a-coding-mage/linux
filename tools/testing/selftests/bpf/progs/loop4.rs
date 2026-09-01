// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_compiler.h"

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn combinations(skb: *mut __sk_buff) -> i32 {
    let mut ret: i32 = 0;
    let mut i: i32;

    // __pragma_loop_no_unroll
    i = 0;
    while i < 20 {
        if unsafe { core::ptr::addr_of!((*skb).len).read_volatile() } != 0 {
            ret |= 1 << i;
        }
        i += 1;
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
