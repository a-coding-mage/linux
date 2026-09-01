// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;
pub type __u64 = u64;

extern "C" {
    pub type bpf_spin_lock;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct htab_val {
    pub lock: bpf_spin_lock,
    pub data: u32,
}

// Original C BPF map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 64);
//     __type(key, unsigned int);
//     __type(value, struct htab_val);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
// } htab SEC(".maps");
#[repr(C)]
pub struct htab_map_def {}

#[no_mangle]
#[link_section = ".maps"]
pub static mut htab: htab_map_def = htab_map_def {};

pub const HTAB_NDATA: usize = 256;

#[repr(C)]
pub struct htab_val_large {
    pub lock: bpf_spin_lock,
    pub seq: __u32,
    pub data: [__u64; HTAB_NDATA],
}

// Original C BPF map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 8);
//     __type(key, unsigned int);
//     __type(value, struct htab_val_large);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
// } htab_lock_consistency SEC(".maps");
#[repr(C)]
pub struct htab_lock_consistency_map_def {}

#[no_mangle]
#[link_section = ".maps"]
pub static mut htab_lock_consistency: htab_lock_consistency_map_def =
    htab_lock_consistency_map_def {};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
