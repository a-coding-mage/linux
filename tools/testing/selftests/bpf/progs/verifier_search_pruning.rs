// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/search_pruning.c */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;

/* C dependencies removed from executable Rust:
 * #include <linux/bpf.h>
 * #include <../../../include/linux/filter.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 */

pub const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: u32,
    pub foo: [i32; MAX_ENTRIES],
}

#[repr(C)]
pub struct map_hash_48b_def {
    /* __uint(type, BPF_MAP_TYPE_HASH);
     * __uint(max_entries, 1);
     * __type(key, long long);
     * __type(value, struct test_val);
     */
    _unused: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static map_hash_48b: map_hash_48b_def = map_hash_48b_def { _unused: [] };

#[repr(C)]
pub struct map_hash_8b_def {
    /* __uint(type, BPF_MAP_TYPE_HASH);
     * __uint(max_entries, 1);
     * __type(key, long long);
     * __type(value, long long);
     */
    _unused: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static map_hash_8b: map_hash_8b_def = map_hash_8b_def { _unused: [] };

extern "C" {
    fn bpf_map_lookup_elem() -> i64;
    fn bpf_ktime_get_ns() -> i64;
    fn bpf_get_prandom_u32() -> u32;
}

pub const POINTER_VALUE: i32 = 0;
pub const BPF_F_ANY_ALIGNMENT: u32 = 0;
pub const BPF_F_TEST_STATE_FREQ: u32 = 0;

#[link_section = "socket"]
#[no_mangle]
/* __description("pointer/scalar confusion in state equality check (way 1)")
 * __success __failure_unpriv __msg_unpriv("R0 leaks addr as return value")
 * __retval(POINTER_VALUE)
 */
pub unsafe extern "C" fn state_equality_check_way_1() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r0 = *(u64*)(r0 + 0)",
        "goto 1f",
        "0:",
        "r0 = r10",
        "1:",
        "goto 2f",
        "2:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
/* __description("pointer/scalar confusion in state equality check (way 2)")
 * __success __failure_unpriv __msg_unpriv("R0 leaks addr as return value")
 * __retval(POINTER_VALUE)
 */
pub unsafe extern "C" fn state_equality_check_way_2() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 != 0 goto 0f",
        "r0 = r10",
        "goto 1f",
        "0:",
        "r0 = *(u64*)(r0 + 0)",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

#[link_section = "lwt_in"]
#[no_mangle]
/* __description("liveness pruning and write screening")
 * __failure __msg("R0 !read_ok")
 */
pub unsafe extern "C" fn liveness_pruning_and_write_screening() {
    asm!(
        "/* Get an unknown value */",
        "r2 = *(u32*)(r1 + 0)",
        "/* branch conditions teach us nothing about R2 */",
        "if r2 >= 0 goto 0f",
        "r0 = 0",
        "0:",
        "if r2 >= 0 goto 1f",
        "r0 = 0",
        "1:",
        "exit",
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
/* __description("varlen_map_value_access pruning")
 * __failure __msg("R0 unbounded memory access")
 * __failure_unpriv __msg_unpriv("R0 leaks addr")
 * __flag(BPF_F_ANY_ALIGNMENT)
 */
pub unsafe extern "C" fn varlen_map_value_access_pruning() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = *(u64*)(r0 + 0)",
        "w2 = {max_entries}",
        "if r2 s> r1 goto 1f",
        "w1 = 0",
        "1:",
        "w1 <<= 2",
        "r0 += r1",
        "goto 2f",
        "2:",
        "r1 = {test_val_foo}",
        "*(u64*)(r0 + 0) = r1",
        "0:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        max_entries = const MAX_ENTRIES,
        test_val_foo = const 4,
        options(noreturn)
    );
}

#[link_section = "tracepoint"]
#[no_mangle]
/* __description("search pruning: all branches should be verified (nop operation)")
 * __failure __msg("R6 invalid mem access 'scalar'")
 */
pub unsafe extern "C" fn should_be_verified_nop_operation() {
    asm!(
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r3 = *(u64*)(r0 + 0)",
        "if r3 == 0xbeef goto 1f",
        "r4 = 0",
        "goto 2f",
        "1:",
        "r4 = 1",
        "2:",
        "*(u64*)(r10 - 16) = r4",
        "call {bpf_ktime_get_ns}",
        "r5 = *(u64*)(r10 - 16)",
        "if r5 == 0 goto 0f",
        "r6 = 0",
        "r1 = 0xdead",
        "*(u64*)(r6 + 0) = r1",
        "0:",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
/* __description("search pruning: all branches should be verified (invalid stack access)")
 * in privileged mode reads from uninitialized stack locations are permitted
 * __success __failure_unpriv
 * __msg_unpriv("invalid read from stack off -16+0 size 8")
 * __retval(0)
 */
pub unsafe extern "C" fn be_verified_invalid_stack_access() {
    asm!(
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r3 = *(u64*)(r0 + 0)",
        "r4 = 0",
        "if r3 == 0xbeef goto 1f",
        "*(u64*)(r10 - 16) = r4",
        "goto 2f",
        "1:",
        "*(u64*)(r10 - 24) = r4",
        "2:",
        "call {bpf_ktime_get_ns}",
        "r5 = *(u64*)(r10 - 16)",
        "0:",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

#[link_section = "tracepoint"]
#[no_mangle]
/* __description("precision tracking for u32 spill/fill")
 * __failure __msg("R0 min value is outside of the allowed memory range")
 */
pub unsafe extern "C" fn tracking_for_u32_spill_fill() {
    asm!(
        "r7 = r1",
        "call {bpf_get_prandom_u32}",
        "w6 = 32",
        "if r0 == 0 goto 0f",
        "w6 = 4",
        "0:",
        "/* Additional insns to introduce a pruning point. */",
        "call {bpf_get_prandom_u32}",
        "r3 = 0",
        "r3 = 0",
        "if r0 == 0 goto 1f",
        "r3 = 0",
        "1:",
        "/* u32 spill/fill */",
        "*(u32*)(r10 - 8) = r6",
        "r8 = *(u32*)(r10 - 8)",
        "/* out-of-bound map value access for r6=32 */",
        "r1 = 0",
        "*(u64*)(r10 - 16) = r1",
        "r2 = r10",
        "r2 += -16",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 2f",
        "r0 += r8",
        "r1 = *(u32*)(r0 + 0)",
        "2:",
        "r0 = 0",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

#[link_section = "tracepoint"]
#[no_mangle]
/* __description("precision tracking for u32 spills, u64 fill")
 * __failure __msg("div by zero")
 */
pub unsafe extern "C" fn for_u32_spills_u64_fill() {
    asm!(
        "call {bpf_get_prandom_u32}",
        "r6 = r0",
        "w7 = 0xffffffff",
        "/* Additional insns to introduce a pruning point. */",
        "r3 = 1",
        "r3 = 1",
        "r3 = 1",
        "r3 = 1",
        "call {bpf_get_prandom_u32}",
        "if r0 == 0 goto 0f",
        "r3 = 1",
        "0:",
        "w3 /= 0",
        "/* u32 spills, u64 fill */",
        "*(u32*)(r10 - 4) = r6",
        "*(u32*)(r10 - 8) = r7",
        "r8 = *(u64*)(r10 - 8)",
        "/* if r8 != X goto pc+1  r8 known in fallthrough branch */",
        "if r8 != 0xffffffff goto 1f",
        "r3 = 1",
        "1:",
        "/* if r8 == X goto pc+1  condition always true on first traversal, so starts backtracking to mark r8 as requiring precision. r7 marked as needing precision. r6 not marked since it's not tracked. */",
        "if r8 == 0xffffffff goto 2f",
        "/* fails if r8 correctly marked unknown after fill. */",
        "w3 /= 0",
        "2:",
        "r0 = 0",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
/* __description("allocated_stack")
 * __success __msg("processed 15 insns")
 * __success_unpriv __msg_unpriv("") __log_level(1) __retval(0)
 */
pub unsafe extern "C" fn allocated_stack() {
    asm!(
        "r6 = r1",
        "call {bpf_get_prandom_u32}",
        "r7 = r0",
        "if r0 == 0 goto 0f",
        "r0 = 0",
        "*(u64*)(r10 - 8) = r6",
        "r6 = *(u64*)(r10 - 8)",
        "*(u8*)(r10 - 9) = r7",
        "r7 = *(u8*)(r10 - 9)",
        "0:",
        "if r0 != 0 goto 1f",
        "1:",
        "if r0 != 0 goto 2f",
        "2:",
        "if r0 != 0 goto 3f",
        "3:",
        "if r0 != 0 goto 4f",
        "4:",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn)
    );
}

/* The test performs a conditional 64-bit write to a stack location
 * fp[-8], this is followed by an unconditional 8-bit write to fp[-8],
 * then data is read from fp[-8]. This sequence is unsafe.
 *
 * The test would be mistakenly marked as safe w/o dst register parent
 * preservation in verifier.c:copy_register_state() function.
 *
 * Note the usage of BPF_F_TEST_STATE_FREQ to force creation of the
 * checkpoint state after conditional 64-bit assignment.
 */

#[link_section = "socket"]
#[no_mangle]
/* __description("write tracking and register parent chain bug")
 * in privileged mode reads from uninitialized stack locations are permitted
 * __success __failure_unpriv
 * __msg_unpriv("invalid read from stack off -8+1 size 8")
 * __retval(0) __flag(BPF_F_TEST_STATE_FREQ)
 */
pub unsafe extern "C" fn and_register_parent_chain_bug() {
    asm!(
        "/* r6 = ktime_get_ns() */",
        "call {bpf_ktime_get_ns}",
        "r6 = r0",
        "/* r0 = ktime_get_ns() */",
        "call {bpf_ktime_get_ns}",
        "/* if r0 > r6 goto +1 */",
        "if r0 > r6 goto 0f",
        "/* *(u64 *)(r10 - 8) = 0xdeadbeef */",
        "r0 = 0xdeadbeef",
        "*(u64*)(r10 - 8) = r0",
        "0:",
        "r1 = 42",
        "*(u8*)(r10 - 8) = r1",
        "r2 = *(u64*)(r10 - 8)",
        "/* exit(0) */",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Without checkpoint forcibly inserted at the back-edge a loop this
 * test would take a very long time to verify.
 */
#[link_section = "kprobe"]
#[no_mangle]
/* __failure __log_level(4)
 * __msg("BPF program is too large.")
 */
pub unsafe extern "C" fn short_loop1() {
    asm!(
        "r7 = *(u16 *)(r1 +0)",
        "1:",
        "r7 += 0x1ab064b9",
        ".8byte {jset}",
        "r7 &= 0x1ee60e",
        "r7 += r1",
        "if r7 s> 0x37d2 goto +0",
        "r0 = 0",
        "exit",
        jset = const 0,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
