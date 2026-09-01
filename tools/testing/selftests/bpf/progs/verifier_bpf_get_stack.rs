// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/bpf_get_stack.c */

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

// Map definition originally declared with:
// SEC(".maps"), type BPF_MAP_TYPE_ARRAY, max_entries 1, key int,
// value struct test_val.
#[repr(C)]
pub struct map_array_48b_def {
    pub _private: [u8; 0],
}

unsafe extern "C" {
    pub static map_array_48b: map_array_48b_def;
}

// Map definition originally declared with:
// SEC(".maps"), type BPF_MAP_TYPE_HASH, max_entries 1, key long long,
// value struct test_val.
#[repr(C)]
pub struct map_hash_48b_def {
    pub _private: [u8; 0],
}

unsafe extern "C" {
    pub static map_hash_48b: map_hash_48b_def;
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_get_stack(
        ctx: *mut core::ffi::c_void,
        buf: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
    ) -> i64;
    fn bpf_get_task_stack(
        task: *mut core::ffi::c_void,
        buf: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
    ) -> i64;
    fn bpf_seq_write(
        seq: *mut core::ffi::c_void,
        data: *const core::ffi::c_void,
        len: u32,
    ) -> i64;
}

// SEC("tracepoint")
// __description("bpf_get_stack return R0 within range")
// __success
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_return_r0_within_range() {
    unsafe {
        asm!(
            "r6 = r1",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_48b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r7 = r0",
            "r9 = {test_val_half}",
            "r1 = r6",
            "r2 = r7",
            "r3 = {test_val_half}",
            "r4 = 256",
            "call {bpf_get_stack}",
            "r1 = 0",
            "r8 = r0",
            "r8 <<= 32",
            "r8 s>>= 32",
            "if r1 s> r8 goto 0f",
            "r9 -= r8",
            "r2 = r7",
            "r2 += r8",
            "r1 = r9",
            "r1 <<= 32",
            "r1 s>>= 32",
            "r3 = r2",
            "r3 += r1",
            "r1 = r7",
            "r5 = {test_val_half}",
            "r1 += r5",
            "if r3 >= r1 goto 0f",
            "r1 = r6",
            "r3 = r9",
            "r4 = 0",
            "call {bpf_get_stack}",
            "0:",
            "exit",
            map_hash_48b = sym map_hash_48b,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            bpf_get_stack = sym bpf_get_stack,
            test_val_half = const core::mem::size_of::<test_val>() / 2,
            options(noreturn)
        );
    }
}

// SEC("iter/task")
// __description("bpf_get_task_stack return R0 range is refined")
// __success
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn return_r0_range_is_refined() {
    unsafe {
        asm!(
            "r6 = *(u64*)(r1 + 0)",
            "r6 = *(u64*)(r6 + 0)",
            "r7 = *(u64*)(r1 + 8)",
            "r1 = {map_array_48b} ll",
            "r2 = 0",
            "*(u64*)(r10 - 8) = r2",
            "r2 = r10",
            "r2 += -8",
            "call {bpf_map_lookup_elem}",
            "if r0 != 0 goto 0f",
            "r0 = 0",
            "exit",
            "0:",
            "if r7 != 0 goto 1f",
            "r0 = 0",
            "exit",
            "1:",
            "r1 = r7",
            "r2 = r0",
            "r9 = r0",
            "r3 = 48",
            "r4 = 0",
            "call {bpf_get_task_stack}",
            "if r0 s> 0 goto 2f",
            "r0 = 0",
            "exit",
            "2:",
            "r1 = r6",
            "r2 = r9",
            "r3 = r0",
            "call {bpf_seq_write}",
            "r0 = 0",
            "exit",
            map_array_48b = sym map_array_48b,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            bpf_get_task_stack = sym bpf_get_task_stack,
            bpf_seq_write = sym bpf_seq_write,
            options(noreturn)
        );
    }
}

// SEC("license")
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
