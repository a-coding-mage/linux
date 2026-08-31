// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// Translated from:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;

const BPF_MAP_TYPE_HASH: u32 = 1;

#[repr(C)]
pub struct value_t {
    pub a: [i64; 32],
}

#[repr(C)]
pub struct map_hash_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut map_hash: map_hash_def = map_hash_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<i64>() as u32,
    value_size: core::mem::size_of::<value_t>() as u32,
};

extern "C" {
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_map_lookup_elem(map: *mut map_hash_def, key: *const i64) -> *mut value_t;
}

#[link_section = "socket"]
#[no_mangle]
// __description("bpf_jit_convergence je <-> jmp")
// __success __retval(0)
// __arch_x86_64
// __jited("	pushq	%rbp")
pub unsafe extern "C" fn btf_jit_convergence_je_jmp() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "if r0 == 0 goto l20_0;",
        "if r0 == 1 goto l21_0;",
        "if r0 == 2 goto l22_0;",
        "if r0 == 3 goto l23_0;",
        "if r0 == 4 goto l24_0;",
        "call {bpf_get_prandom_u32};",
        "call {bpf_get_prandom_u32};",
        "l20_0:",
        "l21_0:",
        "l22_0:",
        "l23_0:",
        "l24_0:",
        "r1 = 0;",
        "*(u64 *)(r10 - 8) = r1;",
        "r2 = r10;",
        "r2 += -8;",
        "r1 = {map_hash} ll;",
        "call {bpf_map_lookup_elem};",
        "if r0 == 0 goto l1_0;",
        "r6 = r0;",
        "call {bpf_get_prandom_u32};",
        "r7 = r0;",
        "r5 = r6;",
        "if r0 != 0x0 goto l12_0;",
        "call {bpf_get_prandom_u32};",
        "r1 = r0;",
        "r2 = r6;",
        "if r1 == 0x0 goto l0_0;",
        "l9_0:",
        "r2 = *(u64 *)(r6 + 0x0);",
        "r2 += 0x1;",
        "*(u64 *)(r6 + 0x0) = r2;",
        "goto l1_0;",
        "l12_0:",
        "r1 = r7;",
        "r1 += 0x98;",
        "r2 = r5;",
        "r2 += 0x90;",
        "r2 = *(u32 *)(r2 + 0x0);",
        "r3 = r7;",
        "r3 &= 0x1;",
        "r2 *= 0xa8;",
        "if r3 == 0x0 goto l2_0;",
        "r1 += r2;",
        "r1 -= r7;",
        "r1 += 0x8;",
        "if r1 <= 0xb20 goto l3_0;",
        "r1 = 0x0;",
        "goto l4_0;",
        "l3_0:",
        "r1 += r7;",
        "l4_0:",
        "if r1 == 0x0 goto l8_0;",
        "goto l9_0;",
        "l2_0:",
        "r1 += r2;",
        "r1 -= r7;",
        "r1 += 0x10;",
        "if r1 <= 0xb20 goto l6_0;",
        "r1 = 0x0;",
        "goto l7_0;",
        "l6_0:",
        "r1 += r7;",
        "l7_0:",
        "if r1 == 0x0 goto l8_0;",
        "goto l9_0;",
        "l0_0:",
        "r1 = 0x3;",
        "*(u64 *)(r10 - 0x10) = r1;",
        "r2 = r1;",
        "goto l1_0;",
        "l8_0:",
        "r1 = r5;",
        "r1 += 0x4;",
        "r1 = *(u32 *)(r1 + 0x0);",
        "*(u64 *)(r10 - 0x8) = r1;",
        "l1_0:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash = sym map_hash,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
