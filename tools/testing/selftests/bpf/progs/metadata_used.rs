// SPDX-License-Identifier: GPL-2.0-only

// Dependencies originally provided by:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

extern "C" {
    pub type xdp_md;
}

#[link_section = ".rodata"]
#[no_mangle]
pub static bpf_metadata_a: [core::ffi::c_char; 4] = [
    b'b' as core::ffi::c_char,
    b'a' as core::ffi::c_char,
    b'r' as core::ffi::c_char,
    0,
];

#[link_section = ".rodata"]
#[no_mangle]
pub static bpf_metadata_b: core::ffi::c_int = 2;

#[link_section = "cgroup_skb/egress"]
#[no_mangle]
pub unsafe extern "C" fn prog(ctx: *mut xdp_md) -> core::ffi::c_int {
    if core::ptr::read_volatile(&bpf_metadata_b) != 0 {
        1
    } else {
        0
    }
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];
