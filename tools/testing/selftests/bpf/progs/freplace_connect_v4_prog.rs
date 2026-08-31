// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies:
// linux/stddef.h, linux/ipv6.h, linux/bpf.h, linux/in.h, sys/socket.h,
// bpf/bpf_helpers.h, bpf/bpf_endian.h

#[link_section = "freplace/connect_v4_prog"]
#[no_mangle]
pub unsafe extern "C" fn new_connect_v4_prog(ctx: *mut bpf_sock_addr) -> ::core::ffi::c_int {
    let _ = ctx;

    // return value that's in invalid range
    255
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];
