// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]

pub type __u32 = u32;

#[repr(C)]
pub struct __sk_buff {
    pub data_end: __u32,
}

#[no_mangle]
pub static mut data_end: __u32 = 0;

#[no_mangle]
#[link_section = "cgroup_skb/ingress"]
pub unsafe extern "C" fn direct_packet_access(skb: *mut __sk_buff) -> i32 {
    data_end = (*skb).data_end;
    1
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
