// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/value.c */

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

pub const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: ::core::ffi::c_uint,
    pub foo: [::core::ffi::c_int; MAX_ENTRIES],
}

// Original C map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 1);
//     __type(key, long long);
//     __type(value, struct test_val);
// } map_hash_48b SEC(".maps");
//
// The __uint/__type/SEC BPF map-definition macros are provided externally.
#[repr(C)]
pub struct map_hash_48b_def {
    pub type_: ::core::ffi::c_uint,
    pub max_entries: ::core::ffi::c_uint,
    pub key: ::core::ffi::c_longlong,
    pub value: test_val,
}

unsafe extern "C" {
    #[link_name = "map_hash_48b"]
    pub static mut map_hash_48b: map_hash_48b_def;

    pub fn bpf_map_lookup_elem(
        map: *mut ::core::ffi::c_void,
        key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}

// SEC("socket")
// __description("map element value store of cleared call register")
// __failure __msg("R1 !read_ok")
// __failure_unpriv __msg_unpriv("R1 !read_ok")
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn store_of_cleared_call_register() {
    // Original inline eBPF assembly:
    // r2 = r10;
    // r2 += -8;
    // r1 = 0;
    // *(u64*)(r2 + 0) = r1;
    // r1 = %[map_hash_48b] ll;
    // call %[bpf_map_lookup_elem];
    // if r0 == 0 goto l0_%=;
    // *(u64*)(r0 + 0) = r1;
    // l0_%=: exit;
    unsafe {
        ::core::arch::asm!(
            "r2 = r10",
            "r2 += -8",
            "r1 = 0",
            "*(u64*)(r2 + 0) = r1",
            "r1 = {map_hash_48b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "*(u64*)(r0 + 0) = r1",
            "0:",
            "exit",
            map_hash_48b = sym map_hash_48b,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("map element value with unaligned store")
// __success __failure_unpriv __msg_unpriv("R0 leaks addr")
// __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn element_value_with_unaligned_store() {
    // Original inline eBPF assembly preserved as Rust asm.
    unsafe {
        ::core::arch::asm!(
            "r2 = r10",
            "r2 += -8",
            "r1 = 0",
            "*(u64*)(r2 + 0) = r1",
            "r1 = {map_hash_48b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r0 += 3",
            "r1 = 42",
            "*(u64*)(r0 + 0) = r1",
            "r1 = 43",
            "*(u64*)(r0 + 2) = r1",
            "r1 = 44",
            "*(u64*)(r0 - 2) = r1",
            "r8 = r0",
            "r1 = 32",
            "*(u64*)(r8 + 0) = r1",
            "r1 = 33",
            "*(u64*)(r8 + 2) = r1",
            "r1 = 34",
            "*(u64*)(r8 - 2) = r1",
            "r8 += 5",
            "r1 = 22",
            "*(u64*)(r8 + 0) = r1",
            "r1 = 23",
            "*(u64*)(r8 + 4) = r1",
            "r1 = 24",
            "*(u64*)(r8 - 7) = r1",
            "r7 = r8",
            "r7 += 3",
            "r1 = 22",
            "*(u64*)(r7 + 0) = r1",
            "r1 = 23",
            "*(u64*)(r7 + 4) = r1",
            "r1 = 24",
            "*(u64*)(r7 - 4) = r1",
            "0:",
            "exit",
            map_hash_48b = sym map_hash_48b,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("map element value with unaligned load")
// __success __failure_unpriv __msg_unpriv("R0 leaks addr")
// __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn element_value_with_unaligned_load() {
    // Original inline eBPF assembly preserved as Rust asm.
    unsafe {
        ::core::arch::asm!(
            "r2 = r10",
            "r2 += -8",
            "r1 = 0",
            "*(u64*)(r2 + 0) = r1",
            "r1 = {map_hash_48b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u32*)(r0 + 0)",
            "if r1 >= {max_entries} goto 0f",
            "r0 += 3",
            "r7 = *(u64*)(r0 + 0)",
            "r7 = *(u64*)(r0 + 2)",
            "r8 = r0",
            "r7 = *(u64*)(r8 + 0)",
            "r7 = *(u64*)(r8 + 2)",
            "r0 += 5",
            "r7 = *(u64*)(r0 + 0)",
            "r7 = *(u64*)(r0 + 4)",
            "0:",
            "exit",
            map_hash_48b = sym map_hash_48b,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            max_entries = const MAX_ENTRIES,
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("map element value is preserved across register spilling")
// __success __failure_unpriv __msg_unpriv("R0 leaks addr")
// __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_preserved_across_register_spilling() {
    // Original inline eBPF assembly preserved as Rust asm.
    unsafe {
        ::core::arch::asm!(
            "r2 = r10",
            "r2 += -8",
            "r1 = 0",
            "*(u64*)(r2 + 0) = r1",
            "r1 = {map_hash_48b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r0 += {test_val_foo}",
            "r1 = 42",
            "*(u64*)(r0 + 0) = r1",
            "r1 = r10",
            "r1 += -184",
            "*(u64*)(r1 + 0) = r0",
            "r3 = *(u64*)(r1 + 0)",
            "r1 = 42",
            "*(u64*)(r3 + 0) = r1",
            "0:",
            "exit",
            map_hash_48b = sym map_hash_48b,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            test_val_foo = const ::core::mem::offset_of!(test_val, foo),
            options(noreturn)
        );
    }
}

// char _license[] SEC("license") = "GPL";
#[unsafe(no_mangle)]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
