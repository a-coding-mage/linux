// SPDX-License-Identifier: GPL-2.0
/* Copyright Leon Hwang */

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[repr(C)]
pub struct sk_buff {
    _bindgen_opaque_blob: [u8; 0],
}

#[no_mangle]
pub static mut count: i32 = 0;

#[no_mangle]
#[link_section = "fentry/subprog_tail"]
pub unsafe extern "C" fn fentry(skb: *mut sk_buff) -> i32 {
    let _ = skb;

    count += 1;

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
