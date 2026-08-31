// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/helper_access_var_len.c */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::asm;

const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: u32,
    pub foo: [i32; MAX_ENTRIES],
}

/*
 * C dependencies:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 *
 * The SEC(), __description(), __success, __failure, __failure_unpriv,
 * __msg(), __msg_unpriv(), __retval(), __flag(), __naked, __uint(),
 * __type(), __imm(), __imm_addr(), __imm_const(), and __clobber_all verifier
 * annotations are preserved below as Rust attributes/comments where Rust has
 * no direct file-local equivalent.
 */

#[repr(C)]
pub struct map_hash_48b_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i64,
    pub value: test_val,
}

#[repr(C)]
pub struct map_hash_8b_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i64,
    pub value: i64,
}

#[repr(C)]
pub struct map_ringbuf_def {
    pub type_: u32,
    pub max_entries: u32,
}

extern "C" {
    #[link_name = "map_hash_48b"]
    pub static mut map_hash_48b: map_hash_48b_def;
    #[link_name = "map_hash_8b"]
    pub static mut map_hash_8b: map_hash_8b_def;
    #[link_name = "map_ringbuf"]
    pub static mut map_ringbuf: map_ringbuf_def;

    pub fn bpf_probe_read_kernel(...) -> i64;
    pub fn bpf_get_prandom_u32() -> u32;
    pub fn bpf_ringbuf_output(...) -> i64;
    pub fn bpf_map_lookup_elem(...) -> *mut core::ffi::c_void;
    pub fn bpf_csum_diff(...) -> i64;
}

// SEC("tracepoint")
// __description("helper access to variable memory: stack, bitwise AND + JMP, correct bounds")
// __success
#[no_mangle]
pub unsafe extern "C" fn bitwise_and_jmp_correct_bounds() {
    asm!(
        "r1 = r10",
        "r1 += -64",
        "r0 = 0",
        "*(u64*)(r10 - 64) = r0",
        "*(u64*)(r10 - 56) = r0",
        "*(u64*)(r10 - 48) = r0",
        "*(u64*)(r10 - 40) = r0",
        "*(u64*)(r10 - 32) = r0",
        "*(u64*)(r10 - 24) = r0",
        "*(u64*)(r10 - 16) = r0",
        "*(u64*)(r10 - 8) = r0",
        "r2 = 16",
        "*(u64*)(r1 - 128) = r2",
        "r2 = *(u64*)(r1 - 128)",
        "r2 &= 64",
        "r4 = 0",
        "if r4 >= r2 goto 0f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "0:",
        "r0 = 0",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("socket")
// __description("helper access to variable memory: stack, bitwise AND, zero included")
/* in privileged mode reads from uninitialized stack locations are permitted */
// __success __failure_unpriv
// __msg_unpriv("invalid read from stack R2 off -64+0 size 64")
// __retval(0)
#[no_mangle]
pub unsafe extern "C" fn stack_bitwise_and_zero_included() {
    asm!(
        "/* set max stack size */",
        "r6 = 0",
        "*(u64*)(r10 - 128) = r6",
        "/* set r3 to a random value */",
        "call {bpf_get_prandom_u32}",
        "r3 = r0",
        "/* use bitwise AND to limit r3 range to [0, 64] */",
        "r3 &= 64",
        "r1 = {map_ringbuf} ll",
        "r2 = r10",
        "r2 += -64",
        "r4 = 0",
        "/* Call bpf_ringbuf_output(), it is one of a few helper functions with",
        " * ARG_MEM_SIZE_OR_ZERO parameter allowed in unpriv mode.",
        " * For unpriv this should signal an error, because memory at &fp[-64] is",
        " * not initialized.",
        " */",
        "call {bpf_ringbuf_output}",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        bpf_ringbuf_output = sym bpf_ringbuf_output,
        map_ringbuf = sym map_ringbuf,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: stack, bitwise AND + JMP, wrong max")
// __failure __msg("invalid write to stack R1 off=-64 size=65")
#[no_mangle]
pub unsafe extern "C" fn bitwise_and_jmp_wrong_max() {
    asm!(
        "r2 = *(u64*)(r1 + 8)",
        "r1 = r10",
        "r1 += -64",
        "*(u64*)(r1 - 128) = r2",
        "r2 = *(u64*)(r1 - 128)",
        "r2 &= 65",
        "r4 = 0",
        "if r4 >= r2 goto 0f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "0:",
        "r0 = 0",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: stack, JMP, correct bounds")
// __success
#[no_mangle]
pub unsafe extern "C" fn memory_stack_jmp_correct_bounds() {
    asm!(
        "r1 = r10",
        "r1 += -64",
        "r0 = 0",
        "*(u64*)(r10 - 64) = r0",
        "*(u64*)(r10 - 56) = r0",
        "*(u64*)(r10 - 48) = r0",
        "*(u64*)(r10 - 40) = r0",
        "*(u64*)(r10 - 32) = r0",
        "*(u64*)(r10 - 24) = r0",
        "*(u64*)(r10 - 16) = r0",
        "*(u64*)(r10 - 8) = r0",
        "r2 = 16",
        "*(u64*)(r1 - 128) = r2",
        "r2 = *(u64*)(r1 - 128)",
        "if r2 > 64 goto 0f",
        "r4 = 0",
        "if r4 >= r2 goto 0f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "0:",
        "r0 = 0",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: stack, JMP (signed), correct bounds")
// __success
#[no_mangle]
pub unsafe extern "C" fn stack_jmp_signed_correct_bounds() {
    asm!(
        "r1 = r10",
        "r1 += -64",
        "r0 = 0",
        "*(u64*)(r10 - 64) = r0",
        "*(u64*)(r10 - 56) = r0",
        "*(u64*)(r10 - 48) = r0",
        "*(u64*)(r10 - 40) = r0",
        "*(u64*)(r10 - 32) = r0",
        "*(u64*)(r10 - 24) = r0",
        "*(u64*)(r10 - 16) = r0",
        "*(u64*)(r10 - 8) = r0",
        "r2 = 16",
        "*(u64*)(r1 - 128) = r2",
        "r2 = *(u64*)(r1 - 128)",
        "if r2 s> 64 goto 0f",
        "r4 = 0",
        "if r4 s>= r2 goto 0f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "0:",
        "r0 = 0",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: stack, JMP, bounds + offset")
// __failure __msg("invalid write to stack R1 off=-64 size=65")
#[no_mangle]
pub unsafe extern "C" fn memory_stack_jmp_bounds_offset() {
    asm!(
        "r2 = *(u64*)(r1 + 8)",
        "r1 = r10",
        "r1 += -64",
        "*(u64*)(r1 - 128) = r2",
        "r2 = *(u64*)(r1 - 128)",
        "if r2 > 64 goto 0f",
        "r4 = 0",
        "if r4 >= r2 goto 0f",
        "r2 += 1",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "0:",
        "r0 = 0",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: stack, JMP, wrong max")
// __failure __msg("invalid write to stack R1 off=-64 size=65")
#[no_mangle]
pub unsafe extern "C" fn memory_stack_jmp_wrong_max() {
    asm!(
        "r2 = *(u64*)(r1 + 8)",
        "r1 = r10",
        "r1 += -64",
        "*(u64*)(r1 - 128) = r2",
        "r2 = *(u64*)(r1 - 128)",
        "if r2 > 65 goto 0f",
        "r4 = 0",
        "if r4 >= r2 goto 0f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "0:",
        "r0 = 0",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: stack, JMP, no max check")
// __failure
/* because max wasn't checked, signed min is negative */
// __msg("R2 min value is negative, either use unsigned or 'var &= const'")
#[no_mangle]
pub unsafe extern "C" fn stack_jmp_no_max_check() {
    asm!(
        "r2 = *(u64*)(r1 + 8)",
        "r1 = r10",
        "r1 += -64",
        "*(u64*)(r1 - 128) = r2",
        "r2 = *(u64*)(r1 - 128)",
        "r4 = 0",
        "if r4 >= r2 goto 0f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "0:",
        "r0 = 0",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("socket")
// __description("helper access to variable memory: stack, JMP, no min check")
/* in privileged mode reads from uninitialized stack locations are permitted */
// __success __failure_unpriv
// __msg_unpriv("invalid read from stack R2 off -64+0 size 64")
// __retval(0)
#[no_mangle]
pub unsafe extern "C" fn stack_jmp_no_min_check() {
    asm!(
        "/* set max stack size */",
        "r6 = 0",
        "*(u64*)(r10 - 128) = r6",
        "/* set r3 to a random value */",
        "call {bpf_get_prandom_u32}",
        "r3 = r0",
        "/* use JMP to limit r3 range to [0, 64] */",
        "if r3 > 64 goto 0f",
        "r1 = {map_ringbuf} ll",
        "r2 = r10",
        "r2 += -64",
        "r4 = 0",
        "/* Call bpf_ringbuf_output(), it is one of a few helper functions with",
        " * ARG_MEM_SIZE_OR_ZERO parameter allowed in unpriv mode.",
        " * For unpriv this should signal an error, because memory at &fp[-64] is",
        " * not initialized.",
        " */",
        "call {bpf_ringbuf_output}",
        "0:",
        "r0 = 0",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        bpf_ringbuf_output = sym bpf_ringbuf_output,
        map_ringbuf = sym map_ringbuf,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: stack, JMP (signed), no min check")
// __failure __msg("R2 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn jmp_signed_no_min_check() {
    asm!(
        "r2 = *(u64*)(r1 + 8)",
        "r1 = r10",
        "r1 += -64",
        "*(u64*)(r1 - 128) = r2",
        "r2 = *(u64*)(r1 - 128)",
        "if r2 s> 64 goto 0f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "r0 = 0",
        "0:",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: map, JMP, correct bounds")
// __success
#[no_mangle]
pub unsafe extern "C" fn memory_map_jmp_correct_bounds() {
    asm!(
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = r0",
        "r2 = {sizeof_test_val}",
        "*(u64*)(r10 - 128) = r2",
        "r2 = *(u64*)(r10 - 128)",
        "if r2 s> {sizeof_test_val} goto 1f",
        "r4 = 0",
        "if r4 s>= r2 goto 1f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "1:",
        "r0 = 0",
        "0:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        map_hash_48b = sym map_hash_48b,
        sizeof_test_val = const core::mem::size_of::<test_val>(),
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: map, JMP, wrong max")
// __failure __msg("invalid access to map value, value_size=48 off=0 size=49")
#[no_mangle]
pub unsafe extern "C" fn memory_map_jmp_wrong_max() {
    asm!(
        "r6 = *(u64*)(r1 + 8)",
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = r0",
        "r2 = r6",
        "*(u64*)(r10 - 128) = r2",
        "r2 = *(u64*)(r10 - 128)",
        "if r2 s> {imm_0} goto 1f",
        "r4 = 0",
        "if r4 s>= r2 goto 1f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "1:",
        "r0 = 0",
        "0:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        map_hash_48b = sym map_hash_48b,
        imm_0 = const core::mem::size_of::<test_val>() + 1,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: map adjusted, JMP, correct bounds")
// __success
#[no_mangle]
pub unsafe extern "C" fn map_adjusted_jmp_correct_bounds() {
    asm!(
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = r0",
        "r1 += 20",
        "r2 = {sizeof_test_val}",
        "*(u64*)(r10 - 128) = r2",
        "r2 = *(u64*)(r10 - 128)",
        "if r2 s> {imm_0} goto 1f",
        "r4 = 0",
        "if r4 s>= r2 goto 1f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "1:",
        "r0 = 0",
        "0:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        map_hash_48b = sym map_hash_48b,
        imm_0 = const core::mem::size_of::<test_val>() - 20,
        sizeof_test_val = const core::mem::size_of::<test_val>(),
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: map adjusted, JMP, wrong max")
// __failure __msg("R1 min value is outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn map_adjusted_jmp_wrong_max() {
    asm!(
        "r6 = *(u64*)(r1 + 8)",
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = r0",
        "r1 += 20",
        "r2 = r6",
        "*(u64*)(r10 - 128) = r2",
        "r2 = *(u64*)(r10 - 128)",
        "if r2 s> {imm_0} goto 1f",
        "r4 = 0",
        "if r4 s>= r2 goto 1f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "1:",
        "r0 = 0",
        "0:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        map_hash_48b = sym map_hash_48b,
        imm_0 = const core::mem::size_of::<test_val>() - 19,
        options(noreturn)
    );
}

// SEC("tc")
// __description("helper access to variable memory: size = 0 allowed on NULL (ARG_PTR_TO_MEM_OR_NULL)")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_1() {
    asm!(
        "r1 = 0",
        "r2 = 0",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "exit",
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

// SEC("tc")
// __description("helper access to variable memory: size > 0 not allowed on NULL (ARG_PTR_TO_MEM_OR_NULL)")
// __failure __msg("R1 type=scalar expected=fp")
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_2() {
    asm!(
        "r2 = *(u32*)(r1 + 0)",
        "r1 = 0",
        "*(u64*)(r10 - 128) = r2",
        "r2 = *(u64*)(r10 - 128)",
        "r2 &= 64",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "exit",
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

// SEC("tc")
// __description("helper access to variable memory: size = 0 allowed on != NULL stack pointer (ARG_PTR_TO_MEM_OR_NULL)")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_3() {
    asm!(
        "r1 = r10",
        "r1 += -8",
        "r2 = 0",
        "*(u64*)(r1 + 0) = r2",
        "r2 &= 8",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "exit",
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

// SEC("tc")
// __description("helper access to variable memory: size = 0 allowed on != NULL map pointer (ARG_PTR_TO_MEM_OR_NULL)")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_4() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = r0",
        "r2 = 0",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "exit",
        bpf_csum_diff = sym bpf_csum_diff,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("tc")
// __description("helper access to variable memory: size possible = 0 allowed on != NULL stack pointer (ARG_PTR_TO_MEM_OR_NULL)")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_5() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r2 = *(u64*)(r0 + 0)",
        "if r2 > 8 goto 0f",
        "r1 = r10",
        "r1 += -8",
        "*(u64*)(r1 + 0) = r2",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "exit",
        bpf_csum_diff = sym bpf_csum_diff,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("tc")
// __description("helper access to variable memory: size possible = 0 allowed on != NULL map pointer (ARG_PTR_TO_MEM_OR_NULL)")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_6() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = r0",
        "r2 = *(u64*)(r0 + 0)",
        "if r2 > 8 goto 0f",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "exit",
        bpf_csum_diff = sym bpf_csum_diff,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("tc")
// __description("helper access to variable memory: size possible = 0 allowed on != NULL packet pointer (ARG_PTR_TO_MEM_OR_NULL)")
// __success __retval(0)
/* csum_diff of 64-byte packet */
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_7() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r3 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r0 = r6",
        "r0 += 8",
        "if r0 > r3 goto 0f",
        "r1 = r6",
        "r2 = *(u64*)(r6 + 0)",
        "if r2 > 8 goto 0f",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "exit",
        bpf_csum_diff = sym bpf_csum_diff,
        __sk_buff_data = const 0,
        __sk_buff_data_end = const 0,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: size = 0 not allowed on NULL (!ARG_PTR_TO_MEM_OR_NULL)")
// __failure __msg("R1 type=scalar expected=fp")
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_8() {
    asm!(
        "r1 = 0",
        "r2 = 0",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: size > 0 not allowed on NULL (!ARG_PTR_TO_MEM_OR_NULL)")
// __failure __msg("R1 type=scalar expected=fp")
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_9() {
    asm!(
        "r1 = 0",
        "r2 = 1",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: size = 0 allowed on != NULL stack pointer (!ARG_PTR_TO_MEM_OR_NULL)")
// __success
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_10() {
    asm!(
        "r1 = r10",
        "r1 += -8",
        "r2 = 0",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: size = 0 allowed on != NULL map pointer (!ARG_PTR_TO_MEM_OR_NULL)")
// __success
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_11() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = r0",
        "r2 = 0",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "0:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: size possible = 0 allowed on != NULL stack pointer (!ARG_PTR_TO_MEM_OR_NULL)")
// __success
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_12() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r2 = *(u64*)(r0 + 0)",
        "if r2 > 8 goto 0f",
        "r1 = r10",
        "r1 += -8",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "0:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: size possible = 0 allowed on != NULL map pointer (!ARG_PTR_TO_MEM_OR_NULL)")
// __success
#[no_mangle]
pub unsafe extern "C" fn ptr_to_mem_or_null_13() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = r0",
        "r2 = *(u64*)(r0 + 0)",
        "if r2 > 8 goto 0f",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "0:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("socket")
// __description("helper access to variable memory: 8 bytes leak")
/* in privileged mode reads from uninitialized stack locations are permitted */
// __success __failure_unpriv
// __msg_unpriv("invalid read from stack R2 off -64+32 size 64")
// __retval(0)
#[no_mangle]
pub unsafe extern "C" fn variable_memory_8_bytes_leak() {
    asm!(
        "/* set max stack size */",
        "r6 = 0",
        "*(u64*)(r10 - 128) = r6",
        "/* set r3 to a random value */",
        "call {bpf_get_prandom_u32}",
        "r3 = r0",
        "r1 = {map_ringbuf} ll",
        "r2 = r10",
        "r2 += -64",
        "r0 = 0",
        "*(u64*)(r10 - 64) = r0",
        "*(u64*)(r10 - 56) = r0",
        "*(u64*)(r10 - 48) = r0",
        "*(u64*)(r10 - 40) = r0",
        "/* Note: fp[-32] left uninitialized */",
        "*(u64*)(r10 - 24) = r0",
        "*(u64*)(r10 - 16) = r0",
        "*(u64*)(r10 - 8) = r0",
        "/* Limit r3 range to [1, 64] */",
        "r3 &= 63",
        "r3 += 1",
        "r4 = 0",
        "/* Call bpf_ringbuf_output(), it is one of a few helper functions with",
        " * ARG_MEM_SIZE_OR_ZERO parameter allowed in unpriv mode.",
        " * For unpriv this should signal an error, because memory region [1, 64]",
        " * at &fp[-64] is not fully initialized.",
        " */",
        "call {bpf_ringbuf_output}",
        "r0 = 0",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        bpf_ringbuf_output = sym bpf_ringbuf_output,
        map_ringbuf = sym map_ringbuf,
        options(noreturn)
    );
}

// SEC("tracepoint")
// __description("helper access to variable memory: 8 bytes no leak (init memory)")
// __success
#[no_mangle]
pub unsafe extern "C" fn bytes_no_leak_init_memory() {
    asm!(
        "r1 = r10",
        "r0 = 0",
        "r0 = 0",
        "*(u64*)(r10 - 64) = r0",
        "*(u64*)(r10 - 56) = r0",
        "*(u64*)(r10 - 48) = r0",
        "*(u64*)(r10 - 40) = r0",
        "*(u64*)(r10 - 32) = r0",
        "*(u64*)(r10 - 24) = r0",
        "*(u64*)(r10 - 16) = r0",
        "*(u64*)(r10 - 8) = r0",
        "r1 += -64",
        "r2 = 0",
        "r2 &= 32",
        "r2 += 32",
        "r3 = 0",
        "call {bpf_probe_read_kernel}",
        "r1 = *(u64*)(r10 - 16)",
        "exit",
        bpf_probe_read_kernel = sym bpf_probe_read_kernel,
        options(noreturn)
    );
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
