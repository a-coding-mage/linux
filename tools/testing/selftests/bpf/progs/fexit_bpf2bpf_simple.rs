// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */
// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[repr(C)]
pub struct sk_buff {
    pub len: ::core::ffi::c_uint,
}

#[no_mangle]
pub static mut test_result: u64 = 0;

#[no_mangle]
#[link_section = "fexit/test_pkt_md_access"]
pub unsafe extern "C" fn test_main2(skb: *mut sk_buff, ret: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let len: ::core::ffi::c_int;

    // Original C used __builtin_preserve_access_index around this field access.
    len = (*skb).len as ::core::ffi::c_int;
    if len != 74 || ret != 0 {
        return 0;
    }

    test_result = 1;
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
