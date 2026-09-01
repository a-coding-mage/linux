// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Isovalent, Inc. */
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct inner {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    pub type_: u32,
    // __type(key, __u32);
    pub key: u32,
    // __type(value, int);
    pub value: i32,
    // __uint(max_entries, 4);
    pub max_entries: u32,
}

#[repr(C)]
pub struct mim {
    // __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    pub type_: u32,
    // __uint(max_entries, 0); /* This will make map creation to fail */
    pub max_entries: u32,
    // __type(key, __u32);
    pub key: u32,
    // __array(values, struct inner);
    pub values: inner,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mim: mim = mim {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    max_entries: 0, /* This will make map creation to fail */
    key: 0,
    values: inner {
        type_: BPF_MAP_TYPE_ARRAY,
        key: 0,
        value: 0,
        max_entries: 4,
    },
};

#[repr(C)]
pub struct xdp_md {
    _unused: [u8; 0],
}

pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_MAP_TYPE_ARRAY_OF_MAPS: u32 = 12;
pub const XDP_PASS: i32 = 2;

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_noop0(ctx: *mut xdp_md) -> i32 {
    XDP_PASS
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
