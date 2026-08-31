// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// <vmlinux.h>, "xdp_metadata.h", <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

extern "C" {
    fn bpf_xdp_metadata_rx_hash(
        ctx: *const xdp_md,
        hash: *mut __u32,
        rss_type: *mut xdp_rss_hash_type,
    ) -> ::core::ffi::c_int;
}

#[no_mangle]
pub static mut called: ::core::ffi::c_int = 0;

#[no_mangle]
#[link_section = "freplace/rx"]
pub unsafe extern "C" fn freplace_rx(ctx: *mut xdp_md) -> ::core::ffi::c_int {
    let mut type_: xdp_rss_hash_type = 0;
    let mut hash: u32 = 0;

    /* Call _any_ metadata function to make sure we don't crash. */
    bpf_xdp_metadata_rx_hash(
        ctx as *const xdp_md,
        &mut hash as *mut u32 as *mut __u32,
        &mut type_ as *mut xdp_rss_hash_type,
    );
    called += 1;
    XDP_PASS
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];
