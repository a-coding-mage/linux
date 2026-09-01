// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// Original dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

// C condition:
// #if __has_attribute(btf_type_tag)
// #define __tag1 __attribute__((btf_type_tag("tag1")))
// #define __tag2 __attribute__((btf_type_tag("tag2")))
// volatile const bool skip_tests = false;
// #else
// #define __tag1
// #define __tag2
// volatile const bool skip_tests = true;
// #endif
//
// Rust has no direct file-local equivalent for Clang's btf_type_tag attribute.
#[no_mangle]
pub static skip_tests: bool = false;

#[repr(C)]
pub struct btf_type_tag_test {
    // C type: int __tag1 * __tag1 __tag2 *p;
    // The pointed-to int and intermediate pointer carry BTF type tags "tag1"
    // and "tag2" in the C source.
    pub p: *mut *mut i32,
}

#[no_mangle]
pub static mut g: btf_type_tag_test = btf_type_tag_test {
    p: core::ptr::null_mut(),
};

// SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn sub(x: i32) -> i32 {
    let _ = x;
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
