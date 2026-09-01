// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies: "vmlinux.h" and <bpf/bpf_helpers.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

#[repr(C)]
pub struct priv_map {
    // __uint(type, BPF_MAP_TYPE_QUEUE);
    // __uint(max_entries, 1);
    // __type(value, __u32);
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut priv_map: priv_map = priv_map {};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
