// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/value_illegal_alu.c */

// C dependencies removed from executable Rust:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, "../../../include/linux/filter.h", "bpf_misc.h"

pub const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: ::core::ffi::c_uint,
    pub foo: [::core::ffi::c_int; MAX_ENTRIES],
}

// Original C map definition used:
// __uint(type, BPF_MAP_TYPE_HASH);
// __uint(max_entries, 1);
// __type(key, long long);
// __type(value, struct test_val);
// SEC(".maps")
#[repr(C)]
pub struct map_hash_48b_def {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut map_hash_48b: map_hash_48b_def = map_hash_48b_def { _private: [] };

extern "C" {
    fn bpf_map_lookup_elem() -> i64;
    fn bpf_get_prandom_u32() -> i64;
}

// SEC("socket")
// __description("map element value illegal alu op, 1")
// __failure __msg("R0 bitwise operator &= on pointer")
// __failure_unpriv
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn value_illegal_alu_op_1() {
    ::core::arch::asm!(
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r0 &= 8",
        "r1 = 22",
        "*(u64*)(r0 + 0) = r1",
        "0:",
        "exit",
        map_hash_48b = sym map_hash_48b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

// SEC("socket")
// __description("map element value illegal alu op, 2")
// __failure __msg("R0 32-bit pointer arithmetic prohibited")
// __failure_unpriv
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn value_illegal_alu_op_2() {
    ::core::arch::asm!(
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "w0 += 0",
        "r1 = 22",
        "*(u64*)(r0 + 0) = r1",
        "0:",
        "exit",
        map_hash_48b = sym map_hash_48b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

// SEC("socket")
// __description("map element value illegal alu op, 3")
// __failure __msg("R0 pointer arithmetic with /= operator")
// __failure_unpriv
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn value_illegal_alu_op_3() {
    ::core::arch::asm!(
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r0 /= 42",
        "r1 = 22",
        "*(u64*)(r0 + 0) = r1",
        "0:",
        "exit",
        map_hash_48b = sym map_hash_48b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

// SEC("socket")
// __description("map element value illegal alu op, 4")
// __failure __msg("invalid mem access 'scalar'")
// __failure_unpriv __msg_unpriv("R0 pointer arithmetic prohibited")
// __flag(BPF_F_ANY_ALIGNMENT)
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn value_illegal_alu_op_4() {
    ::core::arch::asm!(
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r0 = be64 r0",
        "r1 = 22",
        "*(u64*)(r0 + 0) = r1",
        "0:",
        "exit",
        map_hash_48b = sym map_hash_48b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

// SEC("socket")
// __description("map element value illegal alu op, 5")
// __failure __msg("R0 invalid mem access 'scalar'")
// __msg_unpriv("leaking pointer from stack off -8")
// __flag(BPF_F_ANY_ALIGNMENT)
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn value_illegal_alu_op_5() {
    ::core::arch::asm!(
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r3 = 4096",
        "r2 = r10",
        "r2 += -8",
        "*(u64*)(r2 + 0) = r0",
        "lock *(u64 *)(r2 + 0) += r3",
        "r0 = *(u64*)(r2 + 0)",
        "r1 = 22",
        "*(u64*)(r0 + 0) = r1",
        "0:",
        "exit",
        map_hash_48b = sym map_hash_48b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

// SEC("socket")
// __description("map_ptr illegal alu op, map_ptr = -map_ptr")
// __failure __msg("R0 invalid mem access 'scalar'")
// __failure_unpriv __msg_unpriv("R0 pointer arithmetic prohibited")
// __flag(BPF_F_ANY_ALIGNMENT)
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn map_ptr_illegal_alu_op() {
    ::core::arch::asm!(
        "r0 = {map_hash_48b} ll",
        "r0 = -r0",
        "r1 = 22",
        "*(u64*)(r0 + 0) = r1",
        "exit",
        map_hash_48b = sym map_hash_48b,
        options(noreturn)
    );
}

// SEC("flow_dissector")
// __description("flow_keys illegal alu op with variable offset")
// __failure __msg("R7 pointer arithmetic on flow_keys prohibited")
// __naked
#[no_mangle]
#[link_section = "flow_dissector"]
pub unsafe extern "C" fn flow_keys_illegal_variable_offset_alu() {
    ::core::arch::asm!(
        "r6 = r1",
        "r7 = *(u64*)(r6 + {flow_keys_off})",
        "call {bpf_get_prandom_u32}",
        "r8 = r0",
        "r8 &= 8",
        "r7 += r8",
        "r0 = *(u64*)(r7 + 0)",
        "exit",
        flow_keys_off = const 0, // offsetof(struct __sk_buff, flow_keys)
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn)
    );
}

/*
 * Offset fields of 0 and 1 are legal for BPF_{DIV,MOD} instructions.
 * Offset fields of 0 are legal for the rest of ALU instructions.
 * Test that error is reported for illegal offsets, assuming that tests
 * for legal offsets exist.
 */

// DEFINE_BAD_OFFSET_TEST(bad_offset_divx, BPF_ALU64 | BPF_DIV | BPF_X, -1, 0)
// SEC("socket")
// __failure __msg("BPF_ALU uses reserved fields")
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn bad_offset_divx() {
    ::core::arch::asm!(
        "r0 = 1",
        ".8byte {insn}",
        "r0 = 0",
        "exit",
        insn = const 0, // BPF_RAW_INSN(BPF_ALU64 | BPF_DIV | BPF_X, 0, 0, -1, 0)
        options(noreturn)
    );
}

// DEFINE_BAD_OFFSET_TEST(bad_offset_modk, BPF_ALU64 | BPF_MOD | BPF_K, -1, 1)
// SEC("socket")
// __failure __msg("BPF_ALU uses reserved fields")
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn bad_offset_modk() {
    ::core::arch::asm!(
        "r0 = 1",
        ".8byte {insn}",
        "r0 = 0",
        "exit",
        insn = const 0, // BPF_RAW_INSN(BPF_ALU64 | BPF_MOD | BPF_K, 0, 0, -1, 1)
        options(noreturn)
    );
}

// DEFINE_BAD_OFFSET_TEST(bad_offset_addx, BPF_ALU64 | BPF_ADD | BPF_X, -1, 0)
// SEC("socket")
// __failure __msg("BPF_ALU uses reserved fields")
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn bad_offset_addx() {
    ::core::arch::asm!(
        "r0 = 1",
        ".8byte {insn}",
        "r0 = 0",
        "exit",
        insn = const 0, // BPF_RAW_INSN(BPF_ALU64 | BPF_ADD | BPF_X, 0, 0, -1, 0)
        options(noreturn)
    );
}

// DEFINE_BAD_OFFSET_TEST(bad_offset_divx2, BPF_ALU64 | BPF_DIV | BPF_X, 2, 0)
// SEC("socket")
// __failure __msg("BPF_ALU uses reserved fields")
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn bad_offset_divx2() {
    ::core::arch::asm!(
        "r0 = 1",
        ".8byte {insn}",
        "r0 = 0",
        "exit",
        insn = const 0, // BPF_RAW_INSN(BPF_ALU64 | BPF_DIV | BPF_X, 0, 0, 2, 0)
        options(noreturn)
    );
}

// DEFINE_BAD_OFFSET_TEST(bad_offset_modk2, BPF_ALU64 | BPF_MOD | BPF_K, 2, 1)
// SEC("socket")
// __failure __msg("BPF_ALU uses reserved fields")
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn bad_offset_modk2() {
    ::core::arch::asm!(
        "r0 = 1",
        ".8byte {insn}",
        "r0 = 0",
        "exit",
        insn = const 0, // BPF_RAW_INSN(BPF_ALU64 | BPF_MOD | BPF_K, 0, 0, 2, 1)
        options(noreturn)
    );
}

// DEFINE_BAD_OFFSET_TEST(bad_offset_addx2, BPF_ALU64 | BPF_ADD | BPF_X, 1, 0)
// SEC("socket")
// __failure __msg("BPF_ALU uses reserved fields")
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn bad_offset_addx2() {
    ::core::arch::asm!(
        "r0 = 1",
        ".8byte {insn}",
        "r0 = 0",
        "exit",
        insn = const 0, // BPF_RAW_INSN(BPF_ALU64 | BPF_ADD | BPF_X, 0, 0, 1, 0)
        options(noreturn)
    );
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
