// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2024. Huawei Technologies Co., Ltd */

// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#[repr(C)]
pub struct inner_map_type {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

// __uint(type, BPF_MAP_TYPE_ARRAY);
// __uint(key_size, 4);
// __uint(value_size, 4);
// __uint(max_entries, 1);
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static inner_map: inner_map_type = inner_map_type {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: 4,
    value_size: 4,
    max_entries: 1,
};

#[repr(C)]
pub struct outer_htab_map_type {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub values: *const inner_map_type,
}

// __uint(type, BPF_MAP_TYPE_HASH_OF_MAPS);
// __type(key, int);
// __type(value, int);
// __uint(max_entries, 2);
// __array(values, struct inner_map_type);
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static outer_htab_map: outer_htab_map_type = outer_htab_map_type {
    type_: BPF_MAP_TYPE_HASH_OF_MAPS,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 2,
    values: &inner_map as *const inner_map_type,
};

#[repr(C)]
pub struct outer_alloc_htab_map_type {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub values: *const inner_map_type,
}

// __uint(type, BPF_MAP_TYPE_HASH_OF_MAPS);
// __uint(map_flags, BPF_F_NO_PREALLOC);
// __type(key, int);
// __type(value, int);
// __uint(max_entries, 2);
// __array(values, struct inner_map_type);
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static outer_alloc_htab_map: outer_alloc_htab_map_type = outer_alloc_htab_map_type {
    type_: BPF_MAP_TYPE_HASH_OF_MAPS,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 2,
    values: &inner_map as *const inner_map_type,
};

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
