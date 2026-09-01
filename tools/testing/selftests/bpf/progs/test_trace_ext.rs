// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// C dependencies: linux/bpf.h, stdbool.h, bpf/bpf_helpers.h,
// bpf/bpf_endian.h, bpf/bpf_tracing.h

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
}

#[no_mangle]
pub static mut ext_called: u64 = 0;

#[no_mangle]
#[link_section = "freplace/test_pkt_md_access"]
pub unsafe extern "C" fn test_pkt_md_access_new(skb: *mut __sk_buff) -> i32 {
    ext_called = (*skb).len as u64;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
