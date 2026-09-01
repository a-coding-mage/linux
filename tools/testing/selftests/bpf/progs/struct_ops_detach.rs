// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, "../test_kmods/bpf_testmod.h"

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

/*
 * This subprogram validates that libbpf handles the situation in which BPF
 * object has subprograms in .text section, but has no entry BPF programs.
 * At some point that was causing issues due to legacy logic of treating such
 * subprogram as entry program (with unknown program type, which would fail).
 */
#[no_mangle]
pub extern "C" fn dangling_subprog() -> i32 {
    /* do nothing, just be here */
    0
}

extern "C" {
    pub type bpf_testmod_ops;
}

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut testmod_do_detach: bpf_testmod_ops;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
