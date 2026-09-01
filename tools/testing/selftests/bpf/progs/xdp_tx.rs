// SPDX-License-Identifier: GPL-2.0

// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct xdp_md {
    _unused: [u8; 0],
}

extern "C" {
    static XDP_TX: ::core::ffi::c_int;
}

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn xdp_tx(xdp: *mut xdp_md) -> ::core::ffi::c_int {
    XDP_TX
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
