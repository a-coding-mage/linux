// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

// C condition preserved:
// #if __has_attribute(btf_decl_tag)
// #define __tag1 __attribute__((btf_decl_tag("tag1")))
// #define __tag2 __attribute__((btf_decl_tag("tag2")))
// #else
// #define __tag1
// #define __tag2
// #endif

pub const BPF_MAP_TYPE_HASH: u32 = 1;

#[cfg(any())]
#[no_mangle]
pub static mut skip_tests: bool = false;

// Fallback for compilers without btf_decl_tag support in the original C source.
#[cfg(not(any()))]
#[no_mangle]
pub static mut skip_tests: bool = true;

#[repr(C)]
pub struct key_t {
    pub a: i32,
    // Original field had btf_decl_tag("tag1") and btf_decl_tag("tag2").
    pub b: i32,
    pub c: i32,
}

// Original struct type had btf_decl_tag("tag1") and btf_decl_tag("tag2").

#[repr(C)]
#[derive(Copy, Clone)]
pub struct value_t {
    pub a: i32,
    pub b: i32,
}

// Original typedef had btf_decl_tag("tag1") and btf_decl_tag("tag2").

#[repr(C)]
pub struct hashmap1_t {
    // Original C map definition used:
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __uint(max_entries, 3);
    // __type(key, struct key_t);
    // __type(value, value_t);
    pub type_: u32,
    pub max_entries: u32,
    pub key: *mut key_t,
    pub value: *mut value_t,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut hashmap1: hashmap1_t = hashmap1_t {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 3,
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
};

extern "C" {
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

// Original C function was static __noinline and had btf_decl_tag("tag1") and
// btf_decl_tag("tag2"); parameter x had both tags as well.
#[inline(never)]
unsafe fn foo(x: i32) -> i32 {
    let mut key: key_t = core::mem::zeroed();
    let val: value_t = core::mem::zeroed();

    key.c = x;
    key.b = key.c;
    key.a = key.b;
    bpf_map_update_elem(
        core::ptr::addr_of_mut!(hashmap1) as *mut core::ffi::c_void,
        core::ptr::addr_of!(key) as *const core::ffi::c_void,
        core::ptr::addr_of!(val) as *const core::ffi::c_void,
        0,
    );
    0
}

// Original section: SEC("fentry/bpf_fentry_test1")
// Original declaration: int BPF_PROG(sub, int x)
#[link_section = "fentry/bpf_fentry_test1"]
#[no_mangle]
pub unsafe extern "C" fn sub(x: i32) -> i32 {
    foo(x)
}
