// SPDX-License-Identifier: GPL-2.0

// KBUILD_MODNAME was defined as "xdp_dummy" in the C source.
// Original C dependencies: <linux/bpf.h> and <bpf/bpf_helpers.h>.

extern "C" {
    pub type xdp_md;
}

extern "C" {
    static XDP_PASS: ::core::ffi::c_int;
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_dummy_prog(ctx: *mut xdp_md) -> ::core::ffi::c_int {
    XDP_PASS
}

#[no_mangle]
#[link_section = "xdp.frags"]
pub unsafe extern "C" fn xdp_dummy_prog_frags(ctx: *mut xdp_md) -> ::core::ffi::c_int {
    XDP_PASS
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
