// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

// Original map declaration used bpf_helpers macros:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, __u32);
//     __type(value, __u64);
//     __uint(pinning, 2); /* invalid */
// } nopinmap3 SEC(".maps");
#[repr(C)]
pub struct Nopinmap3 {
    pub type_: u32,
    pub max_entries: u32,
    pub key: u32,
    pub value: u64,
    pub pinning: u32, /* invalid */
}

#[link_section = ".maps"]
#[no_mangle]
pub static nopinmap3: Nopinmap3 = Nopinmap3 {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key: 0,
    value: 0,
    pinning: 2,
};

extern "C" {
    pub static BPF_MAP_TYPE_ARRAY: u32;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
