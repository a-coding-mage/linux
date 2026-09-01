// SPDX-License-Identifier: GPL-2.0
/* Copyright Leon Hwang */

// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut count: i32 = 0;

#[link_section = "fexit/subprog_tail"]
#[no_mangle]
pub unsafe extern "C" fn fexit(skb: *mut sk_buff) -> i32 {
    count += 1;

    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
