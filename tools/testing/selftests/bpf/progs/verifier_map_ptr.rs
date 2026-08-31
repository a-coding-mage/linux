// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/map_ptr.c */

// C dependencies removed from executable Rust:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: u32,
    pub foo: [i32; MAX_ENTRIES],
}

// SEC(".maps")
// __uint(type, BPF_MAP_TYPE_ARRAY);
// __uint(max_entries, 1);
// __type(key, int);
// __type(value, struct test_val);
#[repr(C)]
pub struct map_array_48b_def {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut map_array_48b: map_array_48b_def = map_array_48b_def { _private: [] };

#[repr(C)]
pub struct other_val {
    pub foo: i64,
    pub bar: i64,
}

// SEC(".maps")
// __uint(type, BPF_MAP_TYPE_HASH);
// __uint(max_entries, 1);
// __type(key, long long);
// __type(value, struct other_val);
#[repr(C)]
pub struct map_hash_16b_def {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut map_hash_16b: map_hash_16b_def = map_hash_16b_def { _private: [] };

extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

// SEC("socket")
// __description("bpf_map_ptr: read with negative offset rejected")
// __failure
// __msg("R1 is bpf_array invalid negative access: off=-8")
// __failure_unpriv
// __msg_unpriv("access is allowed only to CAP_PERFMON and CAP_SYS_ADMIN")
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn read_with_negative_offset_rejected() {
    core::arch::asm!(
        "r1 = r10",
        "r1 = {map_array_48b} ll",
        "r6 = *(u64*)(r1 - 8)",
        "r0 = 1",
        "exit",
        map_array_48b = sym map_array_48b,
        options(noreturn)
    );
}

// SEC("socket")
// __description("bpf_map_ptr: write rejected")
// __failure
// __msg("only read from bpf_array is supported")
// __failure_unpriv
// __msg_unpriv("access is allowed only to CAP_PERFMON and CAP_SYS_ADMIN")
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn bpf_map_ptr_write_rejected() {
    core::arch::asm!(
        "r0 = 0",
        "*(u64*)(r10 - 8) = r0",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_array_48b} ll",
        "*(u64*)(r1 + 0) = r2",
        "r0 = 1",
        "exit",
        map_array_48b = sym map_array_48b,
        options(noreturn)
    );
}

/*
 * struct bpf_map starts with the SHA256 hash sha[32] at offset 0 (a readable
 * byte array), followed by the ops pointer at offset 32 and the inner_map_meta
 * pointer at offset 40. Reading a u32 at offset 41 reaches into the middle of
 * the inner_map_meta pointer, i.e. a partial pointer access, which is
 * rejected.
 */
// SEC("socket")
// __description("bpf_map_ptr: read non-existent field rejected")
// __failure
// __msg("cannot access ptr member inner_map_meta with moff 40 in struct bpf_map with off 41 size 4")
// __failure_unpriv
// __msg_unpriv("access is allowed only to CAP_PERFMON and CAP_SYS_ADMIN")
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn read_non_existent_field_rejected() {
    core::arch::asm!(
        "r6 = 0",
        "r1 = {map_array_48b} ll",
        "r6 = *(u32*)(r1 + 41)",
        "r0 = 1",
        "exit",
        map_array_48b = sym map_array_48b,
        options(noreturn)
    );
}

/*
 * The sha byte array spans offsets 0..31 (mend 32). Reading a u32 at offset
 * 30 starts inside sha but extends past its end, which the verifier rejects
 * as an out-of-bounds scalar access.
 */
// SEC("socket")
// __description("bpf_map_ptr: read beyond sha field rejected")
// __failure
// __msg("access beyond the end of member sha (mend:32) in struct bpf_map with off 30 size 4")
// __failure_unpriv
// __msg_unpriv("access is allowed only to CAP_PERFMON and CAP_SYS_ADMIN")
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn read_beyond_sha_field_rejected() {
    core::arch::asm!(
        "r6 = 0",
        "r1 = {map_array_48b} ll",
        "r6 = *(u32*)(r1 + 30)",
        "r0 = 1",
        "exit",
        map_array_48b = sym map_array_48b,
        options(noreturn)
    );
}

// SEC("socket")
// __description("bpf_map_ptr: read ops field accepted")
// __success
// __failure_unpriv
// __msg_unpriv("access is allowed only to CAP_PERFMON and CAP_SYS_ADMIN")
// __retval(1)
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn ptr_read_ops_field_accepted() {
    core::arch::asm!(
        "r6 = 0",
        "r1 = {map_array_48b} ll",
        "r6 = *(u64*)(r1 + 32)",
        "r0 = 1",
        "exit",
        map_array_48b = sym map_array_48b,
        options(noreturn)
    );
}

// SEC("socket")
// __description("bpf_map_ptr: r = 0, map_ptr = map_ptr + r")
// __success
// __failure_unpriv
// __msg_unpriv("R1 has pointer with unsupported alu operation")
// __retval(0)
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn map_ptr_map_ptr_r() {
    core::arch::asm!(
        "r0 = 0",
        "*(u64*)(r10 - 8) = r0",
        "r2 = r10",
        "r2 += -8",
        "r0 = 0",
        "r1 = {map_hash_16b} ll",
        "r1 += r0",
        "call {bpf_map_lookup_elem}",
        "r0 = 0",
        "exit",
        map_hash_16b = sym map_hash_16b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

// SEC("socket")
// __description("bpf_map_ptr: r = 0, r = r + map_ptr")
// __success
// __failure_unpriv
// __msg_unpriv("R0 has pointer with unsupported alu operation")
// __retval(0)
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn _0_r_r_map_ptr() {
    core::arch::asm!(
        "r0 = 0",
        "*(u64*)(r10 - 8) = r0",
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "r0 = {map_hash_16b} ll",
        "r1 += r0",
        "call {bpf_map_lookup_elem}",
        "r0 = 0",
        "exit",
        map_hash_16b = sym map_hash_16b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
