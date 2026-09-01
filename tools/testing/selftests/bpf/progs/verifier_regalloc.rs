// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/regalloc.c */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::global_asm;

// C includes translated as external dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

pub const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: u32,
    pub foo: [i32; MAX_ENTRIES],
}

// Original C BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 1);
//     __type(key, long long);
//     __type(value, struct test_val);
// } map_hash_48b SEC(".maps");
//
// The __uint/__type/SEC map-description macros are provided by BPF headers in C.
// Keep the exported symbol name for assembly references; map metadata remains a
// build-system/header dependency in this isolated translation.
#[no_mangle]
#[link_section = ".maps"]
pub static mut map_hash_48b: test_val = test_val {
    index: 0,
    foo: [0; MAX_ENTRIES],
};

extern "C" {
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
}

// SEC("tracepoint")
// __description("regalloc basic")
// __success __flag(BPF_F_ANY_ALIGNMENT)
// __naked void regalloc_basic(void)
global_asm!(
    r#"
    .section tracepoint,"ax"
    .global regalloc_basic
regalloc_basic:
    r6 = r1
    r1 = 0
    *(u64*)(r10 - 8) = r1
    r2 = r10
    r2 += -8
    r1 = map_hash_48b ll
    call bpf_map_lookup_elem
    if r0 == 0 goto 0f
    r7 = r0
    call bpf_get_prandom_u32
    r2 = r0
    if r0 s> 20 goto 0f
    if r2 s< 0 goto 0f
    r7 += r0
    r7 += r2
    r0 = *(u64*)(r7 + 0)
0:  exit
"#
);

// SEC("tracepoint")
// __description("regalloc negative")
// __failure __msg("invalid access to map value, value_size=48 off=48 size=1")
// __naked void regalloc_negative(void)
global_asm!(
    r#"
    .section tracepoint,"ax"
    .global regalloc_negative
regalloc_negative:
    r6 = r1
    r1 = 0
    *(u64*)(r10 - 8) = r1
    r2 = r10
    r2 += -8
    r1 = map_hash_48b ll
    call bpf_map_lookup_elem
    if r0 == 0 goto 0f
    r7 = r0
    call bpf_get_prandom_u32
    r2 = r0
    if r0 s> 24 goto 0f
    if r2 s< 0 goto 0f
    r7 += r0
    r7 += r2
    r0 = *(u8*)(r7 + 0)
0:  exit
"#
);

// SEC("tracepoint")
// __description("regalloc src_reg mark")
// __success __flag(BPF_F_ANY_ALIGNMENT)
// __naked void regalloc_src_reg_mark(void)
global_asm!(
    r#"
    .section tracepoint,"ax"
    .global regalloc_src_reg_mark
regalloc_src_reg_mark:
    r6 = r1
    r1 = 0
    *(u64*)(r10 - 8) = r1
    r2 = r10
    r2 += -8
    r1 = map_hash_48b ll
    call bpf_map_lookup_elem
    if r0 == 0 goto 0f
    r7 = r0
    call bpf_get_prandom_u32
    r2 = r0
    if r0 s> 20 goto 0f
    r3 = 0
    if r3 s>= r2 goto 0f
    r7 += r0
    r7 += r2
    r0 = *(u64*)(r7 + 0)
0:  exit
"#
);

// SEC("tracepoint")
// __description("regalloc src_reg negative")
// __failure __msg("invalid access to map value, value_size=48 off=44 size=8")
// __flag(BPF_F_ANY_ALIGNMENT)
// __naked void regalloc_src_reg_negative(void)
global_asm!(
    r#"
    .section tracepoint,"ax"
    .global regalloc_src_reg_negative
regalloc_src_reg_negative:
    r6 = r1
    r1 = 0
    *(u64*)(r10 - 8) = r1
    r2 = r10
    r2 += -8
    r1 = map_hash_48b ll
    call bpf_map_lookup_elem
    if r0 == 0 goto 0f
    r7 = r0
    call bpf_get_prandom_u32
    r2 = r0
    if r0 s> 22 goto 0f
    r3 = 0
    if r3 s>= r2 goto 0f
    r7 += r0
    r7 += r2
    r0 = *(u64*)(r7 + 0)
0:  exit
"#
);

// SEC("tracepoint")
// __description("regalloc and spill")
// __success __flag(BPF_F_ANY_ALIGNMENT)
// __naked void regalloc_and_spill(void)
global_asm!(
    r#"
    .section tracepoint,"ax"
    .global regalloc_and_spill
regalloc_and_spill:
    r6 = r1
    r1 = 0
    *(u64*)(r10 - 8) = r1
    r2 = r10
    r2 += -8
    r1 = map_hash_48b ll
    call bpf_map_lookup_elem
    if r0 == 0 goto 0f
    r7 = r0
    call bpf_get_prandom_u32
    r2 = r0
    if r0 s> 20 goto 0f
    /* r0 has upper bound that should propagate into r2 */
    *(u64*)(r10 - 8) = r2        /* spill r2 */
    r0 = 0
    r2 = 0                      /* clear r0 and r2 */
    r3 = *(u64*)(r10 - 8)        /* fill r3 */
    if r0 s>= r3 goto 0f
    /* r3 has lower and upper bounds */
    r7 += r3
    r0 = *(u64*)(r7 + 0)
0:  exit
"#
);

// SEC("tracepoint")
// __description("regalloc and spill negative")
// __failure __msg("invalid access to map value, value_size=48 off=48 size=8")
// __flag(BPF_F_ANY_ALIGNMENT)
// __naked void regalloc_and_spill_negative(void)
global_asm!(
    r#"
    .section tracepoint,"ax"
    .global regalloc_and_spill_negative
regalloc_and_spill_negative:
    r6 = r1
    r1 = 0
    *(u64*)(r10 - 8) = r1
    r2 = r10
    r2 += -8
    r1 = map_hash_48b ll
    call bpf_map_lookup_elem
    if r0 == 0 goto 0f
    r7 = r0
    call bpf_get_prandom_u32
    r2 = r0
    if r0 s> 48 goto 0f
    /* r0 has upper bound that should propagate into r2 */
    *(u64*)(r10 - 8) = r2        /* spill r2 */
    r0 = 0
    r2 = 0                      /* clear r0 and r2 */
    r3 = *(u64*)(r10 - 8)        /* fill r3 */
    if r0 s>= r3 goto 0f
    /* r3 has lower and upper bounds */
    r7 += r3
    r0 = *(u64*)(r7 + 0)
0:  exit
"#
);

// SEC("tracepoint")
// __description("regalloc three regs")
// __success __flag(BPF_F_ANY_ALIGNMENT)
// __naked void regalloc_three_regs(void)
global_asm!(
    r#"
    .section tracepoint,"ax"
    .global regalloc_three_regs
regalloc_three_regs:
    r6 = r1
    r1 = 0
    *(u64*)(r10 - 8) = r1
    r2 = r10
    r2 += -8
    r1 = map_hash_48b ll
    call bpf_map_lookup_elem
    if r0 == 0 goto 0f
    r7 = r0
    call bpf_get_prandom_u32
    r2 = r0
    r4 = r2
    if r0 s> 12 goto 0f
    if r2 s< 0 goto 0f
    r7 += r0
    r7 += r2
    r7 += r4
    r0 = *(u64*)(r7 + 0)
0:  exit
"#
);

// SEC("tracepoint")
// __description("regalloc after call")
// __success __flag(BPF_F_ANY_ALIGNMENT)
// __naked void regalloc_after_call(void)
global_asm!(
    r#"
    .section tracepoint,"ax"
    .global regalloc_after_call
regalloc_after_call:
    r6 = r1
    r1 = 0
    *(u64*)(r10 - 8) = r1
    r2 = r10
    r2 += -8
    r1 = map_hash_48b ll
    call bpf_map_lookup_elem
    if r0 == 0 goto 0f
    r7 = r0
    call bpf_get_prandom_u32
    r8 = r0
    r9 = r0
    call regalloc_after_call__1
    if r8 s> 20 goto 0f
    if r9 s< 0 goto 0f
    r7 += r8
    r7 += r9
    r0 = *(u64*)(r7 + 0)
0:  exit

    .global regalloc_after_call__1
regalloc_after_call__1:
    r0 = 0
    exit
"#
);

// SEC("tracepoint")
// __description("regalloc in callee")
// __success __flag(BPF_F_ANY_ALIGNMENT)
// __naked void regalloc_in_callee(void)
global_asm!(
    r#"
    .section tracepoint,"ax"
    .global regalloc_in_callee
regalloc_in_callee:
    r6 = r1
    r1 = 0
    *(u64*)(r10 - 8) = r1
    r2 = r10
    r2 += -8
    r1 = map_hash_48b ll
    call bpf_map_lookup_elem
    if r0 == 0 goto 0f
    r7 = r0
    call bpf_get_prandom_u32
    r1 = r0
    r2 = r0
    r3 = r7
    call regalloc_in_callee__1
0:  exit

    .global regalloc_in_callee__1
regalloc_in_callee__1:
    if r1 s> 20 goto 1f
    if r2 s< 0 goto 1f
    r3 += r1
    r3 += r2
    r0 = *(u64*)(r3 + 0)
    exit
1:  r0 = 0
    exit
"#
);

// SEC("tracepoint")
// __description("regalloc, spill, JEQ")
// __success
// __naked void regalloc_spill_jeq(void)
global_asm!(
    r#"
    .section tracepoint,"ax"
    .global regalloc_spill_jeq
regalloc_spill_jeq:
    r6 = r1
    r1 = 0
    *(u64*)(r10 - 8) = r1
    r2 = r10
    r2 += -8
    r1 = map_hash_48b ll
    call bpf_map_lookup_elem
    *(u64*)(r10 - 8) = r0        /* spill r0 */
    if r0 == 0 goto 0f
0:  /* The verifier will walk the rest twice with r0 == 0 and r0 == map_value */
    call bpf_get_prandom_u32
    r2 = r0
    if r2 == 20 goto 1f
1:  /* The verifier will walk the rest two more times with r0 == 20 and r0 == unknown */
    r3 = *(u64*)(r10 - 8)        /* fill r3 with map_value */
    if r3 == 0 goto 2f           /* skip ldx if map_value == NULL */
    /* Buggy verifier will think that r3 == 20 here */
    r0 = *(u64*)(r3 + 0)         /* read from map_value */
2:  exit
"#
);

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
