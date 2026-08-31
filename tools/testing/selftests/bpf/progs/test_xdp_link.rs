// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

#[no_mangle]
#[link_section = "license"]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_handler(xdp: *mut xdp_md) -> ::std::os::raw::c_int {
    return 0;
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_handler(skb: *mut __sk_buff) -> ::std::os::raw::c_int {
    return 0;
}
