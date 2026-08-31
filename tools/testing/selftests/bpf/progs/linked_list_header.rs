// SPDX-License-Identifier: GPL-2.0
// Original C header guard LINKED_LIST_H omitted in Rust.
// Dependencies from the C header:
// - <vmlinux.h>
// - <bpf/bpf_helpers.h>
// - "bpf_experimental.h"

#[repr(C)]
pub struct bar {
    pub node: bpf_list_node,
    pub data: ::core::ffi::c_int,
}

#[repr(C)]
pub struct foo {
    pub node: bpf_list_node,
    // C annotation: __contains(bar, node)
    pub head: bpf_list_head,
    pub lock: bpf_spin_lock,
    pub data: ::core::ffi::c_int,
    pub node2: bpf_list_node,
}

#[repr(C)]
pub struct map_value {
    pub lock: bpf_spin_lock,
    pub data: ::core::ffi::c_int,
    // C annotation: __contains(foo, node2)
    pub head: bpf_list_head,
}

#[repr(C)]
pub struct array_map {
    // C BPF map metadata:
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __type(key, int);
    // __type(value, struct map_value);
    // __uint(max_entries, 1);
    pub _private: [u8; 0],
}

extern "C" {
    // C declaration: struct array_map array_map SEC(".maps");
    #[link_section = ".maps"]
    pub static mut array_map: array_map;

    // C declaration: struct array_map inner_map SEC(".maps");
    #[link_section = ".maps"]
    pub static mut inner_map: array_map;
}

#[repr(C)]
pub struct map_of_maps {
    // C BPF map metadata:
    // __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    // __uint(max_entries, 1);
    // __type(key, int);
    // __type(value, int);
    // __array(values, struct array_map);
    pub values: [*mut array_map; 1],
}

// C definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, int);
//     __array(values, struct array_map);
// } map_of_maps SEC(".maps") = {
//     .values = {
//         [0] = &inner_map,
//     },
// };
#[link_section = ".maps"]
pub static mut map_of_maps: map_of_maps = map_of_maps {
    values: [unsafe { &mut inner_map as *mut array_map }],
};

// C macro:
// #define private(name) SEC(".bss." #name) __hidden __attribute__((aligned(8)))

// C declaration: private(A) struct bpf_spin_lock glock;
#[link_section = ".bss.A"]
#[no_mangle]
pub static mut glock: bpf_spin_lock = unsafe { ::core::mem::zeroed() };

// C declaration: private(A) struct bpf_list_head ghead __contains(foo, node2);
// C annotation: __contains(foo, node2)
#[link_section = ".bss.A"]
#[no_mangle]
pub static mut ghead: bpf_list_head = unsafe { ::core::mem::zeroed() };

// C declaration: private(B) struct bpf_spin_lock glock2;
#[link_section = ".bss.B"]
#[no_mangle]
pub static mut glock2: bpf_spin_lock = unsafe { ::core::mem::zeroed() };
