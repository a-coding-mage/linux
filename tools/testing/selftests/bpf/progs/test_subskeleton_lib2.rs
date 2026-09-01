// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) Meta Platforms, Inc. and affiliates. */

// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

pub static mut var6: i32 = 6;

#[repr(C)]
pub struct Map2 {
    // __uint(type, BPF_MAP_TYPE_HASH);
    pub type_: u32,
    // __type(key, __u32);
    pub key: u32,
    // __type(value, __u32);
    pub value: u32,
    // __uint(max_entries, 16);
    pub max_entries: u32,
}

// SEC(".maps")
#[link_section = ".maps"]
pub static mut map2: Map2 = Map2 {
    type_: BPF_MAP_TYPE_HASH,
    key: 0,
    value: 0,
    max_entries: 16,
};

// BPF_MAP_TYPE_HASH is provided by <linux/bpf.h> in the original source.
extern "C" {
    pub static BPF_MAP_TYPE_HASH: u32;
}

// SEC("license")
#[link_section = "license"]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
