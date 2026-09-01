// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Isovalent */

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

pub static mut target_id: __u32 = 0;

extern "C" {
    #[link_name = "bpf_map_sum_elem_count"]
    fn bpf_map_sum_elem_count(map: *const bpf_map) -> __s64;
}

#[link_section = "iter/bpf_map"]
pub unsafe extern "C" fn dump_bpf_map(ctx: *mut bpf_iter__bpf_map) -> ::core::ffi::c_int {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let map: *mut bpf_map = (*ctx).map;

    if !map.is_null() && (*map).id == target_id {
        BPF_SEQ_PRINTF(seq, "%lld", bpf_map_sum_elem_count(map));
    }

    return 0;
}

#[link_section = "license"]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
