// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/and.c */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;

// C includes removed:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: u32,
    pub foo: [i32; MAX_ENTRIES],
}

// Original BPF map declaration:
// struct {
//      __uint(type, BPF_MAP_TYPE_HASH);
//      __uint(max_entries, 1);
//      __type(key, long long);
//      __type(value, struct test_val);
// } map_hash_48b SEC(".maps");
#[repr(C)]
pub struct map_hash_48b_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i64,
    pub value: test_val,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut map_hash_48b: map_hash_48b_def = map_hash_48b_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key: 0,
    value: test_val {
        index: 0,
        foo: [0; MAX_ENTRIES],
    },
};

extern "C" {
    static BPF_MAP_TYPE_HASH: u32;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_get_prandom_u32() -> u32;
}

// SEC("socket")
// __description("invalid and of negative number")
// __failure __msg("R0 max value is outside of the allowed memory range")
// __failure_unpriv
// __flag(BPF_F_ANY_ALIGNMENT)
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn invalid_and_of_negative_number() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = *(u8*)(r0 + 0)",
        "r1 &= -4",
        "r1 <<= 2",
        "r0 += r1",
        "0:",
        "r1 = {test_val_foo}",
        "*(u64*)(r0 + 0) = r1",
        "exit",
        map_hash_48b = sym map_hash_48b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        test_val_foo = const core::mem::offset_of!(test_val, foo),
        options(noreturn)
    );
}

// SEC("socket")
// __description("invalid range check")
// __failure __msg("R0 max value is outside of the allowed memory range")
// __failure_unpriv
// __flag(BPF_F_ANY_ALIGNMENT)
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn invalid_range_check() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 0f",
        "r1 = *(u32*)(r0 + 0)",
        "r9 = 1",
        "w1 %= 2",
        "w1 += 1",
        "w9 &= w1",
        "w9 += 1",
        "w9 >>= 1",
        "w3 = 1",
        "w3 -= w9",
        "w3 *= 0x10000000",
        "r0 += r3",
        "*(u32*)(r0 + 0) = r3",
        "0:",
        "r0 = r0",
        "exit",
        map_hash_48b = sym map_hash_48b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

// SEC("socket")
// __description("check known subreg with unknown reg")
// __success __success_unpriv
// __retval(0)
// #ifdef SPEC_V1
// __xlated_unpriv("if w0 < 0x1 goto pc+2")
// __xlated_unpriv("nospec") /* inserted to prevent `R1 !read_ok'` */
// __xlated_unpriv("goto pc-1") /* `r1 = *(u32*)(r1 + 512)`, sanitized dead code */
// __xlated_unpriv("r0 = 0")
// #endif
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn known_subreg_with_unknown_reg() {
    asm!(
        "call {bpf_get_prandom_u32}",
        "r0 <<= 32",
        "r0 += 1",
        "r0 &= 0xFFFF1234",
        "/* Upper bits are unknown but AND above masks out 1 zero'ing lower bits */",
        "if w0 < 1 goto 0f",
        "r1 = *(u32*)(r1 + 512)",
        "0:",
        "r0 = 0",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
