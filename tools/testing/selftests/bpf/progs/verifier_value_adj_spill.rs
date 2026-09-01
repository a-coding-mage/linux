// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/value_adj_spill.c */

// C includes translated as dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: ::core::ffi::c_uint,
    pub foo: [::core::ffi::c_int; MAX_ENTRIES],
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut ::core::ffi::c_void, key: *const ::core::ffi::c_void)
        -> *mut ::core::ffi::c_void;
}

#[repr(C)]
pub struct map_hash_48b_def {
    // __uint(type, BPF_MAP_TYPE_HASH);
    pub type_: *mut ::core::ffi::c_void,
    // __uint(max_entries, 1);
    pub max_entries: *mut [::core::ffi::c_int; 1],
    // __type(key, long long);
    pub key: *mut ::core::ffi::c_longlong,
    // __type(value, struct test_val);
    pub value: *mut test_val,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut map_hash_48b: map_hash_48b_def = map_hash_48b_def {
    type_: ::core::ptr::null_mut(),
    max_entries: ::core::ptr::null_mut(),
    key: ::core::ptr::null_mut(),
    value: ::core::ptr::null_mut(),
};

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
// __description("map element value is preserved across register spilling")
// __success __failure_unpriv __msg_unpriv("R0 leaks addr")
// __retval(0)
// __naked
pub unsafe extern "C" fn is_preserved_across_register_spilling() {
    unsafe {
        ::core::arch::asm!(
            "r2 = r10",
            "r2 += -8",
            "r1 = 0",
            "*(u64*)(r2 + 0) = r1",
            "r1 = {map_hash_48b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
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
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
// __description("map element value or null is marked on register spilling")
// __success __failure_unpriv __msg_unpriv("R0 leaks addr")
// __retval(0)
// __naked
pub unsafe extern "C" fn is_marked_on_register_spilling() {
    unsafe {
        ::core::arch::asm!(
            "r2 = r10",
            "r2 += -8",
            "r1 = 0",
            "*(u64*)(r2 + 0) = r1",
            "r1 = {map_hash_48b} ll",
            "call {bpf_map_lookup_elem}",
            "r1 = r10",
            "r1 += -152",
            "*(u64*)(r1 + 0) = r0",
            "if r0 == 0 goto 0f",
            "r3 = *(u64*)(r1 + 0)",
            "r1 = 42",
            "*(u64*)(r3 + 0) = r1",
            "0:",
            "exit",
            map_hash_48b = sym map_hash_48b,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
